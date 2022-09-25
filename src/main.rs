// SPDX-FileCopyrightText: 2022 Gerry Agbobada <git@gagbo.net>
//
// SPDX-License-Identifier: GPL-3.0-only

use egui::{self, FontData, FontDefinitions, FontFamily, Vec2};
use ok_picker::{colors, widgets};

fn main() {
    let win_options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(640.0, 480.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Ok Picker",
        win_options,
        Box::new(|cc| Box::new(OkPicker::new(cc))),
    );
}

#[derive(Default)]
struct OkPicker {
    color: egui::color::Hsva,
    colour: colors::OkHsv,
}

impl OkPicker {
    fn fonts() -> FontDefinitions {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "Iosevka Clapoto".to_owned(),
            FontData::from_static(include_bytes!("../fonts/iosevka-clapoto-regular.ttf")),
        );
        fonts.font_data.insert(
            "Asap".to_owned(),
            FontData::from_static(include_bytes!("../fonts/Asap-Regular.ttf")),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "Asap".to_owned());
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .insert(0, "Iosevka Clapoto".to_owned());
        fonts
    }

    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_fonts(Self::fonts());
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for OkPicker {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("OkPicker").show(ctx, |ui| {
            ui.label("Experiments for a color picker app in a better color space.");
        });
        egui::CentralPanel::default().show(ctx, |_ui| {});
        egui::Window::new("RGB").show(ctx, |ui| {
            ui.spacing_mut().slider_width = 100.0;
            egui::widgets::color_picker::color_picker_hsva_2d(
                ui,
                &mut self.color,
                egui::color_picker::Alpha::Opaque,
            );
        });
        egui::Window::new("Ok HSV").show(ctx, |ui| {
            ui.spacing_mut().slider_width = 100.0;
            widgets::color_picker_okhsv_2d(ui, &mut self.colour);
        });
    }
}
