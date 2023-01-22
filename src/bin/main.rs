use stylist::css;
use yew::prelude::*;

use ant_design_rs::{button::ButtonType, theme::generate, Button, ConfigProvider};

#[function_component(Root)]
fn root() -> Html {
    let colors = generate("#13C2C2", false, "").unwrap();
    let dark_colors = generate("#13C2C2", true, "").unwrap();

    html! {
        <ConfigProvider>
            <div class={classes!(css!("padding: 8px; background-color: rgb(190, 200, 200);"))}>
                <Button rtype={ButtonType::Default}>{"Default Button"}</Button>
                <Button rtype={ButtonType::Primary}>{"Primary Button"}</Button>
                <Button rtype={ButtonType::Dashed}>{"Dashed Button"}</Button>
                <Button rtype={ButtonType::Link}>{"Link Button"}</Button>
                <Button rtype={ButtonType::Text}>{"Text Button"}</Button>
            </div>

            <div class={classes!(css!("height: 8px;"))}></div>

            <div class={classes!(css!("padding: 8px; background-color: rgb(190, 200, 200);"))}>
                <Button rtype={ButtonType::Default} ghost={true}>{"Default Button"}</Button>
                <Button rtype={ButtonType::Primary} ghost={true}>{"Primary Button"}</Button>
                <Button rtype={ButtonType::Dashed} ghost={true}>{"Dashed Button"}</Button>
                <Button rtype={ButtonType::Link} ghost={true}>{"Link Button"}</Button>
                <Button rtype={ButtonType::Text} ghost={true}>{"Text Button"}</Button>
            </div>

            <div class={classes!(css!("height: 8px;"))}></div>

            {colors.iter().map(|color| {
                html!{
                    <div
                        key={color.clone()}
                        class={classes!(css!("background: ${color}; padding: 8px;", color = color))}
                    >
                        { color }
                    </div>
                }
            }).collect::<Html>()}

            <div class={classes!(css!("height: 8px;"))}></div>

            {dark_colors.iter().map(|color| {
                html!{
                    <div
                        key={color.clone()}
                        class={classes!(css!("background: ${color}; padding: 8px;", color = color))}
                    >
                        { color }
                    </div>
                }
            }).collect::<Html>()}
        </ConfigProvider>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<Root>::new().render();
}
