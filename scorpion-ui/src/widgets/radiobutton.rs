use std::fmt::Display;

use floem::{
    peniko::Color,
    reactive::{ReadSignal, SignalGet},
    style::Style,
    unit::Pct,
    views::{self, container, empty, h_stack, Decorators},
    View,
};
use scorpion_ui_theme::{
    get_theme,
    theme_props::{SemColor, SemSize},
};

use super::common_style::build_b_f_h_d_style;

#[derive(Default, Clone, Copy)]
pub struct RadioProps {
    pub color: SemColor,
    pub outlined: bool,
    pub size: SemSize,
}

fn get_radio_style(
    props: RadioProps,
) -> (Box<dyn Fn(Style) -> Style>, Box<dyn Fn(Style) -> Style>) {
    let r_w_h = crate::BASIS_HEIGHT_UNIT;

    let inner_dot_style_creator = move |s: Style| {
        let theme = get_theme();
        let base_toggle_style =
            build_b_f_h_d_style(props.color, theme.clone())(s).border_radius(Pct(100.));

        let pick_size_style = |s: Style| match props.size {
            
            SemSize::Large => s.width(2.0 * r_w_h).height(2.0 * r_w_h),
            SemSize::Normal => s.width(1.5 * r_w_h).height(1.5 * r_w_h),
            SemSize::Small => s.width(1.0 * r_w_h).height(1.0 * r_w_h),
            SemSize::Tiny => s.width(0.8 * r_w_h).height(0.8 * r_w_h),
        };
        let radio_style = pick_size_style(base_toggle_style);
        radio_style
    };

    
    let outer_dot_style_creator = move |s: Style| {
        let theme = get_theme();
        let bg_sem_color = theme.bg_sem_color(props.color);
        let base_toggle_style = build_b_f_h_d_style(props.color, theme.clone())(s)
            .background(Color::TRANSPARENT)
            .border(1)
            .border_radius(Pct(100.))
            .justify_center()
            .items_center()
            .border_color(bg_sem_color)
            .active(|s| s.border_color(theme.active_sem_color(props.color)));

        let pick_size_style = |s: Style| match props.size {
            
            SemSize::Large => s.width(3.0 * r_w_h).height(3.0 * r_w_h),
            SemSize::Normal => s.width(2.0 * r_w_h).height(2.0 * r_w_h),
            SemSize::Small => s.width(1.5 * r_w_h).height(1.5 * r_w_h),
            SemSize::Tiny => s.width(1.0 * r_w_h).height(1.0 * r_w_h),
        };
        let radio_style = pick_size_style(base_toggle_style);
        radio_style
    };

    (
        Box::new(inner_dot_style_creator),
        Box::new(outer_dot_style_creator),
    )
}

fn radio_button_svg<T>(
    represented_value: T,
    actual_value: ReadSignal<T>,
    inner_style_enhancer: Box<dyn Fn(Style) -> Style>,
    outer_style_enhancer: Box<dyn Fn(Style) -> Style>,
) -> impl View
where
    T: Eq + PartialEq + Clone + 'static,
{
    container(empty().style(move |s| {
        inner_style_enhancer(s).apply_if(actual_value.get() != represented_value, |s| {
            s.display(floem::style::Display::None)
        })
    }))
    .style(move |s| outer_style_enhancer(s))
}

fn f_radio_button<T>(
    represented_value: T,
    actual_value: ReadSignal<T>,
    inner_style_enhancer: Box<dyn Fn(Style) -> Style>,
    outer_style_enhancer: Box<dyn Fn(Style) -> Style>,
) -> impl View
where
    T: Eq + PartialEq + Clone + 'static,
{
    radio_button_svg(
        represented_value,
        actual_value,
        inner_style_enhancer,
        outer_style_enhancer,
    )
    .keyboard_navigatable()
}

pub fn sradio_button<T>(
    represented_value: T,
    actual_value: ReadSignal<T>,
    props: Option<RadioProps>,
) -> impl View
where
    T: Eq + PartialEq + Clone + 'static,
{
    let props = props.unwrap_or(RadioProps::default());

    let (inner_style, outer_style) = get_radio_style(props);

    let base_widget = f_radio_button(represented_value, actual_value, inner_style, outer_style);

    base_widget
}

pub fn sradio_button_labeled<T, S: Display + 'static>(
    represented_value: T,
    actual_value: ReadSignal<T>,
    label: impl Fn() -> S + 'static,
    props: Option<RadioProps>,
) -> impl View
where
    T: Eq + PartialEq + Clone + 'static,
{
    h_stack((
        sradio_button(represented_value, actual_value, props),
        views::label(label),
    ))
    .style(|s| s.gap(8.))
}
