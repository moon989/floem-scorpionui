use floem::{
    action::{add_overlay, remove_overlay},
    event::EventListener,
    kurbo::{Point, Size},
    reactive::{create_effect, create_rw_signal, RwSignal,SignalGet, SignalUpdate},
    style::{CursorStyle, Style},
    views::{
        container, dyn_stack, label,
        scroll::{scroll, PropagatePointerWheel},
        stack, svg, Decorators,
    },
    View,
};
use scorpion_ui_theme::{
    get_theme,
    theme_props::{SemColor, SemSize},
};

use super::common_style::build_b_f_h_d_style;

#[derive(Default, Clone, Copy)]
pub struct ComboBoxProps {
    pub color: SemColor,
    pub size: SemSize,
}

pub trait ComboBoxItems {
    fn unique_id(&self) -> String;
    fn caption(&self) -> String;
}

fn get_combobox_style(
    props: ComboBoxProps,
) -> (Box<dyn Fn(Style) -> Style>, Box<dyn Fn(Style) -> Style>) {
    let e_h = crate::BASIS_HEIGHT_UNIT;

    let main_box_style = move |s: Style| {
        let theme = get_theme();
        let bg_sem_color = theme.bg_sem_color(props.color);
        let combobox_style = build_b_f_h_d_style(props.color, theme.clone())(s)
            .font_size(theme.font_size(props.size))
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

        let style = pick_size_style(combobox_style);
        style
    };

    let main_box_svg_style = move |s: Style| {
        let theme = get_theme();
        let base_style = s.color(theme.content_sem_color(props.color));
        let pick_size_style = |s: Style| match props.size {
            SemSize::Large => s.size(1.4 * e_h, 1.4 * e_h),
            SemSize::Normal => s.size(1.2 * e_h, 1.2 * e_h),
            SemSize::Small => s.size(1.0 * e_h, 1.0 * e_h),
            SemSize::Tiny => s.size(0.8 * e_h, 0.8 * e_h),
        };
        let style = pick_size_style(base_style);
        style
    };

    (Box::new(main_box_style), Box::new(main_box_svg_style))
}

const CHEVRON_DOWN: &str = r##"<svg xmlns="http:
  <path fill="#010002" d="M92.672 144.373a10.707 10.707 0 0 1-7.593-3.138L3.145 59.301c-4.194-4.199-4.194-10.992 0-15.18a10.72 10.72 0 0 1 15.18 0l74.347 74.341 74.347-74.341a10.72 10.72 0 0 1 15.18 0c4.194 4.194 4.194 10.981 0 15.18l-81.939 81.934a10.694 10.694 0 0 1-7.588 3.138z"/>
</svg>"##;



pub fn scombobox<T: Clone + ComboBoxItems + 'static>(
    items: RwSignal<im::Vector<T>>,
    active_value: RwSignal<Option<T>>,
    props: Option<ComboBoxProps>,
    
) -> impl View {
    let prop = props.unwrap_or(ComboBoxProps::default());
    let theme = get_theme();
    
    let font_size = theme.font_size(prop.size);
    let (main_view_style, svg_style) = get_combobox_style(prop);

    let window_origin = create_rw_signal(Point::ZERO);
    let size = create_rw_signal(Size::ZERO);
    let overlay_id = create_rw_signal(None);
    let dropdown_input_focus = create_rw_signal(false);
    let dropdown_scroll_focus = create_rw_signal(false);
    let expanded = create_rw_signal(false);
    {
        
        create_effect(move |_| {
            if expanded.get() {
                let id = add_overlay(Point::ZERO, move |_| {
                    combobox_scroll(
                        items.get(),
                        active_value,
                        expanded,
                        dropdown_scroll_focus,
                        dropdown_input_focus,
                        window_origin,
                        size,
                        prop,
                    )
                });
                overlay_id.set(Some(id));
            } else if let Some(id) = overlay_id.get_untracked() {
                remove_overlay(id);
                overlay_id.set(None);
            }
        });
    }

    stack((
        label(move || {
            if active_value.get().is_some() {
                active_value.get().unwrap().caption()
            } else {
                "".to_string()
            }
        })
        .style(move |s| {
            s.text_ellipsis()
                .font_size(font_size)
                .width_pct(100.0)
                .padding_horiz(10.0)
                .color(theme.content_sem_color(prop.color))
                .selectable(false)
        }),
        container(svg(move || String::from(CHEVRON_DOWN)).style(move |s| svg_style(s)))
            .style(|s| s.padding_right(4.0)),
    ))
    .on_click_stop(move |_| {
        expanded.update(|expanded| {
            *expanded = !*expanded;
        });
    })
    .on_move(move |point| {
        window_origin.set(point);
        if expanded.get_untracked() {
            expanded.set(false);
        }
    })
    .on_resize(move |rect| {
        size.set(rect.size());
    })
    .style(move |s| {
        main_view_style(s)
            .items_center()
            .cursor(CursorStyle::Pointer)
            .min_width(3.0 * crate::BASIS_WIDTH_UNIT)
            .max_width(6.0 * crate::BASIS_WIDTH_UNIT)
            .line_height(1.8)
    })
    .keyboard_navigatable()
    .on_event_stop(EventListener::FocusGained, move |_| {
        dropdown_input_focus.set(true);
    })
    .on_event_stop(EventListener::FocusLost, move |_| {
        dropdown_input_focus.set(false);
        if expanded.get_untracked() && !dropdown_scroll_focus.get_untracked() {
            expanded.set(false);
        }
    })
    .on_cleanup(move || {
        if let Some(id) = overlay_id.get_untracked() {
            remove_overlay(id);
        }
    })
}

fn combobox_scroll<T: Clone + ComboBoxItems + 'static>(
    items: im::Vector<T>,
    active_value: RwSignal<Option<T>>,
    expanded: RwSignal<bool>,
    dropdown_scroll_focus: RwSignal<bool>,
    dropdown_input_focus: RwSignal<bool>,
    window_origin: RwSignal<Point>,
    input_size: RwSignal<Size>,
    prop: ComboBoxProps,
    
) -> impl View {
    dropdown_scroll_focus.set(true);
    let theme = get_theme();
    let bg_sem_color = theme.bg_sem_color(prop.color);
    let hover_sem_color = theme.hover_sem_color(prop.color);
    let content_color = theme.content_sem_color(prop.color);
    let font_size = theme.font_size(prop.size);
    let view_fn = move |combox_item: T| {
        let local_item_string = combox_item.caption();
        label(move || local_item_string.clone())
            .on_click_stop(move |_| {
                active_value.set(Some(combox_item.clone()));
                expanded.set(false);
            })
            .style(move |s| {
                s.text_ellipsis()
                    .font_size(font_size)
                    .color(content_color)
                    .padding_horiz(10.0)
                    .hover(|s| s.cursor(CursorStyle::Pointer).background(hover_sem_color))
                    .height_full()
            })
    };

    let scroll_size = create_rw_signal(Size::ZERO);

    scroll(
        dyn_stack(move || items.clone(), |item| item.unique_id(), view_fn)
            .style(|s| s.flex_col().width_pct(100.0).cursor(CursorStyle::Pointer)),
    )
    .style(move |s| {
        s.width_pct(100.0)
            .max_height(200.0)
            .background(bg_sem_color.with_alpha_factor(0.9))
            .set(PropagatePointerWheel, false)
    })
    .keyboard_navigatable()
    .request_focus(|| {})
    .on_event_stop(EventListener::FocusGained, move |_| {
        dropdown_scroll_focus.set(true);
    })
    .on_event_stop(EventListener::FocusLost, move |_| {
        dropdown_scroll_focus.set(false);
        if expanded.get_untracked() && !dropdown_input_focus.get_untracked() {
            expanded.set(false);
        }
    })
    .on_event_stop(EventListener::PointerMove, move |_| {})
    .on_event_stop(EventListener::PointerDown, move |_| {})
    .on_resize(move |rect| {
        scroll_size.set(rect.size());
    })
    .style(move |s| {
        let window_origin = window_origin.get();
        
        let input_size = input_size.get();
        
        
        
        
        
        

        let x = window_origin.x;
        let y = window_origin.y + input_size.height - 1.0;

        
        
        
        
        
        
        

        s.width(250.0)
            .class(floem::views::scroll::Handle, |s| {
                s.background(hover_sem_color.with_alpha_factor(0.75))
            })
            .line_height(1.8)
            .border(1)
            .border_radius(6.0)
            .box_shadow_blur(3.0)
            .inset_left(x)
            .inset_top(y)
    })
}
