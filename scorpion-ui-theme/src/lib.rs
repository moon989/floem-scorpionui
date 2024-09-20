pub mod colors;
pub mod theme;
pub(crate) mod theme_config;
pub mod theme_props;

use std::{cell::RefCell, sync::Arc};

use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use theme::Theme;

use anyhow::{anyhow, Result};
use theme_config::ThemeConfigHex;



thread_local! {
    pub(crate) static RUNTIME_THEME: RuntimeTheme = RuntimeTheme::new();
}

pub(crate) struct RuntimeTheme {
    pub(crate) current_them: RwSignal<Arc<Theme>>,
    pub(crate) themes: RefCell<Vec<Arc<Theme>>>,
}
impl Default for RuntimeTheme {
    fn default() -> Self {
        Self::new()
    }
}
impl RuntimeTheme {
    pub(crate) fn new() -> Self {
        let theme = Arc::new(Theme::default());
        Self {
            current_them: RwSignal::new(theme.clone()),
            themes: RefCell::new(vec![theme.clone()]),
        }
    }

    pub(crate) fn append_theme(&self, theme: Arc<Theme>) {
        self.themes.borrow_mut().push(theme)
    }

    pub(crate) fn get_current_them(&self) -> Arc<Theme> {
        let theme = self.current_them.get();
        theme.clone()
    }

    pub(crate) fn get_theme_names(&self) -> Vec<(usize, String)> {
        let mut res = Vec::<(usize, String)>::new();
        for (index, item) in self.themes.borrow().iter().enumerate() {
            if index > 0 {
                res.push((index, item.name.clone()))
            }
        }
        res
    }

    pub(crate) fn change_current_theme(&self, index: usize) -> Result<()> {
        let binding = self.themes.borrow();
        let current_theme = binding.get(index);

        if let Some(theme) = current_theme {
            let theme = theme.clone();
            self.current_them.update(|t| *t = theme);
            Ok(())
        } else {
            Err(anyhow!("index out of themes array length"))
        }
    }
}

pub fn get_theme() -> Arc<Theme> {
    RUNTIME_THEME.with(|runtime_theme| runtime_theme.get_current_them())
}




pub fn change_theme(index: usize) -> Result<()> {
    RUNTIME_THEME.with(|runtime_theme| runtime_theme.change_current_theme(index))
}


pub fn get_theme_name_list() -> Vec<(usize, String)> {
    RUNTIME_THEME.with(|runtime_theme| runtime_theme.get_theme_names())
}



pub fn regist_themes(str_themes: &str) -> Result<()> {
    let res = ThemeConfigHex::load(str_themes);
    match res {
        Ok(res) => {
            let default = res.default;
            for (index, themhex) in res.theme.iter().enumerate() {
                let theme = Theme::new(themhex)?;
                let name = theme.name.clone();
                append_theme(theme);
                if default.eq(&name) {
                    let _ = change_theme(index + 1);
                }
            }
            Ok(())
        }
        Err(err) => Err(anyhow!("{}", err)),
    }
}

pub fn append_theme(theme: Theme) {
    RUNTIME_THEME.with(|runtime_theme| runtime_theme.append_theme(Arc::new(theme)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_content_color() {
        let theme = get_theme();
        let c1 = theme.base_content_color();
        let c2 = theme.error_content_color();
        let c3 = theme.info_content_color();
        println!(
            "back ground is {:?}, base content color: {:?}",
            theme.color.background, c1
        );

        println!(
            "err background is {:?}, error content color: {:?}",
            theme.color.error, c2
        );
        println!(
            "info background is {:?}, info content color: {:?}",
            theme.color.info, c3
        );
    }

    #[test]
    fn test_resist_them() {
        let theme_str = r##"
            default = "Light Theme"

            [[theme]]
            name ="Light Theme"

            [theme.base]
            black   = "#383A42"
            blue    = "#4078F2"
            cyan    = "#0184BC"
            green   = "#50A14F"
            gray    = "#E5E5E6"
            magenta = "#A626A4"
            orange  = "#D19A66"
            purple  = "#A626A4"
            red     = "#E45649"
            white   = "#FAFAFA"
            yellow  = "#C18401"

            [theme.color]
            background   = "black"
            default   = "#C18401"
            neutral   = "#C18401"
            primary   = "#C18401"
            secondary = "#C18401"
            accent    = "#C18401"
            ghost     = "#C18401"
            link      = "#C18401"

            info   = "#C18401"
            success= "#C18401"
            warning= "#C18401"
            error  = "#C18401"        
        "##;

        let res = regist_themes(theme_str);

        if let Ok(()) = res {
            let theme = get_theme();
            println!("current them is : {} ", theme.name);
            let themes = get_theme_name_list();
            for theme in themes {
                println!("theme index is : {}    name is : {} ", theme.0, theme.1);
            }
        } else {
            println!("error")
        }

        let _ = change_theme(0);
        let theme = get_theme();
        println!("current them is : {} ", theme.name);

        let res = change_theme(2);
        match res {
            Ok(_) => {
                let theme = get_theme();
                println!("current them is : {} ", theme.name);
            }
            Err(e) => {
                println!("error {}", e)
            }
        }
    }
}
