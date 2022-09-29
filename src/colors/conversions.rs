// SPDX-FileCopyrightText: 2022 Björn Ottosson
// SPDX-FileCopyrightText: 2022 Gerry Agbobada <git@gagbo.net>
//
// SPDX-License-Identifier: MIT

//! Colorspace conversions
//!
//! Yes, it could be shaders. It could.

// TODO: fix NaN issues when converting very dark colors.

use super::{LinSrgb, OkHsl, OkHsv, OkLCh, OkLab, Srgb};

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

impl From<OkHsl> for Srgb {
    fn from(hsl: OkHsl) -> Self {
        if hsl.lightness == 1.0 {
            return Self {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
            };
        }

        if hsl.lightness == 0.0 {
            return Self {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
            };
        }

        let a = hsl.hue.cos();
        let b = hsl.hue.sin();
        let l = inverse_toe(hsl.lightness);

        let Cs { c_0, c_mid, c_max } = Cs::from(OkLab { lightness: l, a, b });

        let mid = 0.8;
        let mid_inv = 1.25_f64;

        let c = if hsl.saturation < mid {
            let t = mid_inv * hsl.saturation;
            let k_1 = mid * c_0;
            let k_2 = 1.0 - k_1 / c_mid;

            t * k_1 / (1.0 - k_2 * t)
        } else {
            let t = (hsl.saturation - mid) / (1.0 - mid);
            let k_0 = c_mid;
            let k_1 = (1.0 - mid) * c_mid.powi(2) * mid_inv.powi(2) / c_0;
            let k_2 = 1.0 - k_1 / (c_max - c_mid);

            k_0 + t * k_1 / (1.0 - k_2 * t)
        };

        Self::from(LinSrgb::from(OkLab {
            lightness: l,
            a: c * a,
            b: c * b,
        }))
    }
}

impl From<Srgb> for OkHsl {
    fn from(rgb: Srgb) -> Self {
        let lab = OkLab::from(LinSrgb::from(rgb));

        let c = (lab.a.powi(2) + lab.b.powi(2)).sqrt();
        let a_ = lab.a / c;
        let b_ = lab.b / c;
        let lightness = lab.lightness;
        let hue = lab.b.atan2(lab.a);

        let Cs { c_0, c_mid, c_max } = Cs::from(OkLab {
            lightness,
            a: a_,
            b: b_,
        });
        // Inverse of the interpolation in Srgb::from::<OkHsl>()
        let mid = 0.8;
        let mid_inv = 1.25_f64;

        let saturation = if c < c_mid {
            let k_1 = mid * c_0;
            let k_2 = 1.0 - k_1 / c_mid;

            mid * c / (k_1 + k_2 * c)
        } else {
            let k_0 = c_mid;
            let k_1 = (1.0 - mid) * c_mid.powi(2) * mid_inv.powi(2) / c_0;
            let k_2 = 1.0 - k_1 / (c_max - c_mid);

            mid + (1.0 - mid) * (c - k_0) / (k_1 + k_2 * (c - k_0))
        };

        Self {
            hue,
            saturation,
            lightness,
        }
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

    /// Returns a smooth approximation of the location of the cusp
    /// This polynomial was created by an optimization process
    /// It has been designed so that S_mid < S_max and T_mid < T_max
    fn mid(a: f64, b: f64) -> Self {
        Self {
            s: 0.115_169_93
                + 1.0
                    / (7.447_789_70
                        + 4.159_012_40 * b
                        + a * (-2.195_573_47
                            + 1.751_984_01 * b
                            + a * (-2.137_049_48 - 10.023_010_43 * b
                                + a * (-4.248_945_61 + 5.387_708_19 * b + 4.698_910_13 * a)))),

            t: 0.112_396_42
                + 1.0
                    / (1.613_203_20 - 0.681_243_79 * b
                        + a * (0.403_706_12
                            + 0.901_481_23 * b
                            + a * (-0.270_879_43
                                + 0.612_239_90 * b
                                + a * (0.002_992_15 - 0.453_995_68 * b - 0.146_618_72 * a)))),
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
            M2_OKLAB_TO_LIN_SRGB[[0, 0]],
            M2_OKLAB_TO_LIN_SRGB[[0, 1]],
            M2_OKLAB_TO_LIN_SRGB[[0, 2]],
        )
    } else if 1.0 < a.mul_add(1.814_441_04, -1.194_452_76 * b) {
        // green component
        (
            0.739_565_15,
            -0.459_544_04,
            0.082_854_27,
            0.125_410_70,
            0.145_032_04,
            M2_OKLAB_TO_LIN_SRGB[[1, 0]],
            M2_OKLAB_TO_LIN_SRGB[[1, 1]],
            M2_OKLAB_TO_LIN_SRGB[[1, 2]],
        )
    } else {
        // blue component
        (
            1.357_336_52,
            -0.009_157_99,
            -1.151_302_10,
            -0.505_596_06,
            0.006_921_67,
            M2_OKLAB_TO_LIN_SRGB[[2, 0]],
            M2_OKLAB_TO_LIN_SRGB[[2, 1]],
            M2_OKLAB_TO_LIN_SRGB[[2, 2]],
        )
    };

    // Approximate max saturation using a polynomial:
    let s = k0 + k1 * a + k2 * b + k3 * a * a + k4 * a * b;

    // Do one step Halley's method to get closer
    // this gives an error less than 10e6, except for some blue hues where the dS/dh is close to infinite
    // this should be sufficient for most applications, otherwise do two/three steps
    let (k_l, k_m, k_s) = (
        M1_OKLAB_TO_LIN_SRGB[[0, 1]] * a + M1_OKLAB_TO_LIN_SRGB[[0, 2]] * b,
        M1_OKLAB_TO_LIN_SRGB[[1, 1]] * a + M1_OKLAB_TO_LIN_SRGB[[1, 2]] * b,
        M1_OKLAB_TO_LIN_SRGB[[2, 1]] * a + M1_OKLAB_TO_LIN_SRGB[[2, 2]] * b,
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

#[derive(Clone, Copy, Debug)]
struct Cs {
    c_0: f64,
    c_mid: f64,
    c_max: f64,
}

impl From<OkLab> for Cs {
    fn from(lab: OkLab) -> Self {
        let cusp = find_cusp(lab.a, lab.b);
        let c_max = find_gamut_intersection(lab.a, lab.b, lab.lightness, 1.0, lab.lightness, cusp);
        let st_max = ST::from_cusp(cusp);
        let k = c_max / (lab.lightness * st_max.s).min((1.0 - lab.lightness) * st_max.t);
        let st_mid = ST::mid(lab.a, lab.b);

        // Use a soft minimum function, instead of a sharp triangle shape to get a smooth value for chroma.
        let c_a = lab.lightness * st_mid.s;
        let c_b = (1.0 - lab.lightness) * st_mid.t;
        let c_mid = 0.9 * k * (c_a.powi(-4) + c_b.powi(-4)).powi(-1).sqrt().sqrt();

        // for C_0, the shape is independent of hue, so ST are constant. Values picked to roughly be the average values of ST.
        let c_a = lab.lightness * 0.4;
        let c_b = (1.0 - lab.lightness) * 0.8;
        let c_0 = (c_a.powi(-2) + c_b.powi(-2)).powi(-1).sqrt();

        Self { c_0, c_mid, c_max }
    }
}

/// Finds intersection of the line defined by
/// L = L0 * (1 - t) + t * L1;
/// C = t * C1;
/// a and b must be normalized so a^2 + b^2 == 1
fn find_gamut_intersection(a: f64, b: f64, l1: f64, c1: f64, l0: f64, cusp: LC) -> f64 {
    // Find the intersection for upper and lower half seprately
    if (l1 - l0) * cusp.chroma <= (cusp.lightness - l0) * c1 {
        // Lower half
        cusp.chroma * l0 / (c1 * cusp.lightness + cusp.chroma * (l0 - l1))
    } else {
        // Upper half

        // First intersect with triangle
        let mut target =
            cusp.chroma * (l0 - 1.0) / (c1 * (cusp.lightness - 1.0) + cusp.chroma * (l0 - l1));
        // Then one step Halley's method

        let d_l = l1 - l0;
        let d_c = c1;
        let k_l = M1_OKLAB_TO_LIN_SRGB[[0, 1]] * a + M1_OKLAB_TO_LIN_SRGB[[0, 2]] * b;
        let k_m = M1_OKLAB_TO_LIN_SRGB[[1, 1]] * a + M1_OKLAB_TO_LIN_SRGB[[1, 2]] * b;
        let k_s = M1_OKLAB_TO_LIN_SRGB[[2, 1]] * a + M1_OKLAB_TO_LIN_SRGB[[2, 2]] * b;

        let l_dt = d_l + d_c * k_l;
        let m_dt = d_l + d_c * k_m;
        let s_dt = d_l + d_c * k_s;

        // If higher accuracy is required, 2 or 3 iterations of the following block can be used:
        {
            let l = l0 * (1.0 - target) + target * l1;
            let c = target * c1;

            let l_ = l + c * k_l;
            let m_ = l + c * k_m;
            let s_ = l + c * k_s;

            let l_c = l_.powi(3);
            let m_c = m_.powi(3);
            let s_c = s_.powi(3);

            let ldt = 3.0 * l_dt * l_ * l_;
            let mdt = 3.0 * m_dt * m_ * m_;
            let sdt = 3.0 * s_dt * s_ * s_;

            let ldt2 = 6.0 * l_dt * l_dt * l_;
            let mdt2 = 6.0 * m_dt * m_dt * m_;
            let sdt2 = 6.0 * s_dt * s_dt * s_;

            let r = M2_OKLAB_TO_LIN_SRGB[[0, 0]] * l_c
                + M2_OKLAB_TO_LIN_SRGB[[0, 1]] * m_c
                + M2_OKLAB_TO_LIN_SRGB[[0, 2]] * s_c
                - 1.0;
            let r1 = M2_OKLAB_TO_LIN_SRGB[[0, 0]] * ldt
                + M2_OKLAB_TO_LIN_SRGB[[0, 1]] * mdt
                + M2_OKLAB_TO_LIN_SRGB[[0, 2]] * sdt;
            let r2 = M2_OKLAB_TO_LIN_SRGB[[0, 0]] * ldt2
                + M2_OKLAB_TO_LIN_SRGB[[0, 1]] * mdt2
                + M2_OKLAB_TO_LIN_SRGB[[0, 2]] * sdt2;

            let u_r = r1 / (r1 * r1 - 0.5 * r * r2);
            let t_r = if u_r.is_sign_positive() {
                Some(-r * u_r)
            } else {
                None
            };

            let g = M2_OKLAB_TO_LIN_SRGB[[1, 0]] * l_c
                + M2_OKLAB_TO_LIN_SRGB[[1, 1]] * m_c
                + M2_OKLAB_TO_LIN_SRGB[[1, 2]] * s_c
                - 1.0;
            let g1 = M2_OKLAB_TO_LIN_SRGB[[1, 0]] * ldt
                + M2_OKLAB_TO_LIN_SRGB[[1, 1]] * mdt
                + M2_OKLAB_TO_LIN_SRGB[[1, 2]] * sdt;
            let g2 = M2_OKLAB_TO_LIN_SRGB[[1, 0]] * ldt2
                + M2_OKLAB_TO_LIN_SRGB[[1, 1]] * mdt2
                + M2_OKLAB_TO_LIN_SRGB[[1, 2]] * sdt2;

            let u_g = g1 / (g1 * g1 - 0.5 * g * g2);
            let t_g = if u_g.is_sign_positive() {
                Some(-g * u_g)
            } else {
                None
            };

            let b = M2_OKLAB_TO_LIN_SRGB[[2, 0]] * l_c
                + M2_OKLAB_TO_LIN_SRGB[[2, 1]] * m_c
                + M2_OKLAB_TO_LIN_SRGB[[2, 2]] * s_c
                - 1.0;
            let b1 = M2_OKLAB_TO_LIN_SRGB[[2, 0]] * ldt
                + M2_OKLAB_TO_LIN_SRGB[[2, 1]] * mdt
                + M2_OKLAB_TO_LIN_SRGB[[2, 2]] * sdt;
            let b2 = M2_OKLAB_TO_LIN_SRGB[[2, 0]] * ldt2
                + M2_OKLAB_TO_LIN_SRGB[[2, 1]] * mdt2
                + M2_OKLAB_TO_LIN_SRGB[[2, 2]] * sdt2;

            let u_b = b1 / (b1 * b1 - 0.5 * b * b2);
            let t_b = if u_b.is_sign_positive() {
                Some(-b * u_b)
            } else {
                None
            };

            target += [t_r, t_g, t_b]
                .into_iter()
                .flatten()
                .min_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal))
                .unwrap_or(0.0)
        }

        target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ACCEPTABLE_ERROR: f64 = 0.00001;

    #[test]
    fn okhsl_srgb() {
        let hue_steps = 200;
        let sat_steps = 200;
        let light_steps = 200;
        for hue_step in 0..hue_steps {
            for sat_step in 0..sat_steps {
                for light_step in 0..light_steps {
                    let init_col = OkHsl {
                        hue: hue_step as f64 * 2.0 * std::f64::consts::PI,
                        saturation: sat_step as f64 / sat_steps as f64,
                        lightness: light_step as f64 / light_steps as f64,
                    };
                    let return_col = OkHsl::from(Srgb::from(init_col));
                    // Comparing with f32 epsilon to allow some leeway
                    if init_col.hue.abs() > std::f32::EPSILON as f64 {
                        let error = (init_col.hue - return_col.hue).abs() / init_col.hue.abs();
                        assert!(
                            error < ACCEPTABLE_ERROR,
                            "The hue is too different: input {} -> {} output ({})",
                            init_col.hue,
                            return_col.hue,
                            error
                        );
                    } else {
                        assert!(
                            return_col.hue.abs() <= std::f32::EPSILON as f64,
                            "The hue should be negligible, got {} instead",
                            return_col.hue
                        );
                    }

                    if init_col.saturation.abs() > std::f32::EPSILON as f64 {
                        let error = (init_col.saturation - return_col.saturation).abs()
                            / init_col.saturation.abs();
                        assert!(
                            error < ACCEPTABLE_ERROR,
                            "The saturation is too different: input {} -> {} output ({})",
                            init_col.saturation,
                            return_col.saturation,
                            error
                        );
                    } else {
                        assert!(
                            return_col.saturation.abs() <= std::f32::EPSILON as f64,
                            "The saturation should be negligible, got {} instead",
                            return_col.saturation
                        );
                    }

                    if init_col.lightness.abs() > std::f32::EPSILON as f64 {
                        let error = (init_col.lightness - return_col.lightness).abs()
                            / init_col.lightness.abs();
                        assert!(
                            error < ACCEPTABLE_ERROR,
                            "The lightness is too different: input {} -> {} output ({})",
                            init_col.lightness,
                            return_col.lightness,
                            error
                        );
                    } else {
                        assert!(
                            return_col.lightness.abs() <= std::f32::EPSILON as f64,
                            "The lightness should be negligible, got {} instead",
                            return_col.lightness
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn okhsv_srgb() {
        let hue_steps = 200;
        let sat_steps = 200;
        let val_steps = 200;
        for hue_step in 0..hue_steps {
            for sat_step in 0..sat_steps {
                for val_step in 0..val_steps {
                    let init_col = OkHsv {
                        hue: hue_step as f64 * 2.0 * std::f64::consts::PI,
                        saturation: sat_step as f64 / sat_steps as f64,
                        value: val_step as f64 / val_steps as f64,
                    };
                    let return_col = OkHsv::from(Srgb::from(init_col));
                    // Comparing with f32 epsilon to allow some leeway
                    if init_col.hue.abs() > std::f32::EPSILON as f64 {
                        let error = (init_col.hue - return_col.hue).abs() / init_col.hue.abs();
                        assert!(
                            error < ACCEPTABLE_ERROR,
                            "The hue is too different: input {} -> {} output ({})",
                            init_col.hue,
                            return_col.hue,
                            error
                        );
                    } else {
                        assert!(
                            return_col.hue.abs() <= std::f32::EPSILON as f64,
                            "The hue should be negligible, got {} instead",
                            return_col.hue
                        );
                    }

                    if init_col.saturation.abs() > std::f32::EPSILON as f64 {
                        let error = (init_col.saturation - return_col.saturation).abs()
                            / init_col.saturation.abs();
                        assert!(
                            error < ACCEPTABLE_ERROR,
                            "The saturation is too different: input {} -> {} output ({})",
                            init_col.saturation,
                            return_col.saturation,
                            error
                        );
                    } else {
                        assert!(
                            return_col.saturation.abs() <= std::f32::EPSILON as f64,
                            "The saturation should be negligible, got {} instead",
                            return_col.saturation
                        );
                    }

                    if init_col.value.abs() > std::f32::EPSILON as f64 {
                        let error =
                            (init_col.value - return_col.value).abs() / init_col.value.abs();
                        assert!(
                            error < ACCEPTABLE_ERROR,
                            "The value is too different: input {} -> {} output ({})",
                            init_col.value,
                            return_col.value,
                            error
                        );
                    } else {
                        assert!(
                            return_col.value.abs() <= std::f32::EPSILON as f64,
                            "The value should be negligible, got {} instead",
                            return_col.value
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn oklab_srgb() {
        let light_steps = 200;
        let a_steps = 200;
        let b_steps = 200;
        for light_step in 0..light_steps {
            for a_step in 0..a_steps {
                for b_step in 0..b_steps {
                    let init_col = OkLab {
                        lightness: light_step as f64 / light_steps as f64,
                        a: -1.0 + 2.0 * a_step as f64 / a_steps as f64,
                        b: -1.0 + 2.0 * b_step as f64 / b_steps as f64,
                    };
                    let return_col = OkLab::from(LinSrgb::from(init_col));
                    // Comparing with f32 epsilon to allow some leeway
                    if init_col.a.abs() > std::f32::EPSILON as f64 {
                        let error = (init_col.a - return_col.a).abs() / init_col.a.abs();
                        assert!(
                            error < ACCEPTABLE_ERROR,
                            "The a is too different: input {} -> {} output ({})",
                            init_col.a,
                            return_col.a,
                            error
                        );
                    } else {
                        assert!(
                            return_col.a.abs() <= std::f32::EPSILON as f64,
                            "The a should be negligible, got {} instead",
                            return_col.a
                        );
                    }

                    if init_col.b.abs() > std::f32::EPSILON as f64 {
                        let error = (init_col.b - return_col.b).abs() / init_col.b.abs();
                        assert!(
                            error < ACCEPTABLE_ERROR,
                            "The b is too different: input {} -> {} output ({})",
                            init_col.b,
                            return_col.b,
                            error
                        );
                    } else {
                        assert!(
                            return_col.b.abs() <= std::f32::EPSILON as f64,
                            "The b should be negligible, got {} instead",
                            return_col.b
                        );
                    }

                    if init_col.lightness.abs() > std::f32::EPSILON as f64 {
                        let error = (init_col.lightness - return_col.lightness).abs()
                            / init_col.lightness.abs();
                        assert!(
                            error < ACCEPTABLE_ERROR,
                            "The lightness is too different: input {} -> {} output ({})",
                            init_col.lightness,
                            return_col.lightness,
                            error
                        );
                    } else {
                        assert!(
                            return_col.lightness.abs() <= std::f32::EPSILON as f64,
                            "The lightness should be negligible, got {} instead",
                            return_col.lightness
                        );
                    }
                }
            }
        }
    }
}
