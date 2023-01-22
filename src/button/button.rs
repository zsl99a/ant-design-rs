use yew::{prelude::*, Classes};

use super::ButtonType;
use crate::{button::use_styled, theme::ControlSize};

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
    let styled = use_styled(props);

    let html_type = match &props.html_type {
        Some(html_type) => html_type.clone(),
        None => "button".into(),
    };

    let disabled = match &props.disabled {
        Some(disabled) => *disabled,
        None => false,
    };

    html! {
        <button
            class={classes!(
                styled,
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
