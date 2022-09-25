// SPDX-FileCopyrightText: 2022 Gerry Agbobada <git@gagbo.net>
//
// SPDX-License-Identifier: GPL-3.0-only

pub mod conversions;

#[derive(Clone, Copy, Debug, Default)]
pub struct Srgb {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct LinSrgb {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct OkLab {
    pub lightness: f64,
    pub a: f64,
    pub b: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct OkLCh {
    pub lightness: f64,
    pub chroma: f64,
    pub hue: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct OkHsl {
    pub hue: f64,
    pub saturation: f64,
    pub lightness: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct OkHsv {
    /// Remapped to [0, 1] range
    pub hue: f64,
    pub saturation: f64,
    pub value: f64,
}
