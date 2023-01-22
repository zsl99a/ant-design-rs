use std::ops::{Deref, DerefMut};

use super::{theme_dark, theme_light, ThemeAlias, ThemeColors, ThemeNeutral, ThemeVars};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ThemeStore {
    pub vars: ThemeVars,
    pub colors: ThemeColors,
    pub neutral: ThemeNeutral,
    pub alias: ThemeAlias,
}

impl Deref for ThemeStore {
    type Target = ThemeVars;

    fn deref(&self) -> &Self::Target {
        &self.vars
    }
}

impl DerefMut for ThemeStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vars
    }
}

impl ThemeStore {
    pub fn new(vars: ThemeVars) -> Self {
        let colors = ThemeColors::new(&vars);

        let mut neutral = ThemeNeutral::default();
        if vars.dark {
            theme_dark(&mut neutral);
        } else {
            theme_light(&mut neutral);
        }

        let alias = ThemeAlias::new(&vars, &colors, &neutral);

        Self { vars, colors, neutral, alias }
    }
}
