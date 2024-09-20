#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod app;
pub mod badges;
pub mod buttons;
pub mod checkboxs;
pub mod edits;
pub mod radiobuttons;
pub mod toggles;
pub mod tooltips;


pub mod combox_test;

use floem::{kurbo::Size, reactive::Scope, views::Decorators, window::WindowConfig, View};
use scorpion_ui::{regist_themes, widgets::button::sbutton};

const THEMS_STR: &str = include_str!("../themes/themes.toml");
const LOGO_ICON: &str = include_str!("./../../icons/xwork/Logo.svg");

fn my_title() -> impl View {
    sbutton(|| "test", None)
}

pub fn main() {
    let res = regist_themes(THEMS_STR);
    if let Err(er) = res {
        panic!(" Error loading theme file {}", er);
    }

    let window_config = WindowConfig::default()
        .size(Size {
            width: 1200.0,
            height: 800.0,
        })
        .show_titlebar(false)
        .apply_default_theme(false);

    
    let root_view = app::app_view();
    let root_view = root_view.style(|s| s.width_full().height_full());
    let title = my_title();

    let cx = Scope::new();
    let logo = Some(LOGO_ICON);
    let customer_window_bar = cx.create_rw_signal(true);
    let window_size = cx.create_rw_signal(Size::new(1200., 800.));
    let app =
        scorpion_ui::widgets::application::SApplicaton::new(cx, logo, customer_window_bar, 50.);
    let app = app.create_app_with_customer_bar(
        move |_| root_view,
        title,
        Some(window_config),
        window_size,
        |_| true,
    );
    app.run();
}
