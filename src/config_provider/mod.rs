use std::ops::Deref;

use yew::prelude::*;

use crate::theme::{ThemeStore, ThemeVars};

#[derive(Debug, PartialEq, Properties)]
pub struct ConfigProviderProps {
    pub children: Children,
}

#[function_component(ConfigProvider)]
pub fn config_provider(props: &ConfigProviderProps) -> Html {
    let theme_store = use_state(|| {
        let store = ThemeStore::new(ThemeVars::default());
        log::debug!("{store:#?}");
        store
    });

    let theme_ctx = ThemeContext::new(theme_store);

    html! {
        <ContextProvider<ThemeContext> context={theme_ctx}>
            {props.children.clone()}
        </ContextProvider<ThemeContext>>
    }
}

//

#[hook]
pub(crate) fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().unwrap()
}

//

#[derive(Debug, Clone)]
pub(crate) struct ThemeContext {
    inner: UseStateHandle<ThemeStore>,
}

impl ThemeContext {
    pub fn new(inner: UseStateHandle<ThemeStore>) -> Self {
        Self { inner }
    }
}

impl Deref for ThemeContext {
    type Target = ThemeStore;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl PartialEq for ThemeContext {
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}
