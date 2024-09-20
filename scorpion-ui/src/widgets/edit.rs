use floem::{
    reactive::RwSignal,
    style::{CursorStyle, Style},
    views::{text_input as f_text_input, Decorators, TextInput},
};
use scorpion_ui_theme::{
    get_theme,
    theme_props::{SemColor, SemSize},
};

use super::common_style::{build_b_f_h_d_style, build_focus_style};

#[derive(Default, Clone, Copy)]
pub struct EditProps {
    pub color: SemColor,
    pub size: SemSize,
}

pub fn get_edit_style(props: EditProps) -> Box<dyn Fn(Style) -> Style> {
    let theme = get_theme();
    let e_h = crate::BASIS_HEIGHT_UNIT;
    let bg_sem_color = theme.bg_sem_color(props.color);
    let style_creator = move |s: Style| {
        let input_style = build_b_f_h_d_style(props.color, theme.clone())(s)
            .focus(|s| build_focus_style(props.color, theme.clone())(s))
            .font_size(theme.font_size(props.size))
            .cursor(CursorStyle::Text)
            .outline(1)
            .border_radius(0.5 * 16.)
            .items_center()
            .outline_color(bg_sem_color)
            .justify_center();

        let pick_size_style = |s: Style| match props.size {
            SemSize::Large => s.height(4.0 * e_h).padding_horiz(1.5 * e_h),
            SemSize::Normal => s.height(3.0 * e_h).padding_horiz(1.0 * e_h),
            SemSize::Small => s.height(2.0 * e_h).padding_horiz(0.75 * e_h),
            SemSize::Tiny => s.height(1.5 * e_h).padding_horiz(0.5 * e_h),
        };

        let edit_style = pick_size_style(input_style);
        edit_style
    };

    Box::new(style_creator)
}

pub fn sedit(buffer: RwSignal<String>, props: Option<EditProps>) -> TextInput {
    let base_widget = f_text_input(buffer);
    let props = props.unwrap_or(EditProps::default());
    let styled_input = base_widget.style(move |s| get_edit_style(props)(s));
    styled_input
}
