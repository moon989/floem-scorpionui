use floem::{
    peniko::Color,
    style::Style,
    views::{label as flabel, Decorators},
    View,
};

use scorpion_ui_theme::get_theme;
use scorpion_ui_theme::theme_props::{SemColor, SemSize};

#[derive(Default, Copy, Clone)]
pub struct BadgeProps {
    pub color: SemColor,
    pub size: SemSize,
    pub outlined: bool,
}


const B_H: f32 = crate::BASIS_HEIGHT_UNIT;

fn get_badge_style(props: BadgeProps) -> Box<dyn Fn(Style) -> Style> {
    let theme = get_theme();
    let bg_sem_color = theme.bg_sem_color(props.color);

    let styles_creator = move |s: Style| {
        let base_style = s
            .padding_vert(3.)
            .padding_horiz(9.)
            .font_size(13.)
            .border_radius(14.)
            .items_center()
            .justify_center()
            .height(2.0 * B_H)
            .background(bg_sem_color)
            .color(theme.base_content_color());

        let pick_size_style = |s: Style| match props.size {
            SemSize::Large => s.font_size(16.).height(2.5 * B_H),
            SemSize::Normal => s,
            SemSize::Small => s.font_size(10.).height(1.5 * B_H),
            SemSize::Tiny => s.font_size(8.).height(1.0 * B_H),
        };

        let pick_outlined_style = |s: Style| match props.outlined {
            true => s
                .border_color(bg_sem_color)
                .border(0.5)
                .background(Color::TRANSPARENT)
                .color(theme.base_content_color()),
            false => s,
        };

        let badge_style = pick_size_style(base_style);
        let badge_style = pick_outlined_style(badge_style);

        badge_style
    };

    Box::new(styles_creator)
}

pub fn sbadge(label: &'static str, props: Option<BadgeProps>) -> impl View {
    let base_widget = flabel(move || label);
    let props = props.unwrap_or(BadgeProps::default());
    let styled_badge = base_widget.style(move |s| get_badge_style(props)(s));
    styled_badge
}
