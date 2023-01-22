use stylist::yew::use_style;
use yew::{prelude::*, Classes};

use crate::{
    config_provider::use_theme_vars,
    theme::{use_styled, ControlSize},
};

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    pub block: Option<bool>,
    pub danger: Option<bool>,
    pub disabled: Option<bool>,
    pub ghost: Option<bool>,

    pub icon: Option<Children>,
    pub loading: Option<bool>,

    pub size: Option<ControlSize>,

    pub href: Option<String>,
    pub target: Option<String>,

    pub html_type: Option<String>,
    pub rtype: Option<ButtonType>,

    pub class: Option<Classes>,
    pub children: Children,
    // pub onclick: Option<Rc<dyn IntoEventCallback<MouseEvent>>>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let theme = use_theme_vars();
    use_styled(include_str!("styles.css"));

    let rtype = match &props.rtype {
        Some(rtype) => rtype.clone(),
        None => ButtonType::Default,
    };

    let ghost = match &props.ghost {
        Some(true) => Some("a-btn-ghost"),
        _ => None,
    };

    let danger = match &props.danger {
        Some(true) => Some("a-btn-danger"),
        _ => None,
    };

    let disabled = match &props.disabled {
        Some(disabled) => *disabled,
        None => false,
    };

    let html_type = match &props.html_type {
        Some(html_type) => html_type.clone(),
        None => "button".into(),
    };

    let padding_vertical = ((theme.control_height - theme.font_size * theme.alias.line_height) / 2.0 - theme.line_width).max(0.0);
    let padding_horizontal = theme.alias.size - theme.line_width;

    let inline_vars = use_style!(
        r#"
        --padding_vertical: ${padding_vertical}; 
        --padding_horizontal: ${padding_horizontal};
        "#,
        padding_vertical = padding_vertical,
        padding_horizontal = padding_horizontal,
    );

    html! {
        <button

            class={classes!(
                inline_vars,
                "a-btn",
                ghost,
                danger,
                Classes::from(rtype),
                props.class.clone(),
            )}
            type={html_type}
            disabled={disabled}
            // onclick={props.onclick}
        >
            <span>{for props.children.iter()}</span>
        </button>
    }
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
        classes!(format!("a-btn-{value:?}").to_lowercase())
    }
}
