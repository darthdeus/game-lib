pub use itertools::Itertools;
pub use macroquad::rand::gen_range;
// pub use nanoserde::{SerJson, DeJson};
pub use serde::{Deserialize, Serialize};

pub use macroquad::prelude::*;

pub use std::collections::HashMap;

pub use egui;
pub use egui_macroquad;
pub use serde_json;
pub use serde;
pub use macroquad;
pub use anyhow::Result;

pub use macroquad_particles::{BlendMode, ColorCurve, Curve, Emitter, EmitterConfig};

pub fn load_json_from_file_or_default<T: for<'a> Deserialize<'a> + Default>(
    path: &str,
) -> T {
    let doit = || -> Result<T> {
        let data = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str::<T>(&data)?)
    };

    doit().unwrap_or_else(|err| {
        error!("Error: {}", err);
        T::default()
    })
}

pub fn save_json_to_file<T: Serialize>(path: &str, val: &T) {
    let doit = || -> Result<()> {
        let json_data = serde_json::to_string_pretty(val)?;
        std::fs::write(path, json_data)?;

        Ok(())
    };

    doit().unwrap_or_else(|err| {
        error!("Error: {}", err);
    });
}

pub fn toss_coin(p: f32) -> bool {
    gen_range(0.0, 1.0) < p
}

pub fn random_around(position: Vec2, min: f32, max: f32) -> Vec2 {
    let r = gen_range(min, max);
    let angle = gen_range(0.0, std::f32::consts::PI * 2.0);

    position + Vec2::new(angle.cos() * r, angle.sin() * r)
}

