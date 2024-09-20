use floem::{
    style::Style,
    views::{self, Decorators},
    View,
};

use scorpion_ui_theme::theme_props::SemSize;

#[derive(Default, Copy, Clone)]
pub struct LabelProps {
    pub size: SemSize,
}

pub fn get_header_style(props: LabelProps) -> Box<dyn Fn(Style) -> Style> {
    let style_creator = move |s: Style| {
        let pick_size_style = |s: Style| match props.size {
            SemSize::Large => s.font_size(2.25 * 16.).font_bold(),
            SemSize::Normal => s.font_size(1.5 * 16.).font_bold(),
            SemSize::Small => s.font_size(1.25 * 16.).font_bold(),
            SemSize::Tiny => s.font_bold(),
        };
        let header_style = pick_size_style(s);
        header_style
    };
    Box::new(style_creator)
}

pub fn sheader(text: &str, props: Option<LabelProps>) -> impl View {
    let base_widget = views::text(text);
    let props = props.unwrap_or(LabelProps::default());
    let styled_header = base_widget.style(move |s| get_header_style(props)(s));
    styled_header
}
