//! Effect system for Composer.

use serde::{Deserialize, Serialize};

/// Video effect types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VideoEffectType {
    Brightness,
    Contrast,
    Saturation,
    HueRotate,
    Blur,
    Sharpen,
    Opacity,
    Invert,
    Vignette,
    ColorBalance,
    Crop,
    Scale,
}

impl VideoEffectType {
    pub const ALL: [Self; 12] = [
        Self::Brightness,
        Self::Contrast,
        Self::Saturation,
        Self::HueRotate,
        Self::Blur,
        Self::Sharpen,
        Self::Opacity,
        Self::Invert,
        Self::Vignette,
        Self::ColorBalance,
        Self::Crop,
        Self::Scale,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Brightness => "Brightness",
            Self::Contrast => "Contrast",
            Self::Saturation => "Saturation",
            Self::HueRotate => "Hue Rotate",
            Self::Blur => "Blur",
            Self::Sharpen => "Sharpen",
            Self::Opacity => "Opacity",
            Self::Invert => "Invert",
            Self::Vignette => "Vignette",
            Self::ColorBalance => "Color Balance",
            Self::Crop => "Crop",
            Self::Scale => "Scale",
        }
    }

    /// Default parameter value for this effect type.
    pub const fn default_param(self) -> f64 {
        match self {
            Self::Brightness => 0.0,
            Self::Contrast => 1.0,
            Self::Saturation => 1.0,
            Self::HueRotate => 0.0,
            Self::Blur => 0.0,
            Self::Sharpen => 0.0,
            Self::Opacity => 1.0,
            Self::Invert => 0.0,
            Self::Vignette => 0.0,
            Self::ColorBalance => 0.0,
            Self::Crop => 1.0,
            Self::Scale => 1.0,
        }
    }
}

/// Audio effect types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudioEffectType {
    Volume,
    FadeIn,
    FadeOut,
    Eq,
    Compressor,
    NoiseGate,
    Pan,
}

impl AudioEffectType {
    pub const ALL: [Self; 7] = [
        Self::Volume,
        Self::FadeIn,
        Self::FadeOut,
        Self::Eq,
        Self::Compressor,
        Self::NoiseGate,
        Self::Pan,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Volume => "Volume",
            Self::FadeIn => "Fade In",
            Self::FadeOut => "Fade Out",
            Self::Eq => "EQ",
            Self::Compressor => "Compressor",
            Self::NoiseGate => "Noise Gate",
            Self::Pan => "Pan",
        }
    }
}

/// A keyframe for parameter animation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyframe {
    /// Frame number on the timeline.
    pub frame: u32,
    /// Parameter value at this keyframe.
    pub value: f64,
    /// Interpolation type to the next keyframe.
    pub interpolation: Interpolation,
}

/// Interpolation type between keyframes.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum Interpolation {
    #[default]
    Linear,
    Bezier,
    Hold,
}

/// An effect applied to a clip.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    pub id: u64,
    pub kind: EffectKind,
    /// Named parameters with their current values.
    pub params: Vec<(String, f64)>,
    /// Keyframes per parameter name.
    pub keyframes: Vec<(String, Vec<Keyframe>)>,
    pub enabled: bool,
}

/// Effect kind: video or audio.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectKind {
    Video(VideoEffectType),
    Audio(AudioEffectType),
}

impl Effect {
    pub fn new_video(id: u64, kind: VideoEffectType) -> Self {
        let default_val = kind.default_param();
        Self {
            id,
            kind: EffectKind::Video(kind),
            params: vec![("value".into(), default_val)],
            keyframes: Vec::new(),
            enabled: true,
        }
    }

    pub fn new_audio(id: u64, kind: AudioEffectType) -> Self {
        Self {
            id,
            kind: EffectKind::Audio(kind),
            params: vec![("value".into(), 1.0)],
            keyframes: Vec::new(),
            enabled: true,
        }
    }

    /// Get a parameter value, interpolating keyframes at the given frame.
    pub fn param_at_frame(&self, name: &str, frame: u32) -> f64 {
        // Check keyframes first.
        for (param_name, keyframes) in &self.keyframes {
            if param_name == name && keyframes.len() >= 2 {
                return Self::interpolate_keyframes(keyframes, frame);
            }
        }
        // Fall back to static param.
        self.params
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, v)| *v)
            .unwrap_or(0.0)
    }

    /// Linear interpolation between keyframes.
    fn interpolate_keyframes(keyframes: &[Keyframe], frame: u32) -> f64 {
        if keyframes.is_empty() {
            return 0.0;
        }
        if frame <= keyframes[0].frame {
            return keyframes[0].value;
        }
        if frame >= keyframes.last().unwrap().frame {
            return keyframes.last().unwrap().value;
        }

        // Find surrounding keyframes.
        let idx = keyframes.partition_point(|k| k.frame < frame);
        let prev = &keyframes[idx - 1];
        let next = &keyframes[idx];

        match prev.interpolation {
            Interpolation::Hold => prev.value,
            Interpolation::Linear => {
                let t = (frame - prev.frame) as f64 / (next.frame - prev.frame) as f64;
                prev.value + (next.value - prev.value) * t
            }
            Interpolation::Bezier => {
                // Simplified bezier — use linear for now.
                let t = (frame - prev.frame) as f64 / (next.frame - prev.frame) as f64;
                prev.value + (next.value - prev.value) * t
            }
        }
    }

    /// Add a keyframe for a parameter.
    pub fn add_keyframe(&mut self, param_name: &str, keyframe: Keyframe) {
        if let Some(entry) = self.keyframes.iter_mut().find(|(n, _)| n == param_name) {
            // Insert sorted by frame.
            let idx = entry.1.partition_point(|k| k.frame < keyframe.frame);
            entry.1.insert(idx, keyframe);
        } else {
            self.keyframes.push((param_name.into(), vec![keyframe]));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn video_effect_default_params() {
        for t in VideoEffectType::ALL {
            let e = Effect::new_video(1, t);
            assert!(!e.params.is_empty());
            assert!(e.enabled);
        }
    }

    #[test]
    fn keyframe_interpolation() {
        let mut e = Effect::new_video(1, VideoEffectType::Brightness);
        e.add_keyframe(
            "value",
            Keyframe {
                frame: 0,
                value: 0.0,
                interpolation: Interpolation::Linear,
            },
        );
        e.add_keyframe(
            "value",
            Keyframe {
                frame: 100,
                value: 1.0,
                interpolation: Interpolation::Linear,
            },
        );

        assert!((e.param_at_frame("value", 0) - 0.0).abs() < 0.001);
        assert!((e.param_at_frame("value", 50) - 0.5).abs() < 0.001);
        assert!((e.param_at_frame("value", 100) - 1.0).abs() < 0.001);
    }

    #[test]
    fn keyframe_hold_interpolation() {
        let mut e = Effect::new_video(1, VideoEffectType::Opacity);
        e.add_keyframe(
            "value",
            Keyframe {
                frame: 0,
                value: 1.0,
                interpolation: Interpolation::Hold,
            },
        );
        e.add_keyframe(
            "value",
            Keyframe {
                frame: 100,
                value: 0.0,
                interpolation: Interpolation::Linear,
            },
        );

        // At frame 50, hold should return 1.0 (previous value).
        assert!((e.param_at_frame("value", 50) - 1.0).abs() < 0.001);
    }

    #[test]
    fn all_effects_have_labels() {
        for v in VideoEffectType::ALL {
            assert!(!v.label().is_empty());
        }
        for a in AudioEffectType::ALL {
            assert!(!a.label().is_empty());
        }
    }
}
