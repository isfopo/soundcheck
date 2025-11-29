//! Audio level smoothing and processing utilities

/// Convert decibels to linear amplitude
pub fn db_to_amplitude(db: f32) -> f32 {
    10.0f32.powf(db / 20.0)
}
