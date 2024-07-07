use anyhow::Context;
use egui::{Color32, Layout};

use crate::prelude::*;
use crate::tiles::{TilePos, TileStorage};

const DEBUG_RECT_SIZE: f32 = 10.;

fn egui_debug_walker_col(ty: WalkerType) -> egui::Color32 {
    match ty {
        WalkerType::Null => egui::Color32::from_rgb(210, 210, 210),
        WalkerType::Player => egui::Color32::from_rgb(100, 0, 0),
        WalkerType::Box => egui::Color32::BLUE,
    }
}

fn egui_debug_tile_col(ty: TileType) -> egui::Color32 {
    match ty {
        TileType::Void => egui::Color32::DARK_GRAY,
        TileType::Floor => egui::Color32::WHITE,
        TileType::Entrance => egui::Color32::GREEN,
        TileType::DroppedBox => egui::Color32::from_rgb(0, 0, 100),
    }
}

fn egui_debug_level_grid(
    canvas_rect: egui::Rect,
    ui: &mut egui::Ui,
    tile_st: &TileStorage,
    walker_q: &Query<(Entity, &TilePos, &WalkerType)>,
    tile_q: &Query<&TileType>,
) {
    let width = tile_st.width();
    let height = tile_st.height();
    let tile_at = |x: u32, y: u32| {
        let e = tile_st.get_tile_at_pos(x, y)
                        .ok_or_else(|| anyhow::anyhow!("Bad tile pos"))?;
        anyhow::Ok(*tile_q.get(e)?)
    };
    let walkers_at = |x: u32, y: u32| {
        walker_q.iter().filter_map(move |(e, pos, ty)| {
            (pos.0 == x && pos.1 == y).then_some((e, *ty))
        })
    };
    let tile_tooltip_contents = |x: u32, y: u32, ui: &mut egui::Ui| {
        ui.label(format!("({x}, {y})"));

        match tile_at(x, y) {
            Ok(ty) => ui.label(format!("tile ty: {ty:?}")),
            Err(e) => ui.colored_label(egui::Color32::RED,
                format!("Failed to get tile type ({e:?}")
            ),
        };

        walkers_at(x, y).for_each(|(e, walker)| {
            ui.label(format!("walker: {walker:?} ({e:?})"));
        });
    };
    let tile_tooltip = |x: u32, y: u32, ui: &mut egui::Ui| {
        egui::popup::show_tooltip(
            ui.ctx(),
            egui::Id::new(("map", x, y)),
            |ui| tile_tooltip_contents(x, y, ui),
        );
    };

    for y in 0..height {
        for x in 0..width {
            let pos =
                egui::pos2(x as f32, y as f32) * DEBUG_RECT_SIZE
                    + canvas_rect.min.to_vec2();
            let tile_rect = egui::Rect {
                min: pos,
                max: pos + egui::vec2(DEBUG_RECT_SIZE, DEBUG_RECT_SIZE),
            };

            let mut ui = ui.child_ui(tile_rect, Layout::default());
            ui.set_width(DEBUG_RECT_SIZE);
            ui.set_height(DEBUG_RECT_SIZE);

            let pointer_inside = ui.ui_contains_pointer();
            if pointer_inside {
                tile_tooltip(x, y, &mut ui);
            };

            let tile_base_col = match tile_at(x, y) {
                Ok(ty) => egui_debug_tile_col(ty),
                Err(_) => Color32::RED,
            };
            let tile_col = if pointer_inside {
                tile_base_col
            } else {
                tile_base_col.gamma_multiply(0.5)
            };

            let painter = ui.painter_at(canvas_rect);
            painter.rect(
                tile_rect,
                0.,
                tile_col,
                egui::Stroke {
                    width: 1.,
                    color: egui::Color32::BLACK,
                },
            );

            let decor_pos = tile_rect.center();
            walkers_at(x, y).for_each(|(_, ty)| {
                painter.circle(
                    decor_pos,
                    DEBUG_RECT_SIZE * 0.4,
                    egui::Color32::TRANSPARENT,
                    egui::Stroke {
                        width: 2.,
                        color: egui_debug_walker_col(ty)
                    },
                );
            })
        }
    }
}

fn egui_debug_level_ui(
    ui: &mut egui::Ui,
    tile_st_q: &Query<&TileStorage>,
    walker_q: &Query<(Entity, &TilePos, &WalkerType)>,
    tile_q: &Query<&TileType>,
) -> anyhow::Result<()> {
    let tile_st = tile_st_q.get_single()
        .context("Acquiring tile storage")?;
    let walker_count = walker_q.iter().count();
    let active_walker_count = walker_q.iter()
        .filter(|(_, _, ty)| !ty.is_null())
        .count();
    let width = tile_st.width();
    let height = tile_st.height();

    ui.label(format!("map size: {}x{}", width, height));
    ui.label(format!("walker count: {walker_count}"));
    ui.label(format!("active walker count: {active_walker_count}"));

    egui::Frame::group(&egui::Style::default())
        .show(ui, |ui| {
            ui.set_width(width as f32 * DEBUG_RECT_SIZE);
            ui.set_height(height as f32 * DEBUG_RECT_SIZE);

            let painter_rect = ui.min_rect();

            egui_debug_level_grid(
                painter_rect,
                ui,
                tile_st,
                walker_q,
                tile_q,
            );
        });

    Ok(())
}

pub fn egui_debug_level(
    mut contexts: EguiContexts,
    tile_st_q: Query<&TileStorage>,
    walker_q: Query<(Entity, &TilePos, &WalkerType)>,
    tile_q: Query<&TileType>,
) {
    egui::Window::new("Level debug").show(
        contexts.ctx_mut(),
        |ui| {
            let res = egui_debug_level_ui(
                ui,
                &tile_st_q,
                &walker_q,
                &tile_q
            );
            if let Err(e) = res {
                ui.label(format!("No level debug ({e})"));
            }
        }
    );
}