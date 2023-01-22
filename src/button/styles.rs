use stylist::yew::use_style;
use yew::prelude::*;

use super::ButtonProps;
use crate::config_provider::use_theme;

#[hook]
pub fn use_styled(props: &ButtonProps) -> Classes {
    let ctype = match &props.rtype {
        Some(ctype) => ctype.clone(),
        _ => ButtonType::Default,
    };

    let theme = use_theme();

    let padding_vertical = ((theme.control_height - theme.font_size * theme.alias.line_height) / 2.0 - theme.line_width).max(0.0);
    let padding_horizontal = theme.alias.size - theme.line_width;

    let style = use_style!(
        r#"
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
        transition: all ${motion_duration_mid} ${motion_ease_in_out};
        user-select: none;
        touch-action: manipulation;
        line-height: ${line_height};
        color: ${color_text};

        padding: ${padding_vertical}px ${padding_horizontal}px;
        border-radius: ${border_radius}px;

        font-size: ${font_size}px;
        height: ${control_height}px;

        &:disabled {
            cursor: not-allowed;
        }
        "#,
        color_text = theme.neutral.color_text,
        line_width = theme.line_width,
        line_type = theme.line_type,
        font_size = theme.font_size,
        control_height = theme.control_height,
        border_radius = theme.border_radius,
        // motion
        motion_duration_mid = theme.alias.motion_duration_mid,
        motion_ease_in_out = theme.alias.motion_ease_in_out,
        // alias
        line_height = theme.alias.line_height,
        // vars
        padding_vertical = padding_vertical,
        padding_horizontal = padding_horizontal,
    );

    let default_style = use_style!(
        r#"
        border-color: ${color_border};
        background-color: ${color_bg_container};
        box-shadow: 0 ${control_outline_width}px 0 ${color_fill_quaternary};
        "#,
        color_border = theme.neutral.color_border,
        color_bg_container = theme.neutral.color_bg_container,
        color_fill_quaternary = theme.neutral.color_fill_quaternary,
        //
        control_outline_width = theme.alias.control_outline_width,
    );
    let primary_style = use_style!(
        r#"
        color: ${color_white};
        background-color: ${color_primary};

        &:hover {
            color: ${color_white};
            background-color: ${color_primary_hover};
        }
        &:active {
            background-color: ${color_primary_active};
        }
        "#,
        color_white = theme.colors.color_white,
        color_primary = theme.color_primary,
        color_primary_hover = theme.colors.color_primary_hover,
        color_primary_active = theme.colors.color_primary_active,
    );
    let dashed_style = use_style!(
        r#"
        color: ${color_text};
        border-color: ${color_border};
        background-color: ${color_bg_container};
        border-style: dashed;
        "#,
        color_text = theme.neutral.color_text,
        color_border = theme.neutral.color_border,
        color_bg_container = theme.neutral.color_bg_container,
    );
    let link_style = use_style!(
        r#"
        color: ${color_primary};
        
        &:hover {
            color: ${color_info_hover};
        }
        &:active {
            color: ${color_info_active};
        }
        "#,
        color_primary = theme.color_primary,
        color_info_hover = theme.colors.color_info_hover,
        color_info_active = theme.colors.color_info_active,
    );
    let text_style = use_style!(
        r#"
        color: ${color_text};

        &:hover {
            background-color: ${color_fill_secondary};
        }
        &:active {
            background-color: ${color_fill};
        }
        "#,
        color_text = theme.neutral.color_text,
        color_fill_secondary = theme.neutral.color_fill_secondary,
        color_fill = theme.neutral.color_fill,
    );

    let hover_style = use_style!(
        r#"
        &:hover {
            color: ${color_primary};
            border-color: ${color_primary};
        }
        &:active {
            color: ${color_primary_active};
            border-color: ${color_primary_active};
        }
        "#,
        color_primary = theme.colors.color_primary,
        color_primary_active = theme.colors.color_primary_active
    );

    let mut class = match ctype {
        ButtonType::Default => classes!(style, hover_style, default_style.clone()),
        ButtonType::Primary => classes!(style, primary_style.clone()),
        ButtonType::Dashed => classes!(style, hover_style, dashed_style.clone()),
        ButtonType::Link => classes!(style, link_style),
        ButtonType::Text => classes!(style, text_style),
    };

    let ghost_style = use_style!(
        r#"
        color: ${color_bg_base};
        background-color: transparent;
        box-shadow: none;

        &.${default_style}, &.${dashed_style} {
            border-color: ${color_bg_base};
        }

        &.${primary_style} {
            color: ${color_primary};
            border-color: ${color_primary};
        }

        &.${default_style}:hover, &.${primary_style}:hover, &.${dashed_style}:hover {
            color: ${color_primary_hover};
            border-color: ${color_primary_hover};
            background-color: transparent;
        }
        
        &.${default_style}:active, &.${primary_style}:active, &.${dashed_style}:active {
            color: ${color_primary_active};
            border-color: ${color_primary_active};
        }
        "#,
        color_bg_base = theme.neutral.color_bg_base,
        primary_style = primary_style.get_class_name(),
        default_style = default_style.get_class_name(),
        dashed_style = dashed_style.get_class_name(),
        color_primary = theme.colors.color_primary,
        color_primary_hover = theme.colors.color_primary_hover,
        color_primary_active = theme.colors.color_primary_active,
    );

    if let Some(ghost) = &props.ghost {
        if *ghost {
            class.push(ghost_style);
        }
    }

    class
}

/// ButtonType

#[derive(Debug, Default, Clone, PartialEq)]
pub enum ButtonType {
    #[default]
    Default,
    Primary,
    Dashed,
    Link,
    Text,
}

impl From<ButtonType> for Classes {
    fn from(value: ButtonType) -> Self {
        classes!(format!("ant-btn-{value:?}").to_lowercase())
    }
}
