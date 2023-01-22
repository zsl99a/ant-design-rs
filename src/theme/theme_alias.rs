use serde::Serialize;

use super::{get_font_sizes, ThemeColors, ThemeNeutral, ThemeVars};

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct ThemeAlias {
    // theme/themes/shared/genSizeMapToken.ts
    pub size_xxl: f64,
    pub size_xl: f64,
    pub size_lg: f64,
    pub size_md: f64,
    pub size_ms: f64,
    pub size: f64,
    pub size_sm: f64,
    pub size_xs: f64,
    pub size_xxs: f64,

    // theme/themes/shared/genControlHeight.ts
    pub control_height_lg: f64,
    pub control_height_sm: f64,
    pub control_height_xs: f64,

    // Text -- theme/interface/alias.ts
    pub color_text_placeholder: String,
    pub color_text_disabled: String,
    pub color_text_heading: String,
    pub color_text_label: String,
    pub color_text_description: String,
    pub color_text_light_solid: String,

    // Control
    pub control_outline_width: f64,
    pub control_interactive_size: f64,
    pub control_item_bg_hover: String,
    pub control_item_bg_active: String,
    pub control_item_bg_active_hover: String,
    pub control_item_bg_active_disabled: String,

    // Padding Content
    pub padding_content_horizontal_lg: f64,
    pub padding_content_horizontal: f64,
    pub padding_content_horizontal_sm: f64,

    pub padding_content_vertical_lg: f64,
    pub padding_content_vertical: f64,
    pub padding_content_vertical_sm: f64,

    // font size -- themes/shared/genFontMapToken.ts
    pub font_size_sm: f64,
    pub font_size: f64,
    pub font_size_lg: f64,
    pub font_size_xl: f64,

    pub line_height_sm: f64,
    pub line_height: f64,
    pub line_height_lg: f64,
    pub line_height_xl: f64,

    pub font_size_heading1: f64,
    pub font_size_heading2: f64,
    pub font_size_heading3: f64,
    pub font_size_heading4: f64,
    pub font_size_heading5: f64,

    pub line_height_heading1: f64,
    pub line_height_heading2: f64,
    pub line_height_heading3: f64,
    pub line_height_heading4: f64,
    pub line_height_heading5: f64,

    // Motion -- theme/themes/seed.ts
    pub motion_unit: f64,
    pub motion_base: f64,
    pub motion_ease_out_circ: String,
    pub motion_ease_in_out_circ: String,
    pub motion_ease_out: String,
    pub motion_ease_in_out: String,
    pub motion_ease_out_back: String,
    pub motion_ease_in_back: String,
    pub motion_ease_in_quint: String,
    pub motion_ease_out_quint: String,

    // theme/themes/shared/genCommonMapToken.ts
    pub motion_duration_fast: String,
    pub motion_duration_mid: String,
    pub motion_duration_slow: String,
}

impl ThemeAlias {
    pub fn new(core: &ThemeVars, colors: &ThemeColors, neutral: &ThemeNeutral) -> Self {
        let mut alias = ThemeAlias { ..Default::default() };

        // theme/themes/shared/*
        alias.size_xxl = core.size_unit * (core.size_step * 8.0);
        alias.size_xl = core.size_unit * (core.size_step * 4.0);
        alias.size_lg = core.size_unit * (core.size_step * 2.0);
        alias.size_md = core.size_unit * (core.size_step * 1.0);
        alias.size_ms = core.size_unit * core.size_step;
        alias.size = core.size_unit * core.size_step;
        alias.size_sm = core.size_unit * (core.size_step * -1.0);
        alias.size_xs = core.size_unit * (core.size_step * -2.0);
        alias.size_xxs = core.size_unit * (core.size_step * -3.0);

        alias.control_height_lg = core.control_height * 1.25;
        alias.control_height_sm = core.control_height * 0.75;
        alias.control_height_xs = core.control_height * 0.5;

        // Text -- theme/interface/alias.ts
        alias.color_text_placeholder = neutral.color_text_quaternary.clone();
        alias.color_text_disabled = neutral.color_text_quaternary.clone();
        alias.color_text_heading = neutral.color_text.clone();
        alias.color_text_label = neutral.color_text_secondary.clone();
        alias.color_text_description = neutral.color_text_tertiary.clone();
        alias.color_text_light_solid = neutral.color_bg_base.clone();

        // Control
        alias.control_outline_width = core.line_width * 2.0;
        alias.control_interactive_size = core.control_height / 2.0;
        alias.control_item_bg_hover = neutral.color_fill_tertiary.clone();
        alias.control_item_bg_active = colors.color_primary_bg.clone();
        alias.control_item_bg_active_hover = colors.color_primary_bg_hover.clone();
        alias.control_item_bg_active_disabled = neutral.color_fill.clone();

        // Padding Content
        alias.padding_content_horizontal_lg = alias.size_lg;
        alias.padding_content_horizontal = alias.size_ms;
        alias.padding_content_horizontal_sm = alias.size;

        alias.padding_content_vertical_lg = alias.size_ms;
        alias.padding_content_vertical = alias.size_sm;
        alias.padding_content_vertical_sm = alias.size_xs;

        // font size
        let font_size_pairs = get_font_sizes(core.font_size);

        alias.font_size_sm = font_size_pairs[0].size;
        alias.font_size = font_size_pairs[1].size;
        alias.font_size_lg = font_size_pairs[2].size;
        alias.font_size_xl = font_size_pairs[3].size;

        alias.line_height_sm = font_size_pairs[0].line_height;
        alias.line_height = font_size_pairs[1].line_height;
        alias.line_height_lg = font_size_pairs[2].line_height;
        alias.line_height_xl = font_size_pairs[3].line_height;

        alias.font_size_heading1 = font_size_pairs[6].size;
        alias.font_size_heading2 = font_size_pairs[5].size;
        alias.font_size_heading3 = font_size_pairs[4].size;
        alias.font_size_heading4 = font_size_pairs[3].size;
        alias.font_size_heading5 = font_size_pairs[2].size;

        alias.line_height_heading1 = font_size_pairs[6].line_height;
        alias.line_height_heading2 = font_size_pairs[5].line_height;
        alias.line_height_heading3 = font_size_pairs[4].line_height;
        alias.line_height_heading4 = font_size_pairs[3].line_height;
        alias.line_height_heading5 = font_size_pairs[2].line_height;

        // Motion
        alias.motion_unit = 0.1;
        alias.motion_base = 0.0;
        alias.motion_ease_out_circ = "cubic-bezier(0.08, 0.82, 0.17, 1)".into();
        alias.motion_ease_in_out_circ = "cubic-bezier(0.78, 0.14, 0.15, 0.86)".into();
        alias.motion_ease_out = "cubic-bezier(0.215, 0.61, 0.355, 1)".into();
        alias.motion_ease_in_out = "cubic-bezier(0.645, 0.045, 0.355, 1)".into();
        alias.motion_ease_out_back = "cubic-bezier(0.12, 0.4, 0.29, 1.46)".into();
        alias.motion_ease_in_back = "cubic-bezier(0.71, -0.46, 0.88, 0.6)".into();
        alias.motion_ease_in_quint = "cubic-bezier(0.755, 0.05, 0.855, 0.06)".into();
        alias.motion_ease_out_quint = "cubic-bezier(0.23, 1, 0.32, 1)".into();

        alias.motion_duration_fast = format!("{}s", ((alias.motion_base + alias.motion_unit) * 10.0).round() / 10.0);
        alias.motion_duration_mid = format!("{}s", ((alias.motion_base + alias.motion_unit * 2.0) * 10.0).round() / 10.0);
        alias.motion_duration_slow = format!("{}s", ((alias.motion_base + alias.motion_unit * 3.0) * 10.0).round() / 10.0);

        alias
    }
}
