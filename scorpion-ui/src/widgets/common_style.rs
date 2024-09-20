use std::sync::Arc;

use floem::style::Style;
use scorpion_ui_theme::{theme::Theme, theme_props::SemColor};


pub fn build_b_f_h_d_style(sem_color: SemColor, theme: Arc<Theme>) -> Box<dyn Fn(Style) -> Style> {
    let hover_bg  = theme.hover_sem_color(sem_color);
    let disable_bg = theme.disable_sem_color(sem_color);
    let style_creator = move |s: Style| {
        s.background(theme.bg_sem_color(sem_color))
            .color(theme.content_sem_color(sem_color))
            .hover(|s| {
                s.background(hover_bg)
                    .color(theme.foreground_color(hover_bg))
            })
            .disabled(|s| {
                s.background(disable_bg)
                    .color(theme.foreground_color(disable_bg))
            })
    };
    Box::new(style_creator)
}

pub fn build_focus_style(sem_color: SemColor, theme: Arc<Theme>) -> Box<dyn Fn(Style) -> Style> {
    let focus_color = theme.focus_sem_color(sem_color);
    let style_creator = move |s: Style| {
        s.background(focus_color)
            .color(theme.foreground_color(focus_color))
    };
    Box::new(style_creator)
}
