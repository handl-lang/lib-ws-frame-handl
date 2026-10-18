//! `frame` — Handl stdlib port.

/// Checkout root of this repo (build-time).
pub const ROOT: &str = env!("CARGO_MANIFEST_DIR");

/// `frame.hnl`.
pub const FRAME_HNL: &str = include_str!("frame.hnl");
