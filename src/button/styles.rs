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

    let style_base = use_style!(
        r#"
        outline: none;
        position: relative;
        display: inline-block;
        font-weight: 400;
        white-space: nowrap;
        text-align: center;
        background-image: none;
        background-color: transparent;
        border: ${line_width}px hidden ${color_border};
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
        "#,
        color_text = theme.neutral.color_text,
        font_size = theme.font_size,
        control_height = theme.control_height,
        border_radius = theme.border_radius,
        // border
        line_width = theme.line_width,
        color_border = theme.neutral.color_border,
        // motion
        motion_duration_mid = theme.alias.motion_duration_mid,
        motion_ease_in_out = theme.alias.motion_ease_in_out,
        // alias
        line_height = theme.alias.line_height,
        // vars
        padding_vertical = padding_vertical,
        padding_horizontal = padding_horizontal,
    );

    let style_default = use_style!(
        r#"
        border-color: ${color_border};
        border-style: ${line_type};
        background-color: ${color_bg_container};
        box-shadow: 0 ${control_outline_width}px 0 ${color_fill_quaternary};
        "#,
        color_border = theme.neutral.color_border,
        line_type = theme.line_type,
        color_bg_container = theme.neutral.color_bg_container,
        color_fill_quaternary = theme.neutral.color_fill_quaternary,
        //
        control_outline_width = theme.alias.control_outline_width,
    );
    let style_primary = use_style!(
        r#"
        color: ${color_white};
        border-style: ${line_type};
        border-color: ${color_primary};
        background-color: ${color_primary};

        &:hover {
            border-color: ${color_primary_hover};
            background-color: ${color_primary_hover};
        }
        &:active {
            border-color: ${color_primary_active};
            background-color: ${color_primary_active};
        }
        "#,
        line_type = theme.line_type,
        color_white = theme.colors.color_white,
        color_primary = theme.color_primary,
        color_primary_hover = theme.colors.color_primary_hover,
        color_primary_active = theme.colors.color_primary_active,
    );
    let style_dashed = use_style!(
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
    let style_link = use_style!(
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
    let style_text = use_style!(
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

    let style_hover = use_style!(
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
        ButtonType::Default => classes!(style_base, style_hover, style_default),
        ButtonType::Primary => classes!(style_base, style_primary),
        ButtonType::Dashed => classes!(style_base, style_hover, style_dashed),
        ButtonType::Link => classes!(style_base, style_link),
        ButtonType::Text => classes!(style_base, style_text),
    };

    // ghost

    let style_ghost = use_style!(
        r#"
        color: ${color_bg_base};
        background-color: transparent;
        box-shadow: none;
        "#,
        color_bg_base = theme.neutral.color_bg_base,
    );

    let style_ghost_border = use_style!(
        r#"
        border-color: ${color_bg_base};
        "#,
        color_bg_base = theme.neutral.color_bg_base,
    );

    let style_ghost_hover = use_style!(
        r#"
        &:hover {
            color: ${color_primary_hover};
            border-color: ${color_primary_hover};
            background-color: transparent;
        }
        "#,
        color_primary_hover = theme.colors.color_primary_hover,
    );

    let style_ghost_active = use_style!(
        r#"
        &:active {
            color: ${color_primary_active};
            border-color: ${color_primary_active};
        }
        "#,
        color_primary_active = theme.colors.color_primary_active,
    );

    let style_ghost_primary = use_style!(
        r#"
        color: ${color_primary};
        border-color: ${color_primary};

        "#,
        color_primary = theme.colors.color_primary,
    );

    let style_ghost_link = use_style!(
        r#"
        color: ${color_primary};
        "#,
        color_primary = theme.color_primary,
    );

    if let Some(ghost) = &props.ghost {
        if *ghost {
            class.push(match ctype {
                ButtonType::Default => classes!(style_ghost, style_ghost_hover, style_ghost_active, style_ghost_border),
                ButtonType::Primary => classes!(style_ghost, style_ghost_hover, style_ghost_active, style_ghost_primary),
                ButtonType::Dashed => classes!(style_ghost, style_ghost_hover, style_ghost_active, style_ghost_border),
                ButtonType::Link => classes!(style_ghost, style_ghost_link),
                ButtonType::Text => classes!(style_ghost),
            });
        }
    }

    // danger

    let style_danger = use_style!(
        r#"
        color: ${color_error};
        border-color: ${color_error};
        "#,
        color_error = theme.colors.color_error,
    );

    let style_danger_hover = use_style!(
        r#"
        &:hover {
            color: ${color_error_hover};
        }
        &:active {
            color: ${color_error_active};
        }
        "#,
        color_error_hover = theme.colors.color_error_hover,
        color_error_active = theme.colors.color_error_active,
    );

    let style_danger_border = use_style!(
        r#"
        &:hover {
            border-color: ${color_error_hover};
        }
        &:active {
            border-color: ${color_error_active};
        }
        "#,
        color_error_hover = theme.colors.color_error_hover,
        color_error_active = theme.colors.color_error_active,
    );

    let style_danger_bg_color = use_style!(
        r#"
        &:hover {
            background-color: ${color_error_hover};
        }
        &:active {
            background-color: ${color_error_active};
        }
        "#,
        color_error_hover = theme.colors.color_error_hover,
        color_error_active = theme.colors.color_error_active,
    );

    let style_danger_primary = use_style!(
        r#"
        color: ${color_white};
        background-color: ${color_error};
        "#,
        color_white = theme.colors.color_white,
        color_error = theme.colors.color_error,
    );

    let style_danger_text = use_style!(
        r#"
        &:hover {
            background-color: ${color_error_bg_hover};
        }
        "#,
        color_error_bg_hover = theme.colors.color_error_bg_hover,
    );

    if let Some(danger) = &props.danger {
        if *danger {
            class.push(match ctype {
                ButtonType::Default => classes!(style_danger, style_danger_hover, style_danger_border),
                ButtonType::Primary => classes!(style_danger, style_danger_border, style_danger_bg_color, style_danger_primary),
                ButtonType::Dashed => classes!(style_danger, style_danger_hover, style_danger_border),
                ButtonType::Link => classes!(style_danger, style_danger_hover),
                ButtonType::Text => classes!(style_danger, style_danger_hover, style_danger_text),
            });
        }
    }

    // disabled

    let style_disabled = use_style!(
        r#"
        &:disabled {
            cursor: not-allowed;
            color: ${color_text_disabled};
            background-color: transparent;
        }
        "#,
        color_text_disabled = theme.alias.color_text_disabled,
    );

    let style_disabled_bg_and_border = use_style!(
        r#"
        &:disabled {
            border-color: ${color_fill};
            background-color: ${color_fill_tertiary};
        }
        "#,
        color_fill = theme.neutral.color_fill,
        color_fill_tertiary = theme.neutral.color_fill_tertiary,
    );

    if let Some(disabled) = &props.disabled {
        if *disabled {
            class.push(match ctype {
                ButtonType::Default => classes!(style_disabled, style_disabled_bg_and_border),
                ButtonType::Primary => classes!(style_disabled, style_disabled_bg_and_border),
                ButtonType::Dashed => classes!(style_disabled, style_disabled_bg_and_border),
                ButtonType::Link => classes!(style_disabled),
                ButtonType::Text => classes!(style_disabled),
            });
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
