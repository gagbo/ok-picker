// SPDX-FileCopyrightText: 2022 Gerry Agbobada <git@gagbo.net>
//
// SPDX-License-Identifier: GPL-3.0-only

use eframe::egui;
use egui::{FontData, FontDefinitions, FontFamily};
use ok_picker::{colors, widgets};

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Ok Picker",
        native_options,
        Box::new(|cc| Box::new(OkPickerApp::new(cc))),
    );
}

#[derive(Default)]
struct OkPickerApp {
    color: egui::color::Hsva,
    colour: colors::Srgb,
}

impl OkPickerApp {
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

impl eframe::App for OkPickerApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // TODO: Resize the frame to the widgets it contains
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("OkPicker chose {:#?}", self.color.to_srgb()));
            // TODO: Make an OkHSV widget for egui
            egui::widgets::color_picker::color_picker_hsva_2d(
                ui,
                &mut self.color,
                egui::color_picker::Alpha::Opaque,
            );
        });
    }
}
