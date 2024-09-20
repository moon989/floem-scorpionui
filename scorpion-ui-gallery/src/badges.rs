use floem::{
    style::Style,
    views::{h_stack, v_stack, Decorators},
    View,
};
use scorpion_ui::widgets::badge::{sbadge, BadgeProps};
use scorpion_ui::{
    theme_props::{SemColor, SemSize},
    widgets::{divider::sdivider, header::sheader},
};

fn sem_color_badge(name: &'static str, color: SemColor) -> impl View {
    sbadge(
        name,
        Some(BadgeProps {
            color,
            ..Default::default()
        }),
    )
}

fn badges_sem_color() -> impl View {
    v_stack((
        sheader("Sementic Color", None),
        sdivider(),
        h_stack((
            sem_color_badge("default", SemColor::Default),
            sem_color_badge("neutral", SemColor::Neutral),
            sem_color_badge("primary", SemColor::Primary),
            sem_color_badge("secondary", SemColor::Secondary),
            sem_color_badge("accent", SemColor::Accent),
            sem_color_badge("ghost", SemColor::Ghost),
            sem_color_badge("link", SemColor::Link),
            sem_color_badge("info", SemColor::Info),
            sem_color_badge("success", SemColor::Success),
            sem_color_badge("warning", SemColor::Warning),
            sem_color_badge("error", SemColor::Error),
        ))
        .style(|s: Style| s.gap(4.)),
    ))
}

fn size_badge(name: &'static str, size: SemSize) -> impl View {
    sbadge(
        name,
        Some(BadgeProps {
            size,
            ..Default::default()
        }),
    )
}

fn badges_sizes() -> impl View {
    v_stack((
        sheader("Size badge", None),
        sdivider(),
        h_stack((
            size_badge("large", SemSize::Large),
            size_badge("normal", SemSize::Normal),
            size_badge("small", SemSize::Small),
            size_badge("tiny", SemSize::Tiny),
        ))
        .style(|s: Style| s.gap(4.)),
    ))
}

fn badges_outlines() -> impl View {
    let badges = h_stack((
        sbadge(
            "outlined default",
            Some(BadgeProps {
                outlined: true,
                ..Default::default()
            }),
        ),
        sbadge(
            "outlined primary",
            Some(BadgeProps {
                outlined: true,
                color: SemColor::Primary,
                ..Default::default()
            }),
        ),
        sbadge(
            "outlined secondary",
            Some(BadgeProps {
                outlined: true,
                color: SemColor::Secondary,
                ..Default::default()
            }),
        ),
    ))
    .style(|s: Style| s.gap(4.));

    v_stack((sheader("Out Lined badges", None), sdivider(), badges))
}

pub fn badges_view() -> impl View {
    v_stack((badges_sem_color(), badges_sizes(), badges_outlines())).style(|s| s.gap(5.))
}
