// SPDX-FileCopyrightText: 2022 Gerry Agbobada <git@gagbo.net>
//
// SPDX-License-Identifier: GPL-3.0-only

use eframe::egui::{Color32, Rgba};

pub mod conversions;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Srgb {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct LinSrgb {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct OkLab {
    pub lightness: f64,
    pub a: f64,
    pub b: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct OkLCh {
    pub lightness: f64,
    pub chroma: f64,
    pub hue: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct OkHsl {
    pub hue: f64,
    pub saturation: f64,
    pub lightness: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct OkHsv {
    pub hue: f64,
    pub saturation: f64,
    pub value: f64,
}

impl From<OkHsv> for Color32 {
    fn from(hsv: OkHsv) -> Self {
        let rgb = Srgb::from(hsv);
        Self::from_rgb(
            (rgb.red * 256.0).floor() as u8,
            (rgb.green * 256.0).floor() as u8,
            (rgb.blue * 256.0).floor() as u8,
        )
    }
}

impl From<OkHsv> for Rgba {
    fn from(hsv: OkHsv) -> Self {
        let rgb = Srgb::from(hsv);
        Self::from_rgb(rgb.red as f32, rgb.green as f32, rgb.blue as f32)
    }
}
