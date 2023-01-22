use stylist::css;
use yew::prelude::*;

use ant_design_rs::{button::ButtonType, config_provider::use_theme, theme::generate, Button, ConfigProvider};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<Root>::new().render();
}

#[function_component(Root)]
fn root() -> Html {
    html! {
        <ConfigProvider>
            <App />
        </ConfigProvider>
    }
}

#[function_component(App)]
fn app() -> Html {
    let theme = use_theme();

    let colors = generate("#13C2C2", false, "").unwrap();
    let dark_colors = generate("#13C2C2", true, "").unwrap();

    html!(
        <>
            <div class={classes!(css!("padding: 8px; background-color: ${color_bg_base};", color_bg_base = theme.neutral.color_bg_base))}>
                <Button rtype={ButtonType::Default}>{"Default Button"}</Button>
                <Button rtype={ButtonType::Primary}>{"Primary Button"}</Button>
                <Button rtype={ButtonType::Dashed}>{"Dashed Button"}</Button>
                <Button rtype={ButtonType::Link}>{"Link Button"}</Button>
                <Button rtype={ButtonType::Text}>{"Text Button"}</Button>
            </div>

            <div class={classes!(css!("height: 8px;"))}></div>

            <div class={classes!(css!("padding: 8px; background-color: ${color_bg_spotlight};", color_bg_spotlight = theme.neutral.color_bg_spotlight))}>
                <Button rtype={ButtonType::Default} ghost={true}>{"Default Button"}</Button>
                <Button rtype={ButtonType::Primary} ghost={true}>{"Primary Button"}</Button>
                <Button rtype={ButtonType::Dashed} ghost={true}>{"Dashed Button"}</Button>
                <Button rtype={ButtonType::Link} ghost={true}>{"Link Button"}</Button>
                <Button rtype={ButtonType::Text} ghost={true}>{"Text Button"}</Button>
            </div>

            <div class={classes!(css!("height: 8px;"))}></div>

            <div class={classes!(css!("padding: 8px; background-color: ${color_bg_base};", color_bg_base = theme.neutral.color_bg_base))}>
                <Button rtype={ButtonType::Default} danger={true}>{"Default Button"}</Button>
                <Button rtype={ButtonType::Primary} danger={true}>{"Primary Button"}</Button>
                <Button rtype={ButtonType::Dashed} danger={true}>{"Dashed Button"}</Button>
                <Button rtype={ButtonType::Link} danger={true}>{"Link Button"}</Button>
                <Button rtype={ButtonType::Text} danger={true}>{"Text Button"}</Button>
            </div>

            <div class={classes!(css!("height: 8px;"))}></div>

            <div class={classes!(css!("padding: 8px; background-color: ${color_bg_base};", color_bg_base = theme.neutral.color_bg_base))}>
                <Button rtype={ButtonType::Default} disabled={true}>{"Default Button"}</Button>
                <Button rtype={ButtonType::Primary} disabled={true}>{"Primary Button"}</Button>
                <Button rtype={ButtonType::Dashed} disabled={true}>{"Dashed Button"}</Button>
                <Button rtype={ButtonType::Link} disabled={true}>{"Link Button"}</Button>
                <Button rtype={ButtonType::Text} disabled={true}>{"Text Button"}</Button>
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
        </>
    )
}
