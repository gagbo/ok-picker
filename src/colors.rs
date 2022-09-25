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

impl From<Srgb> for OkHsv {
    fn from(gammad_rgb: Srgb) -> Self {
        let lab = OkLab::from(LinSrgb::from(gammad_rgb));

        let chroma = (lab.a.powi(2) + lab.b.powi(2)).sqrt();
        let a_ = lab.a / chroma;
        let b_ = lab.b / chroma;

        let hue = lab.b.atan2(lab.a);
        let cusp = find_cusp(a_, b_);
        let st_max = ST::from_cusp(cusp);

        const S0: f64 = 0.5;
        let k = 1.0 - S0 / st_max.s;

        // first we find L_v, C_v, L_vt and C_vt
        let t = st_max.t / (chroma + lab.lightness * st_max.t);
        let l_v = t * lab.lightness;
        let c_v = t * chroma;

        let l_vt = inverse_toe(l_v);
        let c_vt = c_v * l_vt / l_v;

        let rgb_scale = LinSrgb::from(OkLab {
            lightness: l_vt,
            a: a_ * c_vt,
            b: b_ * c_vt,
        });
        let scale_l = (1.0
            / f64::max(
                f64::max(rgb_scale.red, rgb_scale.green),
                f64::max(rgb_scale.blue, 0.0),
            ))
        .cbrt();
        let scaled_lightness = lab.lightness / scale_l;
        /* Code from the original source, that is unused here
         *
         * let scaled_chroma = chroma / scale_l;
         *
         * let scaled_chroma = scaled_chroma * toe(scaled_lightness) / scaled_lightness;
         */
        let l = toe(scaled_lightness);

        Self {
            hue,
            saturation: (S0 + st_max.t) * c_v / ((st_max.t * S0) + st_max.t * k * c_v),
            value: l / l_v,
        }
    }
}

impl From<OkHsv> for Srgb {
    fn from(hsv: OkHsv) -> Self {
        let a_ = (hsv.hue).cos();
        let b_ = (hsv.hue).sin();
        let cusp = find_cusp(a_, b_);
        let st_max = ST::from_cusp(cusp);

        const S0: f64 = 0.5;
        let k = 1.0 - S0 / st_max.s;

        // first we compute L and V as if the gamut is a perfect triangle:

        // L, C when v==1:
        let l_v = 1.0 - hsv.saturation * S0 / (S0 + st_max.t - st_max.t * k * hsv.saturation);
        let c_v = hsv.saturation * st_max.t * S0 / (S0 + st_max.t - st_max.t * k * hsv.saturation);

        let l = hsv.value * l_v;
        let c = hsv.value * c_v;

        // then we compensate for both toe and the curved top part of the triangle:
        let l_vt = inverse_toe(l_v);
        let c_vt = c_v * l_vt / l_v;

        let l_new = inverse_toe(l);
        let c = c * l_new / l;
        let l = l_new;

        let rgb_scale = LinSrgb::from(OkLab {
            lightness: l_vt,
            a: a_ * c_vt,
            b: b_ * c_vt,
        });
        let scale_l = (1.0
            / f64::max(
                f64::max(rgb_scale.red, rgb_scale.green),
                f64::max(rgb_scale.blue, 0.0),
            ))
        .cbrt();
        let l = l * scale_l;
        let c = c * scale_l;

        LinSrgb::from(OkLab {
            lightness: l,
            a: c * a_,
            b: c * b_,
        })
        .into()
    }
}

#[derive(Clone, Copy, Debug)]
struct LC {
    pub lightness: f64,
    pub chroma: f64,
}

/// Alternative representation of (L_cusp, C_cusp)
///
/// Encoded so S = C_cusp/L_cusp and T = C_cusp/(1-L_cusp)
/// The maximum value for C in the triangle is then found as
/// fmin(S*L, T*(1-L)), for a given L
#[derive(Clone, Copy, Debug)]
struct ST {
    pub s: f64,
    pub t: f64,
}

impl ST {
    fn from_cusp(cusp: LC) -> Self {
        Self {
            s: cusp.chroma / cusp.lightness,
            t: cusp.chroma / (1.0 - cusp.lightness),
        }
    }
}

/// toe function for L_r
fn toe(val: f64) -> f64 {
    const K1: f64 = 0.206;
    const K2: f64 = 0.03;
    const K3: f64 = (K1 + 1.0) / (K2 + 1.0);
    0.5 * (K3 * val - K1 + ((K3 * val - K1) * (K3 * val - K1) + 4.0 * K2 * K3 * val).sqrt())
}

/// inverse toe function for L_r
fn inverse_toe(val: f64) -> f64 {
    const K1: f64 = 0.206;
    const K2: f64 = 0.03;
    const K3: f64 = (K1 + 1.0) / (K2 + 1.0);
    (val * val + K1 * val) / (K3 * (val + K2))
}

fn find_cusp(a: f64, b: f64) -> LC {
    let s_cusp = compute_max_saturation(a, b);

    let max_rgb = LinSrgb::from(OkLab {
        lightness: 1.0,
        a: s_cusp * a,
        b: s_cusp * b,
    });
    let lightness = (1.0 / f64::max(max_rgb.red, f64::max(max_rgb.green, max_rgb.blue))).cbrt();
    LC {
        lightness,
        chroma: lightness * s_cusp,
    }
}
/// Finds the maximum saturation possible for a given hue that fits in sRGB
///
/// Saturation here is defined as S = C/L
/// a and b must be normalized so a^2 + b^2 == 1
fn compute_max_saturation(a: f64, b: f64) -> f64 {
    // Max saturation will be when one of r, g or b goes below zero.

    // Select different coefficients depending on which component goes below zero first
    let (k0, k1, k2, k3, k4, wl, wm, ws) = if 1.0 < a.mul_add(-1.881_703_28, -0.809_364_93 * b) {
        // red component
        (
            1.190_862_77,
            1.765_767_28,
            0.596_626_41,
            0.755_151_97,
            0.567_712_45,
            4.076_741_662_1,
            -3.307_711_591_3,
            0.230_969_929_2,
        )
    } else if 1.0 < a.mul_add(1.814_441_04, -1.194_452_76 * b) {
        // green component
        (
            0.739_565_15,
            -0.459_544_04,
            0.082_854_27,
            0.125_410_70,
            0.145_032_04,
            -1.268_438_004_6,
            2.609_757_401_1,
            -0.341_319_396_5,
        )
    } else {
        // blue component
        (
            1.357_336_52,
            -0.009_157_99,
            -1.151_302_10,
            -0.505_596_06,
            0.006_921_67,
            -0.004_196_086_3,
            -0.703_418_614_7,
            1.707_614_701_0,
        )
    };

    // Approximate max saturation using a polynomial:
    let s = k0 + k1 * a + k2 * b + k3 * a * a + k4 * a * b;

    // Do one step Halley's method to get closer
    // this gives an error less than 10e6, except for some blue hues where the dS/dh is close to infinite
    // this should be sufficient for most applications, otherwise do two/three steps
    let (k_l, k_m, k_s) = (
        0.396_337_777_4 * a + 0.215_803_757_3 * b,
        -0.105_561_345_8 * a - 0.063_854_172_8 * b,
        -0.089_484_177_5 * a - 1.291_485_548_0 * b,
    );

    let (l_, m_, s_) = (1.0 + s * k_l, 1.0 + s * k_m, 1.0 + s * k_s);
    let (l, m, s) = (l_.powi(3), m_.powi(3), s_.powi(3));
    let (l_ds, m_ds, s_ds) = (
        3.0 * k_l * l_ * l_,
        3.0 * k_m * m_ * m_,
        3.0 * k_s * s_ * s_,
    );
    let (l_ds2, m_ds2, s_ds2) = (
        6.0 * k_l * k_l * l_,
        6.0 * k_m * k_m * m_,
        6.0 * k_s * k_s * s_,
    );
    let f = wl * l + wm * m + ws * s;
    let f1 = wl * l_ds + wm * m_ds + ws * s_ds;
    let f2 = wl * l_ds2 + wm * m_ds2 + ws * s_ds2;

    s - f * f1 / (f1 * f1 - 0.5 * f * f2)
}
