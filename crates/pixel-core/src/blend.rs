//! Blend modes for layer compositing.

/// Blend mode determines how a layer's pixels are combined with the pixels below.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

impl BlendMode {
    pub const ALL: [BlendMode; 16] = [
        BlendMode::Normal,
        BlendMode::Multiply,
        BlendMode::Screen,
        BlendMode::Overlay,
        BlendMode::Darken,
        BlendMode::Lighten,
        BlendMode::ColorDodge,
        BlendMode::ColorBurn,
        BlendMode::HardLight,
        BlendMode::SoftLight,
        BlendMode::Difference,
        BlendMode::Exclusion,
        BlendMode::Hue,
        BlendMode::Saturation,
        BlendMode::Color,
        BlendMode::Luminosity,
    ];

    pub fn label(self) -> &'static str {
        match self {
            BlendMode::Normal => "Normal",
            BlendMode::Multiply => "Multiply",
            BlendMode::Screen => "Screen",
            BlendMode::Overlay => "Overlay",
            BlendMode::Darken => "Darken",
            BlendMode::Lighten => "Lighten",
            BlendMode::ColorDodge => "Color Dodge",
            BlendMode::ColorBurn => "Color Burn",
            BlendMode::HardLight => "Hard Light",
            BlendMode::SoftLight => "Soft Light",
            BlendMode::Difference => "Difference",
            BlendMode::Exclusion => "Exclusion",
            BlendMode::Hue => "Hue",
            BlendMode::Saturation => "Saturation",
            BlendMode::Color => "Color",
            BlendMode::Luminosity => "Luminosity",
        }
    }

    /// Applies the blend mode to a single pixel pair.
    ///
    /// `src` is the upper layer pixel, `dst` is the lower layer pixel.
    /// All values are in 0..=255 range.
    /// Returns the blended (R, G, B, A) tuple.
    pub fn blend_pixel(self, src: (u8, u8, u8, u8), dst: (u8, u8, u8, u8)) -> (u8, u8, u8, u8) {
        let sa = src.3 as f32 / 255.0;
        let da = dst.3 as f32 / 255.0;

        let sr = src.0 as f32 / 255.0;
        let sg = src.1 as f32 / 255.0;
        let sb = src.2 as f32 / 255.0;
        let dr = dst.0 as f32 / 255.0;
        let dg = dst.1 as f32 / 255.0;
        let db = dst.2 as f32 / 255.0;

        let (cr, cg, cb) = match self {
            BlendMode::Normal => (sr, sg, sb),
            BlendMode::Multiply => (sr * dr, sg * dg, sb * db),
            BlendMode::Screen => (sr + dr - sr * dr, sg + dg - sg * dg, sb + db - sb * db),
            BlendMode::Overlay => (
                if dr <= 0.5 {
                    2.0 * sr * dr
                } else {
                    1.0 - 2.0 * (1.0 - sr) * (1.0 - dr)
                },
                if dg <= 0.5 {
                    2.0 * sg * dg
                } else {
                    1.0 - 2.0 * (1.0 - sg) * (1.0 - dg)
                },
                if db <= 0.5 {
                    2.0 * sb * db
                } else {
                    1.0 - 2.0 * (1.0 - sb) * (1.0 - db)
                },
            ),
            BlendMode::Darken => (sr.min(dr), sg.min(dg), sb.min(db)),
            BlendMode::Lighten => (sr.max(dr), sg.max(dg), sb.max(db)),
            BlendMode::ColorDodge => (
                if dr >= 1.0 {
                    1.0
                } else {
                    (sr / (1.0 - dr)).min(1.0)
                },
                if dg >= 1.0 {
                    1.0
                } else {
                    (sg / (1.0 - dg)).min(1.0)
                },
                if db >= 1.0 {
                    1.0
                } else {
                    (sb / (1.0 - db)).min(1.0)
                },
            ),
            BlendMode::ColorBurn => (
                if dr <= 0.0 {
                    0.0
                } else {
                    (1.0 - ((1.0 - sr) / dr).min(1.0)).max(0.0)
                },
                if dg <= 0.0 {
                    0.0
                } else {
                    (1.0 - ((1.0 - sg) / dg).min(1.0)).max(0.0)
                },
                if db <= 0.0 {
                    0.0
                } else {
                    (1.0 - ((1.0 - sb) / db).min(1.0)).max(0.0)
                },
            ),
            BlendMode::HardLight => (
                if sr <= 0.5 {
                    2.0 * sr * dr
                } else {
                    1.0 - 2.0 * (1.0 - sr) * (1.0 - dr)
                },
                if sg <= 0.5 {
                    2.0 * sg * dg
                } else {
                    1.0 - 2.0 * (1.0 - sg) * (1.0 - dg)
                },
                if sb <= 0.5 {
                    2.0 * sb * db
                } else {
                    1.0 - 2.0 * (1.0 - sb) * (1.0 - db)
                },
            ),
            BlendMode::SoftLight => (soft_light(sr, dr), soft_light(sg, dg), soft_light(sb, db)),
            BlendMode::Difference => ((sr - dr).abs(), (sg - dg).abs(), (sb - db).abs()),
            BlendMode::Exclusion => (
                sr + dr - 2.0 * sr * dr,
                sg + dg - 2.0 * sg * dg,
                sb + db - 2.0 * sb * db,
            ),
            BlendMode::Hue | BlendMode::Saturation | BlendMode::Color | BlendMode::Luminosity => {
                // HSL-based blend modes: convert to HSL, combine, convert back
                let (sh, ss, sl) = rgb_to_hsl(sr, sg, sb);
                let (dh, ds, dl) = rgb_to_hsl(dr, dg, db);
                let (rh, rs, rl) = match self {
                    BlendMode::Hue => (sh, ds, dl),
                    BlendMode::Saturation => (dh, ss, dl),
                    BlendMode::Color => (sh, ss, dl),
                    BlendMode::Luminosity => (dh, ds, sl),
                    _ => unreachable!(),
                };
                let (rr, rg, rb) = hsl_to_rgb(rh, rs, rl);
                (rr, rg, rb)
            }
        };

        // Alpha compositing (Porter-Duff over operator with blend function)
        // The blended color `cr` replaces the source color in the compositing step.
        let out_a = sa + da * (1.0 - sa);
        let out_r = if out_a > 0.0 {
            (cr * sa + dr * da * (1.0 - sa)) / out_a
        } else {
            0.0
        };
        let out_g = if out_a > 0.0 {
            (cg * sa + dg * da * (1.0 - sa)) / out_a
        } else {
            0.0
        };
        let out_b = if out_a > 0.0 {
            (cb * sa + db * da * (1.0 - sa)) / out_a
        } else {
            0.0
        };

        (
            (out_r.clamp(0.0, 1.0) * 255.0) as u8,
            (out_g.clamp(0.0, 1.0) * 255.0) as u8,
            (out_b.clamp(0.0, 1.0) * 255.0) as u8,
            (out_a.clamp(0.0, 1.0) * 255.0) as u8,
        )
    }
}

/// Soft light blending helper (Photoshop formula).
fn soft_light(s: f32, d: f32) -> f32 {
    if s <= 0.5 {
        d - (1.0 - 2.0 * s) * d * (1.0 - d)
    } else {
        let dd = if d <= 0.25 {
            ((16.0 * d - 12.0) * d + 4.0) * d
        } else {
            d.sqrt()
        };
        d + (2.0 * s - 1.0) * (dd - d)
    }
}

/// Convert RGB (0..1) to HSL (h: 0..360, s: 0..1, l: 0..1).
fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    if (max - min).abs() < f32::EPSILON {
        return (0.0, 0.0, l);
    }

    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };

    let h = if (max - r).abs() < f32::EPSILON {
        (g - b) / d + if g < b { 6.0 } else { 0.0 }
    } else if (max - g).abs() < f32::EPSILON {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };

    (h * 60.0, s, l)
}

/// Convert HSL to RGB (0..1).
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    if s.abs() < f32::EPSILON {
        return (l, l, l);
    }

    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;
    let h_norm = h / 360.0;

    (
        hue_to_rgb(p, q, h_norm + 1.0 / 3.0),
        hue_to_rgb(p, q, h_norm),
        hue_to_rgb(p, q, h_norm - 1.0 / 3.0),
    )
}

fn hue_to_rgb(p: f32, q: f32, t: f32) -> f32 {
    let t = if t < 0.0 {
        t + 1.0
    } else if t > 1.0 {
        t - 1.0
    } else {
        t
    };
    if t < 1.0 / 6.0 {
        p + (q - p) * 6.0 * t
    } else if t < 0.5 {
        q
    } else if t < 2.0 / 3.0 {
        p + (q - p) * (2.0 / 3.0 - t) * 6.0
    } else {
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_blend_identity() {
        let result = BlendMode::Normal.blend_pixel((100, 150, 200, 255), (50, 75, 100, 255));
        // With full alpha source, result should be close to source
        assert_eq!(result.0, 100);
        assert_eq!(result.1, 150);
        assert_eq!(result.2, 200);
        assert_eq!(result.3, 255);
    }

    #[test]
    fn multiply_darkens() {
        let result = BlendMode::Multiply.blend_pixel((128, 128, 128, 255), (255, 255, 255, 255));
        // 128/255 * 255/255 ~ 128
        assert!(result.0 < 130 && result.0 > 125);
    }

    #[test]
    fn screen_lightens() {
        let result = BlendMode::Screen.blend_pixel((128, 128, 128, 255), (128, 128, 128, 255));
        // Screen should produce a brighter result than input
        assert!(result.0 > 128);
    }

    #[test]
    fn all_blend_modes_produce_valid_output() {
        for mode in BlendMode::ALL {
            let result = mode.blend_pixel((200, 100, 50, 200), (100, 200, 150, 180));
            assert!(result.3 > 0);
        }
    }

    #[test]
    fn label_returns_non_empty() {
        for mode in BlendMode::ALL {
            assert!(!mode.label().is_empty());
        }
    }
}
