pub use itertools::Itertools;
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

pub use macroquad_particles::{BlendMode, Curve, Emitter, EmitterConfig};

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
