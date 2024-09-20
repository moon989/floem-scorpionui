use std::collections::HashMap;

use anyhow::{anyhow, Result};
use floem::peniko::Color;
use serde::{Deserialize, Serialize};

use crate::theme::{SizeConfig, ThemeColor, ThemeLayout};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct ThemeConfigHex {
    pub default: String,
    pub theme: Vec<ThemeHex>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct ThemeHex {
    pub name: String,
    pub disabledopacity: f32,
    pub dividerweight: f32,
    pub color: HashMap<String, String>,
    pub fontsize: HashMap<String, f32>,
    pub radius: HashMap<String, f32>,
    pub borderwidth: HashMap<String, f32>,
}

impl ThemeHex {
    pub fn build_theme_layout(&self) -> ThemeLayout {
        let tiny = *self.fontsize.get("tiny").unwrap_or(&12.);
        let small = *self.fontsize.get("small").unwrap_or(&14.);
        let normal = *self.fontsize.get("normal").unwrap_or(&16.);
        let large = *self.fontsize.get("large").unwrap_or(&20.);
        let fontsize = SizeConfig {
            tiny,
            small,
            normal,
            large,
        };

        let tiny = *self.radius.get("tiny").unwrap_or(&4.);
        let small = *self.radius.get("small").unwrap_or(&8.);
        let normal = *self.radius.get("normal").unwrap_or(&12.);
        let large = *self.radius.get("large").unwrap_or(&14.);
        let radius = SizeConfig {
            tiny,
            small,
            normal,
            large,
        };

        let tiny = *self.borderwidth.get("tiny").unwrap_or(&0.5);
        let small = *self.borderwidth.get("small").unwrap_or(&1.);
        let normal = *self.borderwidth.get("normal").unwrap_or(&2.);
        let large = *self.borderwidth.get("large").unwrap_or(&3.);

        let borderwidth = SizeConfig {
            tiny,
            small,
            normal,
            large,
        };

        ThemeLayout {
            fontsize,
            radius,
            borderwidth,
        }
    }
    pub fn build_theme_color(&self) -> Result<ThemeColor> {
        let mut is_ok = true;
        let mut err_msg = String::new();
        let background = self.color.get("background");
        let foreground = self.color.get("foreground");
        let divider = self.color.get("divider");
        let focus = self.color.get("focus");
        let overlay = self.color.get("overlay");

        let default = self.color.get("default");
        let neutral = self.color.get("neutral");
        let primary = self.color.get("primary");
        let secondary = self.color.get("secondary");
        let accent = self.color.get("accent");
        let ghost = self.color.get("ghost");
        let link = self.color.get("link");

        let info = self.color.get("info");
        let error = self.color.get("error");
        let success = self.color.get("success");
        let warning = self.color.get("warning");

        if background.is_none() {
            is_ok = false;
            err_msg.push_str("The 'background' configuration item does not exist\n");
        }
        if default.is_none() {
            is_ok = false;
            err_msg.push_str("The 'default' configuration item does not exist\n");
        }
        if neutral.is_none() {
            is_ok = false;
            err_msg.push_str("The 'neutral' configuration item does not exist\n");
        }
        if primary.is_none() {
            is_ok = false;
            err_msg.push_str("The 'primary' configuration item does not exist\n");
        }
        if secondary.is_none() {
            is_ok = false;
            err_msg.push_str("The 'secondary' configuration item does not exist\n");
        }
        if accent.is_none() {
            is_ok = false;
            err_msg.push_str("The 'accent' configuration item does not exist\n");
        }
        if ghost.is_none() {
            is_ok = false;
            err_msg.push_str("The 'ghost' configuration item does not exist\n");
        }
        if link.is_none() {
            is_ok = false;
            err_msg.push_str("The 'link' configuration item does not exist\n");
        }
        if info.is_none() {
            is_ok = false;
            err_msg.push_str("The 'info' configuration item does not exist\n");
        }
        if success.is_none() {
            is_ok = false;
            err_msg.push_str("The 'success' configuration item does not exist\n");
        }
        if warning.is_none() {
            is_ok = false;
            err_msg.push_str("The 'warning' configuration item does not exist\n");
        }
        if error.is_none() {
            is_ok = false;
            err_msg.push_str("The 'error' configuration item does not exist\n");
        };

        if !is_ok {
            return Err(anyhow!("{}", err_msg));
        };

        let background = Color::parse(&background.unwrap()).unwrap();

        let foreground = Color::parse(&foreground.unwrap_or(&"FFFFFF".to_string())).unwrap();
        let divider = Color::parse(&divider.unwrap_or(&"".to_string())).take();
        let focus = Color::parse(&focus.unwrap_or(&"".to_string())).take();
        let overlay = Color::parse(&overlay.unwrap_or(&"".to_string())).take();

        let default = Color::parse(&default.unwrap()).unwrap();
        let neutral = Color::parse(&neutral.unwrap()).unwrap();
        let primary = Color::parse(&primary.unwrap()).unwrap();
        let secondary = Color::parse(&secondary.unwrap()).unwrap();
        let accent = Color::parse(&accent.unwrap()).unwrap();
        let ghost = Color::TRANSPARENT;
        let link = Color::parse(&link.unwrap()).unwrap();

        let info = Color::parse(&info.unwrap()).unwrap();
        let success = Color::parse(&success.unwrap()).unwrap();
        let warning = Color::parse(&warning.unwrap()).unwrap();
        let error = Color::parse(&error.unwrap()).unwrap();

        let res = ThemeColor {
            background,
            foreground,
            divider,
            focus,
            overlay,

            default,
            neutral,
            primary,
            secondary,
            accent,
            ghost,
            link,

            info,
            success,
            warning,
            error,
        };

        Ok(res)
    }
}

impl ThemeConfigHex {
    pub fn load(source: &str) -> Result<Self> {
        let me = toml::from_str(source);
        match me {
            Ok(t) => Ok(t),
            Err(e) => Err(anyhow!("Theme config is wrong. {}", e)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::theme::Theme;
    const THEMS_STR: &str = include_str!("../Themes.toml");

    #[test]
    fn test_load() {
        let theme = ThemeConfigHex::load(THEMS_STR).unwrap();
        println!("Test theme");
        println!("default:{}", theme.default);

        let my_them = theme.theme.get(0).unwrap();
        let my_them = Theme::new(my_them);
        match my_them {
            Ok(theme) => {
                println!("{}", theme.name);
                println!("{:?}", theme.color.primary);
            }
            Err(err) => {
                println!("load theme is wrong:  {}", err);
            }
        }
    }
}
