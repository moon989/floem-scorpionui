use floem::{
    views::{stack, v_stack, Decorators},
    View,
};
use scorpion_ui::widgets::button::{sbutton, ButtonProps};
use scorpion_ui::{
    theme_props::{SemColor, SemSize},
    widgets::{divider::sdivider, header::sheader},
};

fn btn_sem_color() -> impl View {
    v_stack((
        sheader("Sementic Color Buttons", None),
        sdivider(),
        stack((
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
        .style(|s| s.gap(4.0)),
    ))
}

fn btn_sizes() -> impl View {
    v_stack((
        sheader("Button sizes", None),
        sdivider(),
        stack((
            sbutton(
                || "Large",
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

fn btn_outlines() -> impl View {
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

pub fn buttons_view() -> impl View {
    v_stack( (
        btn_sem_color(),
        btn_sizes(),
        btn_outlines(), 
    ))
    .style(move |s| s.width_full().height_full())
}
