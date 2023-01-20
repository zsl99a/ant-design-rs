use std::ops::{Deref, DerefMut};

use super::{ThemeAlias, ThemeColors, ThemeCore, ThemePalettes};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ThemeStore {
    pub core: ThemeCore,
    pub colors: ThemeColors,
    pub palettes: ThemePalettes,
    pub alias: ThemeAlias,
}

impl Deref for ThemeStore {
    type Target = ThemeCore;

    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

impl DerefMut for ThemeStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.core
    }
}

impl ThemeStore {
    pub fn new() -> Self {
        // let preset = PresetColorPalettes::light();

        let core = ThemeCore::default();
        let colors = ThemeColors::new(&core);
        let palettes = ThemePalettes::default();
        let alias = ThemeAlias::new(&core, &colors, &palettes);
        Self { core, colors, palettes, alias }
    }
}
