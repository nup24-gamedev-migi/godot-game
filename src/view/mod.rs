use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
use strum::{VariantArray, VariantNames};
use strum_macros::{VariantArray, VariantNames};

use crate::logic::Logic;

mod raw_view;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[derive(VariantNames, VariantArray)]
enum ViewMode {
    Raw = 0,
    SimpleGeometry = 1,
}

impl Into<usize> for ViewMode {
    fn into(self) -> usize {
        self as usize
    }
}

pub struct GameView {
    mode: ViewMode,
}

impl GameView {
    pub fn new() -> Self {
        Self {
            mode: ViewMode::Raw,
        }
    }

    pub fn draw_logic(&self, logic: &Logic) {
        match self.mode {
            ViewMode::Raw => raw_view::draw_logic(logic),
            ViewMode::SimpleGeometry => (),
        }
    }

    pub fn debug_ui(&mut self) {
        widgets::Window::new(hash!(), vec2(470., 50.), vec2(300., 300.))
        .label("View debug")
        .ui(&mut *root_ui(), |ui| {
            self.view_mode_picker(ui);
            ui.tree_node(hash!(), "general info", |ui| {
                ui.label(None, "TODO");
            });
        });
    }

    fn view_mode_picker(&mut self, ui: &mut macroquad::ui::Ui) {
        let mut id = self.mode as usize;
        ui.combo_box(hash!(), "view mode",
            <ViewMode as VariantNames>::VARIANTS,
            Some(&mut id)
        );
        self.mode = <ViewMode as VariantArray>::VARIANTS[id];
    }
}