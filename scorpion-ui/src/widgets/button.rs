use std::fmt::Display;


use floem::peniko::Color;
use floem::style::{Background, Style, StyleValue, Transition};
use floem::text::Weight;
use floem::{views::button as fbutton, views::Decorators, View};

use scorpion_ui_theme::get_theme;
use scorpion_ui_theme::theme_props::{SemColor, SemSize};

use super::common_style::build_b_f_h_d_style;

#[derive(Default, Clone, Copy)]
pub struct ButtonProps {
    pub color: SemColor,
    pub outlined: bool,
    pub size: SemSize,
}

const BTN_B_H: f32 = crate::BASIS_HEIGHT_UNIT;
const BTN_B_W: f32 = crate::BASIS_WIDTH_UNIT;
fn get_button_style(props: ButtonProps) -> Box<dyn Fn(Style) -> Style> {
    let theme = get_theme();
    let style_creator = move |s: Style| {
        let base_button_style = {
            build_b_f_h_d_style(props.color, theme.clone())(s)
                .font_size(theme.font_size(props.size))
                .line_height(1.)
                .font_weight(Weight::SEMIBOLD)
                .transition(Background, Transition::linear(0.04))
                .border_radius(5.0)
                .justify_center()
                .items_center()
                .cursor(StyleValue::Val(floem::style::CursorStyle::Pointer))
        };

        let pick_size_style = match props.size {
            SemSize::Large => |s: Style| {
                s.min_height(4.0 * BTN_B_H)
                    .height(4.0 * BTN_B_H)
                    .min_width(4. * BTN_B_W)
                    .padding_horiz(1.5 * BTN_B_H)
            },
            SemSize::Normal => |s: Style| {
                s.min_height(3.0 * BTN_B_H)
                    .height(3. * BTN_B_H)
                    .min_width(3. * BTN_B_W)
                    .padding_horiz(1.5 * BTN_B_H)
            },
            SemSize::Small => |s: Style| {
                s.min_height(2. * BTN_B_H)
                    .height(2. * BTN_B_H)
                    .min_width(2. * BTN_B_W)
                    .padding_horiz(0.75 * BTN_B_H)
            },
            SemSize::Tiny => |s: Style| {
                s.min_height(1.5 * BTN_B_H)
                    .height(1.5 * BTN_B_H)
                    .min_width(1.5 * BTN_B_W)
                    .padding_horiz(0.5 * 16.)
                    .padding_vert(0)
            },
        };

        let bg_sem_color = theme.bg_sem_color(props.color);
        let button_style = pick_size_style(base_button_style);
        
        let button_style = if props.outlined {
            let outline_style = button_style
                .outline(0.5)
                .outline_color(bg_sem_color)
                .background(Color::TRANSPARENT);
            match props.color {
                SemColor::Default => outline_style,
                SemColor::Neutral => outline_style,
                SemColor::Ghost => outline_style,
                _ => outline_style
                    .color(bg_sem_color)
                    .hover(|s| s.color(theme.base_content_color())),
            }
        } else {
            button_style
        };

        button_style
    };
    Box::new(style_creator)
}


pub fn sbutton<S: Display + 'static>(
    label: impl Fn() -> S + 'static,
    props: Option<ButtonProps>,
) -> impl View {
    let base_widget = fbutton(label);
    let props = props.unwrap_or(ButtonProps::default());
    let styled_button = base_widget.style(move |s| get_button_style(props)(s));
    styled_button
}
