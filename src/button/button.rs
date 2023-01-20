use stylist::yew::styled_component;
use yew::{prelude::*, Classes};

use super::ButtonType;
use crate::button::use_styled;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub r#type: Option<ButtonType>,
    pub class: Option<Classes>,
    pub children: Children,
}

#[styled_component]
pub fn Button(props: &Props) -> Html {
    let styled = use_styled();

    html! {
        <button
            class={classes!(
                styled,
                props.r#type.clone(),
                props.class.clone(),
            )}
            type="button"
        >
            <span>{for props.children.iter()}</span>
        </button>
    }
}
