use std::str::FromStr;

use anyhow::Result;
use palette::{convert::IntoColorUnclamped, Hsla, IntoColor, Shade, Srgb, Srgba, WithAlpha};

pub fn get_alpha_color(base_color: &str, alpha: f64) -> Result<String> {
    let srgb = Srgb::from_str(base_color)?;
    let alpha = srgb.with_alpha(alpha);
    let srgba: Srgba<u8> = alpha.into_format();
    Ok(format!("#{srgba:02X}"))
}

pub fn get_solid_color(base_color: &str, brightness: f32) -> Result<String> {
    let srgb = Srgb::from_str(base_color)?;
    let alpha = srgb.with_alpha(1.0);
    let hsla: Hsla = alpha.into_format().into_color();
    let hsla = hsla.darken(brightness);
    let srgba: Srgba = hsla.into_color_unclamped();
    let srgba: Srgba<u8> = srgba.into_format();
    Ok(format!("#{srgba:02X}"))
}

#[test]
fn it_works() -> Result<()> {
    let base_color = "#5839a4";

    println!("{base_color}  {}", get_alpha_color(base_color, 0.5)?);

    println!("{base_color}  {}", get_solid_color(base_color, 0.5)?);

    Ok(())
}
