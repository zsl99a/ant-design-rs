use super::{get_alpha_color, get_solid_color};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ThemeNeutral {
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

pub fn theme_light(neutral: &mut ThemeNeutral) {
    if neutral.color_text_base.is_empty() {
        neutral.color_text_base = "#000".into()
    }
    if neutral.color_bg_base.is_empty() {
        neutral.color_bg_base = "#fff".into()
    }

    neutral.color_text = get_alpha_color(&neutral.color_text_base, 0.88).unwrap();
    neutral.color_text_secondary = get_alpha_color(&neutral.color_text_base, 0.65).unwrap();
    neutral.color_text_tertiary = get_alpha_color(&neutral.color_text_base, 0.45).unwrap();
    neutral.color_text_quaternary = get_alpha_color(&neutral.color_text_base, 0.25).unwrap();

    neutral.color_fill = get_alpha_color(&neutral.color_text_base, 0.15).unwrap();
    neutral.color_fill_secondary = get_alpha_color(&neutral.color_text_base, 0.06).unwrap();
    neutral.color_fill_tertiary = get_alpha_color(&neutral.color_text_base, 0.04).unwrap();
    neutral.color_fill_quaternary = get_alpha_color(&neutral.color_text_base, 0.02).unwrap();

    neutral.color_bg_layout = get_solid_color(&neutral.color_bg_base, 0.04).unwrap();
    neutral.color_bg_container = get_solid_color(&neutral.color_bg_base, 0.0).unwrap();
    neutral.color_bg_elevated = get_solid_color(&neutral.color_bg_base, 0.0).unwrap();
    neutral.color_bg_spotlight = get_alpha_color(&neutral.color_text_base, 0.85).unwrap();

    neutral.color_border = get_solid_color(&neutral.color_bg_base, 0.15).unwrap();
    neutral.color_border_secondary = get_solid_color(&neutral.color_bg_base, 0.06).unwrap();
}

pub fn theme_dark(neutral: &mut ThemeNeutral) {
    if neutral.color_text_base.is_empty() {
        neutral.color_text_base = "#fff".into()
    }
    if neutral.color_bg_base.is_empty() {
        neutral.color_bg_base = "#000".into()
    }

    neutral.color_text = get_alpha_color(&neutral.color_text_base, 0.88).unwrap();
    neutral.color_text_secondary = get_alpha_color(&neutral.color_text_base, 0.65).unwrap();
    neutral.color_text_tertiary = get_alpha_color(&neutral.color_text_base, 0.45).unwrap();
    neutral.color_text_quaternary = get_alpha_color(&neutral.color_text_base, 0.25).unwrap();

    neutral.color_fill = get_alpha_color(&neutral.color_text_base, 0.18).unwrap();
    neutral.color_fill_secondary = get_alpha_color(&neutral.color_text_base, 0.12).unwrap();
    neutral.color_fill_tertiary = get_alpha_color(&neutral.color_text_base, 0.08).unwrap();
    neutral.color_fill_quaternary = get_alpha_color(&neutral.color_text_base, 0.04).unwrap();

    neutral.color_bg_layout = get_solid_color(&neutral.color_bg_base, 0.0).unwrap();
    neutral.color_bg_container = get_solid_color(&neutral.color_bg_base, 0.08).unwrap();
    neutral.color_bg_elevated = get_solid_color(&neutral.color_bg_base, 0.12).unwrap();
    neutral.color_bg_spotlight = get_solid_color(&neutral.color_text_base, 0.26).unwrap();

    neutral.color_border = get_solid_color(&neutral.color_bg_base, 0.26).unwrap();
    neutral.color_border_secondary = get_solid_color(&neutral.color_bg_base, 0.19).unwrap();
}
