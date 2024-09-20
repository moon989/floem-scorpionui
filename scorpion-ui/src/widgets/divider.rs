use floem::{
    peniko::Color,
    style::Style,
    taffy::Display,
    views::{empty, Decorators},
    View,
};

use crate::get_theme;

pub fn get_divider_style() -> Box<dyn Fn(Style) -> Style> {
    let theme = get_theme().clone();
    let gray = theme.color.divider.unwrap_or(Color::GRAY);
    let styles_creator = move |s: Style| {
        s.width_full()
            .min_width_full()
            .height(1)
            .background(gray)
            .display(Display::Flex)
            .flex_grow(1.)
            .margin_vert(10.)
    };

    Box::new(styles_creator)
}

pub fn sdivider() -> impl View {
    let base_widget = empty();
    let styled_divider = base_widget.style(move |s| get_divider_style()(s));
    styled_divider
}
