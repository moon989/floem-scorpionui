use floem::peniko::Color;
use floem::style::{CursorStyle, Style};
use floem::views::{container, svg};
use floem::{views::Decorators, View};

use scorpion_ui_theme::get_theme;
use scorpion_ui_theme::theme_props::{SemColor, SemSize};



#[derive(Default, Clone, Copy)]
pub struct IconButtonProps {
    pub color: SemColor,
    pub outlined: bool,
    pub size: SemSize,
}

fn get_clickable_icon_style(props: IconButtonProps) -> Box<dyn Fn(Style) -> Style> {
    
    let w_h = crate::BASIS_HEIGHT_UNIT;
    let style_creator = move |s: Style| {
        let base = s.color(get_theme().content_sem_color(props.color));
        
        let pick_size_style = |s: Style| match props.size {
            SemSize::Large => s.width(4.0 * w_h).height(4. * w_h),
            SemSize::Normal => s.width(2.0 * w_h).height(2.0 * w_h),
            SemSize::Small => s.width(1.5 * w_h).height(1.5 * w_h),
            SemSize::Tiny => s.width(1. * w_h).height(1. * w_h),
        };
        let style = pick_size_style(base);
        style
    };
    Box::new(style_creator)
}

pub fn sicon_button(
    icon: impl Fn() -> String + 'static,
    on_click: Option<impl Fn() + 'static>,
    active_fn: impl Fn() -> bool + 'static,
    disabled_fn: impl Fn() -> bool + 'static + Copy,
    props: Option<IconButtonProps>,
) -> impl View {
    let props = props.unwrap_or(IconButtonProps::default());

    let active_color = get_theme().active_sem_color(props.color);
    let view = container(
        svg(move || icon())
            .style(move |s| get_clickable_icon_style(props)(s))
            .disabled(disabled_fn),
    )
    .disabled(disabled_fn)
    .style(move |s| {
        s.padding(4.0)
            .border_radius(6.0)
            .border(1.0)
            .border_color(Color::TRANSPARENT)
            .apply_if(active_fn(), |s| s.border_color(active_color))
            .hover(|s| {
                s.cursor(CursorStyle::Pointer)
                    .background(get_theme().hover_sem_color(props.color))
            })
            .active(|s| s.background(active_color))
    });

    if let Some(on_click) = on_click {
        view.on_click_stop(move |_| {
            on_click();
        })
    } else {
        view
    }
}
