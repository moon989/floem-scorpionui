use floem::{
    style::Style,
    views::{self, container, Decorators},
    View,
};
use scorpion_ui_theme::{
    get_theme,
    theme_props::{SemColor, SemSize},
};

#[derive(Default, Clone, Copy)]
pub struct TooltipProps {
    pub color: SemColor,
    pub size: SemSize,
}

fn get_tooltip_style(props: TooltipProps) -> Box<dyn Fn(Style) -> Style> {
    let theme = get_theme();
    let bg_sem_color = theme.bg_sem_color(props.color);
    let styles_creator = move |s: Style| {
        let base_style = s
            .padding(8.)
            .background(bg_sem_color)
            .color(theme.content_sem_color(props.color));

        let pick_size_style = |s: Style| {
            let font_size = theme.font_size(props.size);
            s.font_size(font_size)
        };
        let tooltip_style = pick_size_style(base_style);
        tooltip_style
    };

    Box::new(styles_creator)
}

pub fn stooltip<V: View + 'static, T: View + 'static>(
    child: V,
    tip: impl Fn() -> T + 'static,
    props: Option<TooltipProps>,
) -> impl View {
    let props = props.unwrap_or(TooltipProps::default());
    let base_widget = views::tooltip(child, move || {
        container(tip().style(move |s| get_tooltip_style(props)(s)))
    });
    base_widget
}
