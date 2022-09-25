// SPDX-FileCopyrightText: 2022 Gerry Agbobada <git@gagbo.net>
//
// SPDX-License-Identifier: GPL-3.0-only

use std::f64::consts::PI;

use eframe::{
    egui::{
        color_picker::show_color, lerp, pos2, remap_clamp, vec2, Color32, Mesh, Painter, Rect,
        Response, Rgba, Sense, Shape, Stroke, Ui, Vec2,
    },
    epaint,
};

use crate::colors::{OkHsv, Srgb};

/// Number of vertices per dimension in the color sliders.
/// We need at least 6 for hues, and more for smooth 2D areas.
/// Should always be a multiple of 6 to hit the peak hues in HSV/HSL (every 60°).
const N: u32 = 6 * 6;

//// Shows a color picker where the user can change the given [`OkHsv`] color.
///
/// Returns `true` on change.
pub fn color_picker_okhsv_2d(ui: &mut Ui, okhsv: &mut OkHsv) -> bool {
    let mut new_okhsv = *okhsv;

    ui.vertical_centered(|ui| {
        color_picker_okhsv_2d_impl(ui, &mut new_okhsv);
    });

    if *okhsv == new_okhsv {
        false
    } else {
        *okhsv = new_okhsv;
        true
    }
}

fn color_picker_okhsv_2d_impl(ui: &mut Ui, okhsv: &mut OkHsv) {
    let current_color_size = vec2(
        2.0 * ui.spacing().slider_width,
        2.0 * ui.spacing().interact_size.y,
    );
    show_color(ui, *okhsv, current_color_size).on_hover_text("Selected color");

    color_text_ui(ui, *okhsv);

    let current = *okhsv;

    let OkHsv {
        hue,
        saturation,
        value,
    } = okhsv;

    color_slider_1d(ui, hue, -PI, PI, |hue| OkHsv {
        hue,
        saturation: 1.0,
        value: 1.0,
    })
    .on_hover_text("Hue fully saturated");
    color_slider_1d(ui, hue, -PI, PI, |hue| OkHsv { hue, ..current }).on_hover_text("Hue");

    color_slider_2d(ui, value, saturation, |value, saturation| OkHsv {
        saturation: saturation as f64,
        value: value as f64,
        ..current
    });

    if true {
        color_slider_1d(ui, saturation, 0.0, 1.0, |saturation| OkHsv {
            saturation,
            ..current
        })
        .on_hover_text("Saturation");
    }

    if true {
        color_slider_1d(ui, value, 0.0, 1.0, |value| OkHsv { value, ..current })
            .on_hover_text("Value");
    }
}

fn color_text_ui(ui: &mut Ui, color: impl Into<Srgb>) {
    let color = color.into();
    ui.vertical_centered(|ui| {
        ui.horizontal(|ui| {
            let Srgb { red, green, blue } = color;

            let r = (256.0 * red).floor() as u8;
            let g = (256.0 * green).floor() as u8;
            let b = (256.0 * blue).floor() as u8;

            if ui.button("📋").on_hover_text("Click to copy").clicked() {
                ui.output().copied_text = format!("{}, {}, {}", r, g, b);
            }

            ui.label(format!("rgb({}, {}, {})", r, g, b))
                .on_hover_text("Red Green Blue");
        });
        ui.horizontal(|ui| {
            let Srgb { red, green, blue } = color;

            let r = (256.0 * red).floor() as u8;
            let g = (256.0 * green).floor() as u8;
            let b = (256.0 * blue).floor() as u8;

            if ui.button("📋").on_hover_text("Click to copy").clicked() {
                ui.output().copied_text = format!("#{:02X}{:02X}{:02X}", r, g, b);
            }

            ui.label(format!("rgb(#{:02X}{:02X}{:02X})", r, g, b))
                .on_hover_text("Red Green Blue, Hex");
        });
        ui.horizontal(|ui| {
            let hsv = OkHsv::from(color);

            if ui.button("📋").on_hover_text("Click to copy").clicked() {
                ui.output().copied_text = format!("{}, {}, {}", hsv.hue, hsv.saturation, hsv.value);
            }

            // Approx 512 even steps for the rounding
            let trunc = 1.0 / 2.0_f64.powi(9);

            ui.label(format!(
                "okhsv({}, {}, {})",
                trunc * (hsv.hue / trunc).trunc(),
                trunc * (hsv.saturation / trunc).trunc(),
                trunc * (hsv.value / trunc).trunc()
            ))
            .on_hover_text("Hue Saturation Value, OkHSV");
        });
    });
}

fn color_slider_1d(
    ui: &mut Ui,
    value: &mut f64,
    min: f64,
    max: f64,
    color_at: impl Fn(f64) -> OkHsv,
) -> Response {
    #![allow(clippy::identity_op)]

    let span = max - min;

    let desired_size = vec2(
        2.0 * ui.spacing().slider_width,
        ui.spacing().interact_size.y,
    );
    let (rect, response) = ui.allocate_at_least(desired_size, Sense::click_and_drag());

    if let Some(mpos) = response.interact_pointer_pos() {
        *value = min + span * remap_clamp(mpos.x, rect.left()..=rect.right(), 0.0..=1.0) as f64;
    }

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact(&response);

        background_checkers(ui.painter(), rect); // for alpha:

        {
            // fill color:
            let mut mesh = Mesh::default();
            for i in 0..=N {
                let t = min + (i as f64 * span / (N as f64));
                let color = color_at(t);
                let x = lerp(
                    rect.left()..=rect.right(),
                    (t as f32 - min as f32) / span as f32,
                );
                mesh.colored_vertex(pos2(x, rect.top()), color.into());
                mesh.colored_vertex(pos2(x, rect.bottom()), color.into());
                if i < N {
                    mesh.add_triangle(2 * i + 0, 2 * i + 1, 2 * i + 2);
                    mesh.add_triangle(2 * i + 1, 2 * i + 2, 2 * i + 3);
                }
            }
            ui.painter().add(Shape::mesh(mesh));
        }

        ui.painter().rect_stroke(rect, 0.0, visuals.bg_stroke); // outline

        {
            // Show where the slider is at:
            let x = lerp(
                rect.left()..=rect.right(),
                (*value as f32 - min as f32) / span as f32,
            );
            let r = rect.height() / 4.0;
            let picked_color = color_at(*value);
            ui.painter().add(Shape::convex_polygon(
                vec![
                    pos2(x, rect.center().y),   // tip
                    pos2(x + r, rect.bottom()), // right bottom
                    pos2(x - r, rect.bottom()), // left bottom
                ],
                picked_color,
                Stroke::new(visuals.fg_stroke.width, contrast_color(picked_color)),
            ));
        }
    }

    response
}

fn color_slider_2d(
    ui: &mut Ui,
    x_value: &mut f64,
    y_value: &mut f64,
    color_at: impl Fn(f64, f64) -> OkHsv,
) -> Response {
    let desired_size = Vec2::splat(2.0 * ui.spacing().slider_width);
    let (rect, response) = ui.allocate_at_least(desired_size, Sense::click_and_drag());

    if let Some(mpos) = response.interact_pointer_pos() {
        *x_value = remap_clamp(mpos.x, rect.left()..=rect.right(), 0.0..=1.0) as f64;
        *y_value = remap_clamp(mpos.y, rect.bottom()..=rect.top(), 0.0..=1.0) as f64;
    }

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact(&response);
        let mut mesh = Mesh::default();

        for xi in 0..=N {
            for yi in 0..=N {
                let xt = xi as f64 / (N as f64);
                let yt = yi as f64 / (N as f64);
                let color = color_at(xt, yt);
                let x = lerp(rect.left()..=rect.right(), xt as f32);
                let y = lerp(rect.bottom()..=rect.top(), yt as f32);
                mesh.colored_vertex(pos2(x, y), color.into());

                if xi < N && yi < N {
                    let x_offset = 1;
                    let y_offset = N + 1;
                    let tl = yi * y_offset + xi;
                    mesh.add_triangle(tl, tl + x_offset, tl + y_offset);
                    mesh.add_triangle(tl + x_offset, tl + y_offset, tl + y_offset + x_offset);
                }
            }
        }
        ui.painter().add(Shape::mesh(mesh)); // fill

        ui.painter().rect_stroke(rect, 0.0, visuals.bg_stroke); // outline

        // Show where the slider is at:
        let x = lerp(rect.left()..=rect.right(), *x_value as f32);
        let y = lerp(rect.bottom()..=rect.top(), *y_value as f32);
        let picked_color = color_at(*x_value, *y_value);
        ui.painter().add(epaint::CircleShape {
            center: pos2(x, y),
            radius: rect.width() / 12.0,
            fill: picked_color.into(),
            stroke: Stroke::new(visuals.fg_stroke.width, contrast_color(picked_color)),
        });
    }

    response
}

fn background_checkers(painter: &Painter, rect: Rect) {
    let rect = rect.shrink(0.5); // Small hack to avoid the checkers from peeking through the sides
    if !rect.is_positive() {
        return;
    }

    let dark_color = Color32::from_gray(32);
    let bright_color = Color32::from_gray(128);

    let checker_size = Vec2::splat(rect.height() / 2.0);
    let n = (rect.width() / checker_size.x).round() as u32;

    let mut mesh = Mesh::default();
    mesh.add_colored_rect(rect, dark_color);

    let mut top = true;
    for i in 0..n {
        let x = lerp(rect.left()..=rect.right(), i as f32 / (n as f32));
        let small_rect = if top {
            Rect::from_min_size(pos2(x, rect.top()), checker_size)
        } else {
            Rect::from_min_size(pos2(x, rect.center().y), checker_size)
        };
        mesh.add_colored_rect(small_rect, bright_color);
        top = !top;
    }
    painter.add(Shape::mesh(mesh));
}

fn contrast_color(color: impl Into<Rgba>) -> Color32 {
    if color.into().intensity() < 0.5 {
        Color32::WHITE
    } else {
        Color32::BLACK
    }
}
