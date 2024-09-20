use floem::{
    style::Style,
    views::{label, static_label, v_stack, Decorators},
    View,
};
use scorpion_ui::{
    theme_props::{SemColor, SemSize},
    widgets::{
        button::sbutton,
        tooltip::{stooltip, TooltipProps},
    },
};

fn button_tooltips() -> impl View {
    stooltip(
        sbutton(|| "I'm a button(hover over me!)", None),
        || static_label("And I'm the button tooltip!"),
        None,
    )
}

fn label_tooltips() -> impl View {
    v_stack((
        stooltip(
            label(|| "Default label tooltip(hover on me!)"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                color: SemColor::Default,
                ..Default::default()
            }),
        ),
        stooltip(
            label(|| "Neutral label tooltip(hover on me!)"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                color: SemColor::Neutral,
                ..Default::default()
            }),
        ),
        stooltip(
            label(|| "Primary label tooltip(hover on me!)"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                color: SemColor::Primary,
                ..Default::default()
            }),
        ),
        stooltip(
            label(|| "Secondary label tooltip(hover on me!)"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                color: SemColor::Secondary,
                ..Default::default()
            }),
        ),
        stooltip(
            label(|| "Accent label tooltip(hover on me!)"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                color: SemColor::Accent,
                ..Default::default()
            }),
        ),
        stooltip(
            label(|| "Ghost label tooltip(hover on me!)"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                color: SemColor::Ghost,
                ..Default::default()
            }),
        ),
        stooltip(
            label(|| "Link label tooltip(hover on me!)"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                color: SemColor::Link,
                ..Default::default()
            }),
        ),
        stooltip(
            label(|| "Info label tooltip(hover on me!)"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                color: SemColor::Info,
                ..Default::default()
            }),
        ),
        stooltip(
            label(|| "Success label tooltip(hover on me!)"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                color: SemColor::Success,
                ..Default::default()
            }),
        ),
        stooltip(
            label(|| "Warning label tooltip(hover on me!)"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                color: SemColor::Warning,
                ..Default::default()
            }),
        ),
        stooltip(
            label(|| "Error label tooltip(hover on me!)"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                color: SemColor::Error,
                ..Default::default()
            }),
        ),
    ))
    .style(|s: Style| s.gap(10.))
}

fn sizes_tooltips() -> impl View {
    v_stack((
        stooltip(
            label(|| "Large tooltip"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                size: SemSize::Large,
                ..Default::default()
            }),
        ),
        stooltip(
            label(|| "Normal tooltip"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                size: SemSize::Normal,
                ..Default::default()
            }),
        ),
        stooltip(
            label(|| "Small tooltip"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                size: SemSize::Small,
                ..Default::default()
            }),
        ),
        stooltip(
            label(|| "Tiny tooltip"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                size: SemSize::Tiny,
                ..Default::default()
            }),
        ),
    ))
    .style(|s| s.gap(10.))
}

pub fn tooltips_view() -> impl View {
    v_stack( (
        button_tooltips(),
        label_tooltips(),
        sizes_tooltips(), 
    ))
    .style(move |s| s.width_full().height_full())
}
