#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use floem::{
    kurbo::Size,
    views::{stack, v_stack, Decorators},
    window::WindowConfig,
    Application, View,
};
use scorpion_ui::widgets::button::{sbutton, ButtonProps};
use scorpion_ui::{
    get_theme,
    theme_props::{SemColor, SemSize},
    widgets::{divider::sdivider, header::sheader},
};

pub fn btn_variants() -> impl View {
    v_stack((stack((
        sbutton(|| "Default", None),
        sbutton(
            || "Neutral",
            Some(ButtonProps {
                color: SemColor::Neutral,
                ..Default::default()
            }),
        ),
        sbutton(
            || "Primary",
            Some(ButtonProps {
                color: SemColor::Primary,
                ..Default::default()
            }),
        ),
        sbutton(
            || "Secondary",
            Some(ButtonProps {
                color: SemColor::Secondary,
                ..Default::default()
            }),
        ),
        sbutton(
            || "Accent",
            Some(ButtonProps {
                color: SemColor::Accent,
                ..Default::default()
            }),
        ),
        sbutton(
            || "Ghost",
            Some(ButtonProps {
                color: SemColor::Ghost,
                ..Default::default()
            }),
        ),
        sbutton(
            || "Link",
            Some(ButtonProps {
                color: SemColor::Link,
                ..Default::default()
            }),
        ),
        sbutton(
            || "Info",
            Some(ButtonProps {
                color: SemColor::Info,
                ..Default::default()
            }),
        ),
        sbutton(
            || "Success",
            Some(ButtonProps {
                color: SemColor::Success,
                ..Default::default()
            }),
        ),
        sbutton(
            || "Warning",
            Some(ButtonProps {
                color: SemColor::Warning,
                ..Default::default()
            }),
        ),
        sbutton(
            || "Error",
            Some(ButtonProps {
                color: SemColor::Error,
                ..Default::default()
            }),
        ),
    ))
    .style(|s| s.gap(4.0)),))
}

pub fn btn_sizes() -> impl View {
    v_stack((
        sheader("Button sizes", None),
        sdivider(),
        stack((
            sbutton(
                || "超大按钮",
                Some(ButtonProps {
                    size: SemSize::Large,
                    ..Default::default()
                }),
            ),
            sbutton(
                || "Normal",
                Some(ButtonProps {
                    size: SemSize::Normal,
                    ..Default::default()
                }),
            ),
            sbutton(
                || "Small",
                Some(ButtonProps {
                    size: SemSize::Small,
                    ..Default::default()
                }),
            ),
            sbutton(
                || "Tiny",
                Some(ButtonProps {
                    size: SemSize::Tiny,
                    ..Default::default()
                }),
            ),
        ))
        .style(|s| s.gap(4.0)),
    ))
}

pub fn btn_outlines() -> impl View {
    v_stack((
        sheader("Outlined buttons", None),
        sdivider(),
        stack((
            sbutton(
                || "Info",
                Some(ButtonProps {
                    color: SemColor::Info,
                    outlined: true,
                    ..Default::default()
                }),
            ),
            sbutton(
                || "Success",
                Some(ButtonProps {
                    color: SemColor::Success,
                    outlined: true,
                    ..Default::default()
                }),
            ),
            sbutton(
                || "Warning",
                Some(ButtonProps {
                    color: SemColor::Warning,
                    outlined: true,
                    ..Default::default()
                }),
            ),
            sbutton(
                || "Error",
                Some(ButtonProps {
                    color: SemColor::Error,
                    outlined: true,
                    ..Default::default()
                }),
            ),
        ))
        .style(|s| s.gap(4.0)),
    ))
}

fn app_view() -> impl View {
    v_stack((btn_variants(), btn_sizes(), btn_outlines()))
}

pub fn main() {
    let theme = get_theme();
    let window_config = WindowConfig::default()
        .size(Size {
            width: 1200.0,
            height: 800.0,
        })
        .apply_default_theme(false);

    let root_view = app_view();
    let root_view = root_view.style(move |s| {
        s.width_full()
            .background(theme.color.background)
            .color(theme.base_content_color())
            .padding(16.)
            .height_full()
    });

    let app = Application::new().window(move |_| root_view, Some(window_config));
    app.run();
}
