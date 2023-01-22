use std::ops::{Deref, DerefMut};

use serde::Serialize;
use serde_json::{json, Value};

use super::{theme_dark, theme_light, ThemeAlias, ThemeColors, ThemeNeutral, ThemeVars};

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct ThemeStore {
    #[serde(flatten)]
    pub vars: ThemeVars,
    #[serde(flatten)]
    pub colors: ThemeColors,
    #[serde(flatten)]
    pub neutral: ThemeNeutral,
    #[serde(flatten)]
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

impl ThemeStore {
    pub fn export(&self) -> String {
        let mut vars = String::new();
        if let Value::Object(list) = json!(self) {
            for (key, value) in list {
                match value {
                    Value::Bool(value) => {
                        vars.push_str(&format!("--{key}:{value};"));
                    }
                    Value::Number(value) => {
                        vars.push_str(&format!("--{key}:{value};"));
                    }
                    Value::String(value) => {
                        vars.push_str(&format!("--{key}:{value};"));
                    }
                    Value::Null => todo!(),
                    _ => {}
                }
            }
        }
        format!(":root{{{vars}}}")
    }
}

#[test]
fn it_works() {
    let store = ThemeStore::new(ThemeVars::default());
    store.export();
}
