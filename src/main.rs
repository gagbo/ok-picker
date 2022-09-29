// SPDX-FileCopyrightText: 2022 Gerry Agbobada <git@gagbo.net>
//
// SPDX-License-Identifier: GPL-3.0-only

use eframe::egui::{
    self, CentralPanel, Context, FontData, FontDefinitions, FontFamily, Hyperlink, ScrollArea,
    TopBottomPanel, Vec2,
};
use ok_picker::{colors, widgets};

fn main() {
    tracing_subscriber::fmt::init();

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
    colour: colors::Srgb,
    colour_too: colors::Srgb,
    palette: Vec<colors::Srgb>,
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

fn render_footer(ctx: &Context) {
    TopBottomPanel::bottom("Footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(5.);
            ui.add(Hyperlink::from_label_and_url(
                "Source",
                "https://github.com/gagbo/ok-picker",
            ));
            ui.add_space(5.);
        })
    });
}

fn render_header(ctx: &Context) {
    TopBottomPanel::top("header").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(5.);
            ui.label("Experiments for a color picker app in a better color space.");
            ui.add_space(5.);
        })
    });
}
impl eframe::App for OkPicker {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        render_footer(ctx);
        render_header(ctx);
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("RGB");
                    ui.spacing_mut().slider_width = 100.0;
                    egui::widgets::color_picker::color_picker_hsva_2d(
                        ui,
                        &mut self.color,
                        egui::color_picker::Alpha::Opaque,
                    );
                });

                ui.add_space(5.0);
                ui.separator();
                ui.add_space(5.0);

                ui.vertical_centered(|ui| {
                    ui.label("OkHSV");
                    ui.spacing_mut().slider_width = 100.0;
                    widgets::color_picker_okhsv_2d(ui, &mut self.colour);
                });

                ui.add_space(5.0);
                ui.separator();
                ui.add_space(5.0);

                ui.vertical_centered(|ui| {
                    ui.label("OkHSV circle");
                    ui.spacing_mut().slider_width = 100.0;
                    widgets::color_picker_okhsv_circle(ui, &mut self.colour_too);
                });
            });
        });
    }
}
