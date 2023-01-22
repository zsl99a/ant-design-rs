use std::rc::Rc;

use stylist::{ast::Sheet, StyleSource};
use yew::prelude::*;

use crate::theme::{ThemeStore, ThemeVars};

#[derive(Debug, PartialEq, Properties)]
pub struct ConfigProviderProps {
    pub children: Children,
}

#[function_component(ConfigProvider)]
pub fn config_provider(props: &ConfigProviderProps) -> Html {
    let theme_store = use_memo(|_| ThemeStore::new(ThemeVars::default()), ());

    html! {
        <ContextProvider<Rc<ThemeStore>> context={theme_store}>
            {props.children.clone()}
        </ContextProvider<Rc<ThemeStore>>>
    }
}

#[hook]
pub fn use_theme_vars() -> Rc<ThemeStore> {
    use_context::<Rc<ThemeStore>>().unwrap()
}

#[hook]
pub fn use_theme_export() -> StyleSource {
    let store = use_theme_vars();
    let export = store.export();
    let sheet: Sheet = export.parse().expect("Failed to parse stylesheet");
    StyleSource::from(sheet)
}
