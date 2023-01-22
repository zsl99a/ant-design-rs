use stylist::{ast::Sheet, GlobalStyle, StyleSource};
use yew::hook;

mod color_algorithm;
mod get_font_sizes;
mod preset_color;
mod theme_alias;
mod theme_colors;
mod theme_neutral;
mod theme_store;
mod theme_vars;

pub use {color_algorithm::*, get_font_sizes::*, preset_color::*, theme_alias::*, theme_colors::*, theme_neutral::*, theme_store::*, theme_vars::*};

#[hook]
pub fn use_styled(source: &'static str) -> GlobalStyle {
    let sheet: Sheet = source.parse().expect("Failed to parse stylesheet");
    let style = StyleSource::from(sheet);
    let style = GlobalStyle::new(style).unwrap();
    style
}
