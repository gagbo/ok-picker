// SPDX-FileCopyrightText: 2022 Björn Ottosson
// SPDX-FileCopyrightText: 2022 Gerry Agbobada <git@gagbo.net>
//
// SPDX-License-Identifier: MIT

use once_cell::sync::Lazy;

/// The matrices were updated 2021-01-25
static M1_LIN_SRGB_TO_OKLAB: Lazy<ndarray::Array2<f64>> = Lazy::new(|| {
    ndarray::arr2(&[
        [0.412_221_470_8, 0.536_332_536_3, 0.051_445_992_9],
        [0.211_903_498_2, 0.680_699_545_1, 0.107_396_956_6],
        [0.088_302_461_9, 0.281_718_837_6, 0.629_978_700_5],
    ])
});

/// The matrices were updated 2021-01-25
static M2_LIN_SRGB_TO_OKLAB: Lazy<ndarray::Array2<f64>> = Lazy::new(|| {
    ndarray::arr2(&[
        [0.210_454_255_3, 0.793_617_785_0, -0.004_072_046_8],
        [1.977_998_495_1, -2.428_592_205_0, 0.450_593_709_9],
        [0.025_904_037_1, 0.782_771_766_2, -0.808_675_766_0],
    ])
});

/// The matrices were updated 2021-01-25
static M1_OKLAB_TO_LIN_SRGB: Lazy<ndarray::Array2<f64>> = Lazy::new(|| {
    ndarray::arr2(&[
        [1.0, 0.396_337_777_4, 0.215_803_757_3],
        [1.0, -0.105_561_345_8, -0.063_854_172_8],
        [1.0, -0.089_484_177_5, -1.291_485_548_0],
    ])
});

/// The matrices were updated 2021-01-25
static M2_OKLAB_TO_LIN_SRGB: Lazy<ndarray::Array2<f64>> = Lazy::new(|| {
    ndarray::arr2(&[
        [4.076_741_662_1, -3.307_711_591_3, 0.230_969_929_2],
        [-1.268_438_004_6, 2.609_757_401_1, -0.341_319_396_5],
        [-0.004_196_086_3, -0.703_418_614_7, 1.707_614_701_0],
    ])
});

#[derive(Clone, Copy, Debug)]
pub struct Srgb {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl From<LinSrgb> for Srgb {
    fn from(linear: LinSrgb) -> Self {
        fn transform(val: f64) -> f64 {
            if val >= 0.003_130_8 {
                val.powf(1.0 / 2.4).mul_add(1.055, -0.055)
            } else {
                12.92 * val
            }
        }

        Self {
            red: transform(linear.red),
            green: transform(linear.green),
            blue: transform(linear.blue),
        }
    }
}

impl From<Srgb> for LinSrgb {
    fn from(gammad: Srgb) -> Self {
        fn inverse_transform(val: f64) -> f64 {
            if val >= 0.040_45 {
                ((val + 0.055) / 1.055).powf(2.4)
            } else {
                val / 12.92
            }
        }

        Self {
            red: inverse_transform(gammad.red),
            green: inverse_transform(gammad.green),
            blue: inverse_transform(gammad.blue),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct LinSrgb {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl From<&ndarray::Array1<f64>> for LinSrgb {
    fn from(col: &ndarray::Array1<f64>) -> Self {
        let slice = col.as_slice().unwrap();
        debug_assert_eq!(slice.len(), 3);
        Self {
            red: slice[0],
            green: slice[1],
            blue: slice[2],
        }
    }
}

impl From<LinSrgb> for ndarray::Array1<f64> {
    fn from(col: LinSrgb) -> Self {
        ndarray::arr1(&[col.red, col.green, col.blue])
    }
}

#[derive(Clone, Copy, Debug)]
pub struct OkLab {
    pub lightness: f64,
    pub a: f64,
    pub b: f64,
}

impl From<&ndarray::Array1<f64>> for OkLab {
    fn from(col: &ndarray::Array1<f64>) -> Self {
        let slice = col.as_slice().unwrap();
        debug_assert_eq!(slice.len(), 3);
        Self {
            lightness: slice[0],
            a: slice[1],
            b: slice[2],
        }
    }
}

impl From<OkLab> for ndarray::Array1<f64> {
    fn from(col: OkLab) -> Self {
        ndarray::arr1(&[col.lightness, col.a, col.b])
    }
}

#[derive(Clone, Copy, Debug)]
pub struct OkLCh {
    pub lightness: f64,
    pub chroma: f64,
    pub hue: f64,
}

impl From<OkLab> for OkLCh {
    fn from(lab: OkLab) -> Self {
        Self {
            lightness: lab.lightness,
            chroma: (lab.a.powi(2) + lab.b.powi(2)).sqrt(),
            hue: lab.b.atan2(lab.a),
        }
    }
}

impl From<OkLCh> for OkLab {
    fn from(lch: OkLCh) -> Self {
        Self {
            lightness: lch.lightness,
            a: lch.chroma * lch.hue.cos(),
            b: lch.chroma * lch.hue.sin(),
        }
    }
}

impl From<LinSrgb> for OkLab {
    fn from(lin: LinSrgb) -> Self {
        use ndarray::Array1;
        let lin_vec = Array1::<f64>::from(lin);
        let lms = M1_LIN_SRGB_TO_OKLAB.dot(&lin_vec).to_vec();
        let lms_ = ndarray::arr1(&[lms[0].cbrt(), lms[1].cbrt(), lms[2].cbrt()]);
        (&M2_LIN_SRGB_TO_OKLAB.dot(&lms_)).into()
    }
}

impl From<OkLab> for LinSrgb {
    fn from(lab: OkLab) -> Self {
        use ndarray::Array1;
        let lab_vec = Array1::<f64>::from(lab);
        let lms_ = M1_OKLAB_TO_LIN_SRGB.dot(&lab_vec).to_vec();
        let lms = ndarray::arr1(&[lms_[0].powi(3), lms_[1].powi(3), lms_[2].powi(3)]);
        (&M2_OKLAB_TO_LIN_SRGB.dot(&lms)).into()
    }
}

// TODO: https://bottosson.github.io/posts/colorpicker/#common-code
