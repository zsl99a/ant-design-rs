use super::{get_font_sizes, ThemeCore};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ThemeAlias {
    // Text -- theme/interface/alias.ts
    pub color_text_placeholder: String,
    pub color_text_disabled: String,
    pub color_text_heading: String,
    pub color_text_label: String,
    pub color_text_description: String,
    pub color_text_light_solid: String,

    pub control_outline_width: f64,

    // font size -- themes/shared/genFontMapToken.ts
    pub font_size_sm: usize,
    pub font_size: usize,
    pub font_size_lg: usize,
    pub font_size_xl: usize,

    pub font_size_heading1: usize,
    pub font_size_heading2: usize,
    pub font_size_heading3: usize,
    pub font_size_heading4: usize,
    pub font_size_heading5: usize,

    pub line_height: f64,
    pub line_height_lg: f64,
    pub line_height_sm: f64,

    pub line_height_heading1: f64,
    pub line_height_heading2: f64,
    pub line_height_heading3: f64,
    pub line_height_heading4: f64,
    pub line_height_heading5: f64,
}

impl ThemeAlias {
    pub fn new(core: &ThemeCore) -> Self {
        let mut alias = ThemeAlias {
            control_outline_width: core.line_width * 2.0,
            ..Default::default()
        };

        // font size
        let font_size_pairs = get_font_sizes(core.font_size);

        alias.font_size_sm = font_size_pairs[0].size;
        alias.font_size = font_size_pairs[1].size;
        alias.font_size_lg = font_size_pairs[2].size;
        alias.font_size_xl = font_size_pairs[3].size;

        alias.font_size_heading1 = font_size_pairs[6].size;
        alias.font_size_heading2 = font_size_pairs[5].size;
        alias.font_size_heading3 = font_size_pairs[4].size;
        alias.font_size_heading4 = font_size_pairs[3].size;
        alias.font_size_heading5 = font_size_pairs[2].size;

        alias.line_height = font_size_pairs[1].line_height;
        alias.line_height_lg = font_size_pairs[2].line_height;
        alias.line_height_sm = font_size_pairs[0].line_height;

        alias.line_height_heading1 = font_size_pairs[6].line_height;
        alias.line_height_heading2 = font_size_pairs[5].line_height;
        alias.line_height_heading3 = font_size_pairs[4].line_height;
        alias.line_height_heading4 = font_size_pairs[3].line_height;
        alias.line_height_heading5 = font_size_pairs[2].line_height;

        alias
    }
}
