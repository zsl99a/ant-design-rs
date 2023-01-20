use std::ops::Deref;

use stylist::yew::styled_component;
use yew::{hook, html, use_context, use_state, Children, ContextProvider, Html, Properties, UseStateHandle};

use crate::theme::{theme_light, ThemeStore};

#[derive(Debug, PartialEq, Properties)]
pub struct ConfigProviderProps {
    pub children: Children,
}

#[styled_component]
pub fn ConfigProvider(props: &ConfigProviderProps) -> Html {
    let theme_store = use_state(|| {
        let mut theme = ThemeStore::new();
        theme_light(&mut theme.palettes);
        theme
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
