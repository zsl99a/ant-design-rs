use yew::prelude::*;

use ant_design_rs::{button::ButtonType, Button, ConfigProvider};

#[function_component(Root)]
fn app() -> Html {
    html! {
        <ConfigProvider>
            <Button r#type={ButtonType::Default}>{"Default"}</Button>
            <Button r#type={ButtonType::Primary}>{"Primary"}</Button>
            <Button r#type={ButtonType::Ghost}>{"Ghost"}</Button>
            <Button r#type={ButtonType::Dashed}>{"Dashed"}</Button>
            <Button r#type={ButtonType::Link}>{"Link"}</Button>
            <Button r#type={ButtonType::Text}>{"Text"}</Button>
        </ConfigProvider>
    }
}

fn main() {
    yew::Renderer::<Root>::new().render();
}
