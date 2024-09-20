use std::fmt::Display;

use floem::{
    reactive::{ReadSignal, SignalGet},
    style::{Background, Style, Transition},
    views::{self, h_stack, svg, Decorators},
    View,
};

use crate::get_theme;

use scorpion_ui_theme::theme_props::{SemColor, SemSize};

use super::common_style::build_b_f_h_d_style;

#[derive(Default, Clone, Copy)]
pub struct CheckboxProps {
    pub color: SemColor,
    pub size: SemSize,
}

fn get_checkbox_style(props: CheckboxProps) -> Box<dyn Fn(Style) -> Style> {
    let theme = get_theme();
    let w_h = crate::BASIS_HEIGHT_UNIT;

    let style_creator = move |s: Style| {
        let base_checkbox_style = build_b_f_h_d_style(props.color, theme.clone())(s)
            .transition(Background, Transition::linear(0.04))
            .border_radius(8);

        let pick_size_style = |s: Style| match props.size {
            SemSize::Large => s.width(4. * w_h).height(4. * w_h),
            SemSize::Normal => s.width(2.6 * w_h).height(2.6 * w_h),
            SemSize::Small => s.width(2.0 * w_h).height(2.0 * w_h),
            SemSize::Tiny => s.width(1. * w_h).height(1. * w_h),
        };

        let checkbox_style = pick_size_style(base_checkbox_style);
        checkbox_style
    };
    Box::new(style_creator)
}


fn checkbox_svg(checked: ReadSignal<bool>, props: Option<CheckboxProps>) -> impl View {
    const CHECKBOX_SVG: &str =  r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="-2 -2 16 16"><polygon points="5.19,11.83 0.18,7.44 1.82,5.56 4.81,8.17 10,1.25 12,2.75" /></svg>"#;
    let svg_str = move || if checked.get() { CHECKBOX_SVG } else { "" }.to_string();
    let base_widget = svg(svg_str);

    let props = props.unwrap_or(CheckboxProps::default());
    let styled_checkbox = base_widget.style(move |s| get_checkbox_style(props)(s));
    styled_checkbox
}

pub fn scheckbox(checked: ReadSignal<bool>, props: Option<CheckboxProps>) -> impl View {
    checkbox_svg(checked, props).keyboard_navigatable()
}

pub fn checkbox_labeled<S: Display + 'static>(
    checked: ReadSignal<bool>,
    label: impl Fn() -> S + 'static,
    props: Option<CheckboxProps>,
) -> impl View {
    h_stack((checkbox_svg(checked, props), views::label(label)))
        .style(|s| s.gap(8.))
        .keyboard_navigatable()
}
