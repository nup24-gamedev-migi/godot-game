use anyhow::Context;
use egui::Layout;

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

fn egui_debug_level_grid(
    canvas_rect: egui::Rect,
    ui: &mut egui::Ui,
    width: u32,
    height: u32,
    walker_q: &Query<(&TilePos, &WalkerType)>,
) {
    let walker_at = |x: u32, y: u32| {
        walker_q.iter().find_map(|(pos, ty)| {
            (pos.0 == x && pos.1 == y).then_some(*ty)
        })
    };
    let tile_tooltip_contents = |x: u32, y: u32, ui: &mut egui::Ui| {
        ui.label("TODO: tile type");
        if let Some(walker) = walker_at(x, y) {
            ui.label(format!("walker: {walker:?}"));
        }
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

            if ui.ui_contains_pointer() {
                tile_tooltip(x, y, &mut ui);
            };

            let painter = ui.painter_at(canvas_rect);
            painter.rect(
                tile_rect,
                0.,
                egui::Color32::WHITE,
                egui::Stroke {
                    width: 1.,
                    color: egui::Color32::BLACK,
                },
            );

            let decor_pos = tile_rect.center();
            if let Some(ty) = walker_at(x, y) {
                painter.circle(
                    decor_pos,
                    DEBUG_RECT_SIZE * 0.4,
                    egui::Color32::TRANSPARENT,
                    egui::Stroke {
                        width: 2.,
                        color: egui_debug_walker_col(ty)
                    },
                );
            }
        }
    }
}

fn egui_debug_level_ui(
    ui: &mut egui::Ui,
    tile_st_q: &Query<&TileStorage>,
    walker_q: &Query<(&TilePos, &WalkerType)>,
) -> anyhow::Result<()> {
    let tile_st = tile_st_q.get_single()
        .context("Acquiring tile storage")?;
    let walker_count = walker_q.iter().count();
    let active_walker_count = walker_q.iter()
        .filter(|(_, ty)| !ty.is_null())
        .count();
    let width = tile_st.width();
    let height = tile_st.height();

    ui.label(format!("map size: {}x{}", width, height));
    ui.label(format!("walker count: {walker_count}"));
    ui.label(format!("active walker count: {active_walker_count}"));

    // let resp = egui::ScrollArea::both()
    //     .max_height(200.)
    //     .max_width(200.)
    //     .auto_shrink([false, false])
    //     .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
    //     .show(ui, |ui| {
    //     });
    // let mut frame = egui::Frame::canvas(&egui::Style::default()).begin(ui);
    // frame.content_ui.set_width(width as f32 * DEBUG_RECT_SIZE);
    // frame.content_ui.set_height(height as f32 * DEBUG_RECT_SIZE);
    // let painter = frame.content_ui.painter();
    // egui_debug_level_grid(&painter, width, height, walker_q);
    // frame.end(ui);
    egui::Frame::group(&egui::Style::default())
        .show(ui, |ui| {
            ui.set_width(width as f32 * DEBUG_RECT_SIZE);
            ui.set_height(height as f32 * DEBUG_RECT_SIZE);

            let painter_rect = ui.min_rect();

            egui_debug_level_grid(
                painter_rect,
                ui,
                width,
                height,
                walker_q
            );
        });

    Ok(())
}

pub fn egui_debug_level(
    mut contexts: EguiContexts,
    tile_st_q: Query<&TileStorage>,
    walker_q: Query<(&TilePos, &WalkerType)>,
) {
    egui::Window::new("Level debug").show(
        contexts.ctx_mut(),
        |ui| {
            if let Err(e) = egui_debug_level_ui(ui, &tile_st_q, &walker_q) {
                ui.label(format!("No level debug ({e})"));
            }
        }
    );
}