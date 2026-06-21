//! Transition types for Composer.

use serde::{Deserialize, Serialize};

/// Transition type between two clips.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransitionType {
    CrossDissolve,
    DipToBlack,
    DipToWhite,
    WipeLeft,
    WipeRight,
    SlideLeft,
    SlideRight,
    FadeIn,
    FadeOut,
}

impl TransitionType {
    pub const ALL: [Self; 9] = [
        Self::CrossDissolve,
        Self::DipToBlack,
        Self::DipToWhite,
        Self::WipeLeft,
        Self::WipeRight,
        Self::SlideLeft,
        Self::SlideRight,
        Self::FadeIn,
        Self::FadeOut,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::CrossDissolve => "Cross Dissolve",
            Self::DipToBlack => "Dip to Black",
            Self::DipToWhite => "Dip to White",
            Self::WipeLeft => "Wipe Left",
            Self::WipeRight => "Wipe Right",
            Self::SlideLeft => "Slide Left",
            Self::SlideRight => "Slide Right",
            Self::FadeIn => "Fade In",
            Self::FadeOut => "Fade Out",
        }
    }
}

/// Easing curve for transitions.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum TransitionCurve {
    Linear,
    EaseIn,
    EaseOut,
    #[default]
    EaseInOut,
}

/// A transition between two clips.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    pub id: u64,
    pub kind: TransitionType,
    /// Duration in frames.
    pub duration: u32,
    pub curve: TransitionCurve,
}

impl Transition {
    pub fn new(id: u64, kind: TransitionType, duration: u32) -> Self {
        Self {
            id,
            kind,
            duration,
            curve: TransitionCurve::default(),
        }
    }

    /// Compute the interpolation factor (0.0–1.0) for a given progress.
    pub fn interpolate(&self, progress: f64) -> f64 {
        match self.curve {
            TransitionCurve::Linear => progress,
            TransitionCurve::EaseIn => progress * progress,
            TransitionCurve::EaseOut => 1.0 - (1.0 - progress) * (1.0 - progress),
            TransitionCurve::EaseInOut => {
                if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    1.0 - (-2.0 * progress + 2.0).powi(2) / 2.0
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ease_in_out_curve() {
        let t = Transition::new(1, TransitionType::CrossDissolve, 24);
        let t_ease = Transition {
            curve: TransitionCurve::EaseInOut,
            ..t
        };

        let mid = t_ease.interpolate(0.5);
        assert!((mid - 0.5).abs() < 0.01);

        let start = t_ease.interpolate(0.0);
        assert_eq!(start, 0.0);

        let end = t_ease.interpolate(1.0);
        assert!((end - 1.0).abs() < 0.001);
    }

    #[test]
    fn all_transitions_have_labels() {
        for t in TransitionType::ALL {
            assert!(!t.label().is_empty());
        }
    }
}
