use std::fmt::Display;

use floem::{
    reactive::{create_signal, SignalUpdate},
    views::{h_stack, label, v_stack, Decorators},
    View,
};
use scorpion_ui::{
    theme_props::{SemColor, SemSize},
    widgets::{
        divider::sdivider,
        header::sheader,
        radiobutton::{sradio_button, sradio_button_labeled, RadioProps},
    },
};

#[derive(PartialEq, Eq, Clone)]
enum RgbSpace {
    Red,
    Green,
    Blue,
}

impl Display for RgbSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            RgbSpace::Red => write!(f, "Red"),
            RgbSpace::Green => write!(f, "Green"),
            RgbSpace::Blue => write!(f, "Blue"),
        }
    }
}

fn labeled_radio_color_with_state(sem_color: SemColor, size: SemSize) -> impl View {
    let (color, set_color) = create_signal(RgbSpace::Green);

    v_stack((
        sradio_button_labeled(
            RgbSpace::Red,
            color,
            || RgbSpace::Red,
            Some(RadioProps {
                color: sem_color,
                size,
                ..Default::default()
            }),
        )
        .on_click_stop(move |_| {
            set_color.set(RgbSpace::Red);
        }),
        sradio_button_labeled(
            RgbSpace::Green,
            color,
            || RgbSpace::Green,
            Some(RadioProps {
                color: sem_color,
                size,
                ..Default::default()
            }),
        )
        .on_click_stop(move |_| {
            set_color.set(RgbSpace::Green);
        }),
        sradio_button_labeled(
            RgbSpace::Blue,
            color,
            || RgbSpace::Blue,
            Some(RadioProps {
                color: sem_color,
                size,
                ..Default::default()
            }),
        )
        .on_click_stop(move |_| {
            set_color.set(RgbSpace::Blue);
        }),
    ))
}

fn radio_variant_with_state(sem_color: SemColor, size: SemSize) -> impl View {
    let (color, set_color) = create_signal(RgbSpace::Green);

    v_stack((
        sradio_button(
            RgbSpace::Red,
            color,
            Some(RadioProps {
                color: sem_color,
                size,
                ..Default::default()
            }),
        )
        .on_click_stop(move |_| {
            set_color.set(RgbSpace::Red);
        }),
        sradio_button(
            RgbSpace::Green,
            color,
            Some(RadioProps {
                color: sem_color,
                size,
                ..Default::default()
            }),
        )
        .on_click_stop(move |_| {
            set_color.set(RgbSpace::Green);
        }),
        sradio_button(
            RgbSpace::Blue,
            color,
            Some(RadioProps {
                color: sem_color,
                size,
                ..Default::default()
            }),
        )
        .on_click_stop(move |_| {
            set_color.set(RgbSpace::Blue);
        }),
    ))
}

fn radios_sem_color() -> impl View {
    v_stack((
        sheader("Radio button variants", None),
        sdivider(),
        h_stack((
            radio_variant_with_state(SemColor::Default, SemSize::Normal),
            radio_variant_with_state(SemColor::Neutral, SemSize::Normal),
            radio_variant_with_state(SemColor::Primary, SemSize::Normal),
            radio_variant_with_state(SemColor::Secondary, SemSize::Normal),
            radio_variant_with_state(SemColor::Accent, SemSize::Normal),
            radio_variant_with_state(SemColor::Ghost, SemSize::Normal),
            radio_variant_with_state(SemColor::Link, SemSize::Normal),
            radio_variant_with_state(SemColor::Info, SemSize::Normal),
            radio_variant_with_state(SemColor::Success, SemSize::Normal),
            radio_variant_with_state(SemColor::Warning, SemSize::Normal),
            radio_variant_with_state(SemColor::Error, SemSize::Normal),
        ))
        .style(|s| s.gap(10.)),
    ))
}
fn radio_sizes() -> impl View {
    v_stack((
        label(|| "Radio button sizes"),
        h_stack((
            radio_variant_with_state(SemColor::Primary, SemSize::Large),
            radio_variant_with_state(SemColor::Primary, SemSize::Normal),
            radio_variant_with_state(SemColor::Primary, SemSize::Small),
            radio_variant_with_state(SemColor::Primary, SemSize::Tiny),
        ))
        .style(|s| s.gap(10.)),
    ))
}
fn radio_labeled_sem_color() -> impl View {
    v_stack((
        sheader("Labeled radio buttons", None),
        sdivider(),
        h_stack((
            labeled_radio_color_with_state(SemColor::Primary, SemSize::Normal),
            labeled_radio_color_with_state(SemColor::Secondary, SemSize::Normal),
        ))
        .style(|s| s.gap(10.)),
    ))
}
fn radio_labeled_disabled_sem_color() -> impl View {
    v_stack((
        sheader("Disabled states", None),
        sdivider(),
        h_stack((
            labeled_radio_color_with_state(SemColor::Primary, SemSize::Normal).disabled(|| true),
            labeled_radio_color_with_state(SemColor::Secondary, SemSize::Normal).disabled(|| true),
        ))
        .style(|s| s.gap(10)),
    ))
}

pub fn radios_view() -> impl View {
    v_stack( (
        radios_sem_color(),
        radio_sizes(),
        radio_labeled_sem_color(),
        radio_labeled_disabled_sem_color(), 
    ))
    .style(move |s| s.width_full().height_full())
}
