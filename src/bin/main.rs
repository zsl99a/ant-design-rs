use stylist::css;
use yew::prelude::*;

use ant_design_rs::{button::ButtonType, theme::generate, Button, ConfigProvider};

#[function_component(Root)]
fn app() -> Html {
    let colors = generate("#13C2C2", false, "").unwrap();
    let dark_colors = generate("#13C2C2", true, "").unwrap();

    html! {
        <ConfigProvider>
            <Button r#type={ButtonType::Default}>{"Default"}</Button>
            <Button r#type={ButtonType::Primary}>{"Primary"}</Button>
            <Button r#type={ButtonType::Ghost}>{"Ghost"}</Button>
            <Button r#type={ButtonType::Dashed}>{"Dashed"}</Button>
            <Button r#type={ButtonType::Link}>{"Link"}</Button>
            <Button r#type={ButtonType::Text}>{"Text"}</Button>

            {colors.iter().map(|color| {
                html!{<div key={color.clone()} class={classes!(css!("background: ${color}; padding: 10px;", color = color))}>{ color }</div>}
            }).collect::<Html>()}

            <div class={classes!(css!("height: 10px;"))}></div>

            {dark_colors.iter().map(|color| {
                html!{<div key={color.clone()} class={classes!(css!("background: ${color}; padding: 10px;", color = color))}>{ color }</div>}
            }).collect::<Html>()}
        </ConfigProvider>
    }
}

fn main() {
    yew::Renderer::<Root>::new().render();
}
