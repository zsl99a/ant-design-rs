use serde::Serialize;

use super::{generate, get_alpha_color, ThemeVars};

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct ThemeColors {
    pub color_primary_bg: String,
    pub color_primary_bg_hover: String,
    pub color_primary_border: String,
    pub color_primary_border_hover: String,
    pub color_primary_hover: String,
    pub color_primary: String,
    pub color_primary_active: String,
    pub color_primary_text_hover: String,
    pub color_primary_text: String,
    pub color_primary_text_active: String,

    pub color_success_bg: String,
    pub color_success_bg_hover: String,
    pub color_success_border: String,
    pub color_success_border_hover: String,
    pub color_success_hover: String,
    pub color_success: String,
    pub color_success_active: String,
    pub color_success_text_hover: String,
    pub color_success_text: String,
    pub color_success_text_active: String,

    pub color_warning_bg: String,
    pub color_warning_bg_hover: String,
    pub color_warning_border: String,
    pub color_warning_border_hover: String,
    pub color_warning_hover: String,
    pub color_warning: String,
    pub color_warning_active: String,
    pub color_warning_text_hover: String,
    pub color_warning_text: String,
    pub color_warning_text_active: String,

    pub color_error_bg: String,
    pub color_error_bg_hover: String,
    pub color_error_border: String,
    pub color_error_border_hover: String,
    pub color_error_hover: String,
    pub color_error: String,
    pub color_error_active: String,
    pub color_error_text_hover: String,
    pub color_error_text: String,
    pub color_error_text_active: String,

    pub color_info_bg: String,
    pub color_info_bg_hover: String,
    pub color_info_border: String,
    pub color_info_border_hover: String,
    pub color_info_hover: String,
    pub color_info: String,
    pub color_info_active: String,
    pub color_info_text_hover: String,
    pub color_info_text: String,
    pub color_info_text_active: String,

    pub color_bg_mask: String,
    pub color_white: String,
}

impl ThemeColors {
    pub fn new(core: &ThemeVars) -> Self {
        let primary_colors = generate(&core.color_primary, false, "").unwrap();
        let success_colors = generate(&core.color_success, false, "").unwrap();
        let warning_colors = generate(&core.color_warning, false, "").unwrap();
        let error_colors = generate(&core.color_error, false, "").unwrap();
        let info_colors = generate(&core.color_info, false, "").unwrap();

        Self {
            color_primary_bg: primary_colors[0].clone(),
            color_primary_bg_hover: primary_colors[1].clone(),
            color_primary_border: primary_colors[2].clone(),
            color_primary_border_hover: primary_colors[3].clone(),
            color_primary_hover: primary_colors[4].clone(),
            color_primary: primary_colors[5].clone(),
            color_primary_active: primary_colors[6].clone(),
            color_primary_text_hover: primary_colors[7].clone(),
            color_primary_text: primary_colors[8].clone(),
            color_primary_text_active: primary_colors[9].clone(),

            color_success_bg: success_colors[0].clone(),
            color_success_bg_hover: success_colors[1].clone(),
            color_success_border: success_colors[2].clone(),
            color_success_border_hover: success_colors[3].clone(),
            color_success_hover: success_colors[4].clone(),
            color_success: success_colors[5].clone(),
            color_success_active: success_colors[6].clone(),
            color_success_text_hover: success_colors[7].clone(),
            color_success_text: success_colors[8].clone(),
            color_success_text_active: success_colors[9].clone(),

            color_warning_bg: warning_colors[0].clone(),
            color_warning_bg_hover: warning_colors[1].clone(),
            color_warning_border: warning_colors[2].clone(),
            color_warning_border_hover: warning_colors[3].clone(),
            color_warning_hover: warning_colors[4].clone(),
            color_warning: warning_colors[5].clone(),
            color_warning_active: warning_colors[6].clone(),
            color_warning_text_hover: warning_colors[7].clone(),
            color_warning_text: warning_colors[8].clone(),
            color_warning_text_active: warning_colors[9].clone(),

            color_error_bg: error_colors[0].clone(),
            color_error_bg_hover: error_colors[1].clone(),
            color_error_border: error_colors[2].clone(),
            color_error_border_hover: error_colors[3].clone(),
            color_error_hover: error_colors[4].clone(),
            color_error: error_colors[5].clone(),
            color_error_active: error_colors[6].clone(),
            color_error_text_hover: error_colors[7].clone(),
            color_error_text: error_colors[8].clone(),
            color_error_text_active: error_colors[9].clone(),

            color_info_bg: info_colors[0].clone(),
            color_info_bg_hover: info_colors[1].clone(),
            color_info_border: info_colors[2].clone(),
            color_info_border_hover: info_colors[3].clone(),
            color_info_hover: info_colors[4].clone(),
            color_info: info_colors[5].clone(),
            color_info_active: info_colors[6].clone(),
            color_info_text_hover: info_colors[7].clone(),
            color_info_text: info_colors[8].clone(),
            color_info_text_active: info_colors[9].clone(),

            color_bg_mask: get_alpha_color("#000", 0.45).unwrap(),
            color_white: "#fff".into(),
        }
    }
}
