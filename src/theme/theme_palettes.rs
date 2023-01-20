use super::{get_alpha_color, get_solid_color};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ThemePalettes {
    pub color_text_base: String,
    pub color_bg_base: String,

    pub color_text: String,
    pub color_text_secondary: String,
    pub color_text_tertiary: String,
    pub color_text_quaternary: String,

    pub color_fill: String,
    pub color_fill_secondary: String,
    pub color_fill_tertiary: String,
    pub color_fill_quaternary: String,

    pub color_bg_layout: String,
    pub color_bg_container: String,
    pub color_bg_elevated: String,
    pub color_bg_spotlight: String,

    pub color_border: String,
    pub color_border_secondary: String,
}

pub fn theme_light(palette: &mut ThemePalettes) {
    if palette.color_text_base.is_empty() {
        palette.color_text_base = "#000".into()
    }
    if palette.color_bg_base.is_empty() {
        palette.color_bg_base = "#fff".into()
    }

    palette.color_text = get_alpha_color(&palette.color_text_base, 0.88).unwrap();
    palette.color_text_secondary = get_alpha_color(&palette.color_text_base, 0.65).unwrap();
    palette.color_text_tertiary = get_alpha_color(&palette.color_text_base, 0.45).unwrap();
    palette.color_text_quaternary = get_alpha_color(&palette.color_text_base, 0.25).unwrap();

    palette.color_fill = get_alpha_color(&palette.color_text_base, 0.15).unwrap();
    palette.color_fill_secondary = get_alpha_color(&palette.color_text_base, 0.06).unwrap();
    palette.color_fill_tertiary = get_alpha_color(&palette.color_text_base, 0.04).unwrap();
    palette.color_fill_quaternary = get_alpha_color(&palette.color_text_base, 0.02).unwrap();

    palette.color_bg_layout = get_solid_color(&palette.color_bg_base, 4.0 / 255.0).unwrap();
    palette.color_bg_container = get_solid_color(&palette.color_bg_base, 0.0).unwrap();
    palette.color_bg_elevated = get_solid_color(&palette.color_bg_base, 0.0).unwrap();
    palette.color_bg_spotlight = get_alpha_color(&palette.color_text_base, 0.85).unwrap();

    palette.color_border = get_solid_color(&palette.color_bg_base, 15.0 / 255.0).unwrap();
    palette.color_border_secondary = get_solid_color(&palette.color_bg_base, 6.0 / 255.0).unwrap();
}

pub fn theme_dark(palette: &mut ThemePalettes) {
    if palette.color_text_base.is_empty() {
        palette.color_text_base = "#fff".into()
    }
    if palette.color_bg_base.is_empty() {
        palette.color_bg_base = "#000".into()
    }

    palette.color_text = get_alpha_color(&palette.color_text_base, 0.88).unwrap();
    palette.color_text_secondary = get_alpha_color(&palette.color_text_base, 0.65).unwrap();
    palette.color_text_tertiary = get_alpha_color(&palette.color_text_base, 0.45).unwrap();
    palette.color_text_quaternary = get_alpha_color(&palette.color_text_base, 0.25).unwrap();

    palette.color_fill = get_alpha_color(&palette.color_text_base, 0.18).unwrap();
    palette.color_fill_secondary = get_alpha_color(&palette.color_text_base, 0.12).unwrap();
    palette.color_fill_tertiary = get_alpha_color(&palette.color_text_base, 0.08).unwrap();
    palette.color_fill_quaternary = get_alpha_color(&palette.color_text_base, 0.04).unwrap();

    palette.color_bg_layout = get_solid_color(&palette.color_bg_base, 0.0).unwrap();
    palette.color_bg_container = get_solid_color(&palette.color_bg_base, 8.0 / 255.0).unwrap();
    palette.color_bg_elevated = get_solid_color(&palette.color_bg_base, 12.0 / 255.0).unwrap();
    palette.color_bg_spotlight = get_solid_color(&palette.color_text_base, 26.0 / 255.0).unwrap();

    palette.color_border = get_solid_color(&palette.color_bg_base, 26.0 / 255.0).unwrap();
    palette.color_border_secondary = get_solid_color(&palette.color_bg_base, 19.0 / 255.0).unwrap();
}
