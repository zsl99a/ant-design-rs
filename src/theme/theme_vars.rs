use serde::Serialize;

// ThemeStore -- theme/themes/seed.ts
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ThemeVars {
    pub dark: bool,

    // Color
    pub color_primary: String,
    pub color_success: String,
    pub color_warning: String,
    pub color_error: String,
    pub color_info: String,

    // Font
    pub font_family: String,
    pub font_size: f64,

    // Line
    pub line_width: f64,
    pub line_type: String,

    // Radius
    pub border_radius: f64,

    // Size
    pub size_unit: f64,
    pub size_step: f64,

    // Control Base
    pub control_height: f64,

    // zIndex
    pub z_index_base: usize,
    pub z_index_popup_base: usize,

    // Image
    pub opacity_image: f64,

    // Wireframe
    pub wireframe: bool,
}

impl Default for ThemeVars {
    fn default() -> Self {
        Self {
            dark: false,

            // Color
            color_primary: "#1677ff".into(),
            color_success: "#52c41a".into(),
            color_warning: "#faad14".into(),
            color_error: "#ff4d4f".into(),
            color_info: "#1677ff".into(),

            // Font
            font_family: r#"-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans',
sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'"#
                .into(),
            font_size: 14.0,

            // Line
            line_width: 1.0,
            line_type: "solid".into(),

            // Radius
            border_radius: 6.0,

            // Size
            size_unit: 4.0,
            size_step: 4.0,

            // Control Base
            control_height: 32.0,

            // zIndex
            z_index_base: 0,
            z_index_popup_base: 1000,

            // Image
            opacity_image: 1.0,

            // Wireframe
            wireframe: false,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum ControlSize {
    #[default]
    Default,
    Large,
    Small,
}
