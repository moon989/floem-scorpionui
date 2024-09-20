use floem::{
    event::EventListener,
    peniko::Color,
    reactive::{create_signal, SignalGet, SignalUpdate},
    style::{Background, CursorStyle, Transition},
    unit::UnitExt,
    views::{
        container, h_stack, label, scroll, stack, tab, v_stack, virtual_stack, Container,
        Decorators, VirtualDirection, VirtualItemSize,
    },
    View,
};
use scorpion_ui::{
    theme_props::{SemColor, SemSize},
    widgets::{
        button::{sbutton, ButtonProps},
        divider::sdivider,
        header::{sheader, LabelProps},
    },
};

use crate::{
    badges::badges_view, buttons::buttons_view, checkboxs::checkboxs_view, combox_test::combobox_view, edits::edits_view, radiobuttons::radios_view, toggles::toggles_view, tooltips::tooltips_view
};

pub fn change_app_theme() -> impl View {
    h_stack((
        sheader(
            "Click to Change Theme",
            Some(LabelProps {
                size: SemSize::Small,
            }),
        ),
        stack((
            sbutton(
                || "Scorpion_Ui_Dark_Theme",
                Some(ButtonProps {
                    color: SemColor::Info,
                    outlined: true,
                    ..Default::default()
                }),
            )
            .on_click_stop(move |_| {
                let _ = scorpion_ui::change_theme(0);
            }),
            sbutton(
                || "Dark Theme",
                Some(ButtonProps {
                    color: SemColor::Info,
                    outlined: true,
                    ..Default::default()
                }),
            )
            .on_click_stop(move |_| {
                let _ = scorpion_ui::change_theme(2);
            }),
            sbutton(
                || "Blue Theme",
                Some(ButtonProps {
                    color: SemColor::Success,
                    outlined: true,
                    ..Default::default()
                }),
            )
            .on_click_stop(move |_| {
                let _ = scorpion_ui::change_theme(1);
            }),
        ))
        .style(|s| s.gap(4.0).items_start()),
    ))
    .style(|s| s.gap(10.).items_start().width_full())
}

fn skeleton() -> impl View {
    let top_bar = v_stack((
        sheader(
            "Component Gallery Examples ",
            Some(LabelProps {
                size: SemSize::Normal,
                ..Default::default()
            }),
        ),
        sdivider(),
        change_app_theme(),
        sdivider(),
    ))
    .style(|s| s.width_full().items_center().gap(3.));

    

    let tabs: im::Vector<&str> = vec![
        "Button", "Badge", "CheckBox", "Toggle", "Radio", "Edit", "Tooltip", "ComboBox", "Editor_Inner",
    ]
    .into_iter()
    .collect();

    let (tabs, _set_tabs) = create_signal(tabs);
    let (active_tab, set_active_tab) = create_signal(0);

    let left_nav_list = container(
        scroll({
            virtual_stack(
                VirtualDirection::Vertical,
                VirtualItemSize::Fixed(Box::new(|| 36.0)),
                move || tabs.get(),
                move |item| *item,
                move |item| {
                    let index = tabs
                        .get_untracked()
                        .iter()
                        .position(|it| *it == item)
                        .unwrap();

                    stack((label(move || item).style(|s| s.font_size(18.0)),))
                        .on_click_stop(move |_| {
                            set_active_tab.update(|v: &mut usize| {
                                *v = tabs
                                    .get_untracked()
                                    .iter()
                                    .position(|it| *it == item)
                                    .unwrap();
                            });
                        })
                        .style(move |s| {
                            s.flex_row()
                                .padding(5.0)
                                .width(100_i32.pct())
                                .height(36.0)
                                .transition(Background, Transition::linear(0.4))
                                .items_center()
                                .border_bottom(1.0)
                                .border_color(Color::LIGHT_GRAY)
                                .apply_if(index == active_tab.get(), |s| {
                                    s.background(Color::GRAY.with_alpha_factor(0.6))
                                })
                                .focus_visible(|s| s.border(2.).border_color(Color::BLUE))
                                .hover(|s| {
                                    s.background(Color::LIGHT_GRAY)
                                        .apply_if(index == active_tab.get(), |s| {
                                            s.background(Color::GRAY)
                                        })
                                        .cursor(CursorStyle::Pointer)
                                })
                        })
                },
            )
            .style(|s| s.flex_col().width(140.0).height_full().items_center())
        })
        .style(|s| s.flex_col().width(140.0)),
    )
    .style(|s| {
        s.border(1.0)
            .border_color(Color::BLUE)
            .width(140.)
            .height_full()
    });

    let tab = tab(
        move || active_tab.get(),
        move || tabs.get(),
        |it| *it,
        |it| match it {
            "Button" => pannel_box(buttons_view()),
            "Badge" => pannel_box(badges_view()),
            "CheckBox" => pannel_box(checkboxs_view()),
            "Toggle" => pannel_box(toggles_view()),
            "Radio" => pannel_box(radios_view()),
            "Edit" => pannel_box(edits_view()),
            "Tooltip" => pannel_box(tooltips_view()),
            "ComboBox" => pannel_box(combobox_view()),
            
            
            _ => pannel_box(label(|| "Not implemented Component ".to_owned())),
        },
    )
    .style(|s| s.flex_col().items_start());

    let tab = scroll(tab).style(|s| {
        s.flex_basis(0)
            .min_width(0)
            .flex_grow(1.0)
            .width_full()
            .height_full()
            .border(1.0)
            .border_color(Color::SKY_BLUE)
    });

    let view_content = h_stack((left_nav_list, tab))
        .style(|s| s.padding(5.0).height_full().gap(5.0).items_start());

    let view = v_stack((top_bar, view_content))
        .window_title(|| "Compoent Gallery".to_owned())
        .style(|s| s.height_full().width_full().gap(5.0));
    let id = view.id();
    view.on_event_stop(EventListener::KeyUp, move |e| {
        if let floem::event::Event::KeyUp(e) = e {
            if e.key.logical_key == floem::keyboard::Key::Named(floem::keyboard::NamedKey::F11) {
                id.inspect();
            }
        }
    })
}

fn pannel_box(child: impl View + 'static) -> Container {
    container(child).style(|s| s.padding(10.))
}

pub fn app_view() -> impl View {
    skeleton()
}
