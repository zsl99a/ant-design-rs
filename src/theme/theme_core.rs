// ThemeStore -- theme/themes/seed.ts
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeCore {
    // Color
    pub color_primary: String,
    pub color_success: String,
    pub color_warning: String,
    pub color_error: String,
    pub color_info: String,

    // Font
    pub font_family: String,
    pub font_size: usize,

    // Line
    pub line_width: f64,
    pub line_type: String,

    // Radius
    pub border_radius: usize,

    // Size
    pub size_unit: usize,
    pub size_step: usize,
    pub size_popup_arrow: usize,

    // Control Base
    pub control_height: usize,

    // zIndex
    pub z_index_base: usize,
    pub z_index_popup_base: usize,

    // Image
    pub opacity_image: f64,

    // Wireframe
    pub wireframe: bool,
}

impl Default for ThemeCore {
    fn default() -> Self {
        Self {
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
            font_size: 14,

            // Line
            line_width: 1.0,
            line_type: "solid".into(),

            // Radius
            border_radius: 6,

            // Size
            size_unit: 4,
            size_step: 4,
            size_popup_arrow: 16,

            // Control Base
            control_height: 32,

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
