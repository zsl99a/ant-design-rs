use stylist::css;
use yew::prelude::*;

use crate::config_provider::use_theme;

#[hook]
pub fn use_styled() -> Classes {
    let theme = use_theme();

    let padding_vertical = 0.0_f64.max((theme.control_height - theme.font_size * theme.alias.line_height) / 2.0 - theme.line_width);
    let padding_horizontal = 8.0 - theme.line_width;

    let style = css!(
        r#"
& {
    outline: none;
    position: relative;
    display: inline-block;
    font-weight: 400;
    white-space: nowrap;
    text-align: center;
    background-image: none;
    background-color: transparent;
    border: ${line_width}px ${line_type} transparent;
    cursor: pointer;
    user-select: none;
    touch-action: manipulation;
    line-height: ${line_height};
    color: ${color_primary};

    padding: ${padding_vertical}px ${padding_horizontal}px;
    border-radius: ${border_radius}px;
}

&:disabled {
    cursor: not-allowed;
}

&.ant-btn-default {
    border-color: ${color_border};
    background-color: ${color_bg_container};
    box-shadow: 0 ${control_outline_width}px 0 ${color_fill_quaternary};

}

&.ant-btn-primary {
    background-color: ${color_primary};
    color: ${color_bg_base};
}

&.ant-btn-ghost {
    color: ${color_text};
    background-color: transparent;
    border-color: borderColor;
    box-shadow: none;
}

&.ant-btn-dashed {
    border-style: dashed;
}

        "#,
        color_primary = theme.color_primary,
        line_width = theme.line_width,
        line_type = theme.line_type,
        color_bg_base = theme.palettes.color_bg_base,
        color_text = theme.palettes.color_text,
        color_border = theme.palettes.color_border,
        color_bg_container = theme.palettes.color_bg_container,
        color_fill_quaternary = theme.palettes.color_fill_quaternary,
        control_outline_width = theme.alias.control_outline_width,
        line_height = theme.alias.line_height,
        padding_vertical = padding_vertical,
        padding_horizontal = padding_horizontal,
        border_radius = theme.border_radius,
    );

    classes!(style)
}

/// ButtonType

#[derive(Debug, Default, Clone, PartialEq)]
pub enum ButtonType {
    #[default]
    Default,
    Primary,
    Ghost,
    Dashed,
    Link,
    Text,
}

impl From<ButtonType> for Classes {
    fn from(value: ButtonType) -> Self {
        classes!(format!("ant-btn-{value:?}").to_lowercase())
    }
}
