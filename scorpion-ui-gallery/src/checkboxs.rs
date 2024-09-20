use floem::{
    reactive::{create_signal, SignalUpdate},
    views::{h_stack, v_stack, Decorators},
    View,
};
use scorpion_ui::{
    theme_props::{SemColor, SemSize},
    widgets::{
        checkbox::{checkbox_labeled, scheckbox, CheckboxProps},
        divider::sdivider,
        header::sheader,
    },
};

fn checkbox_with_state(props: Option<CheckboxProps>) -> impl View {
    let (checked, set_checked) = create_signal(true);

    scheckbox(checked, props).on_click_stop(move |_| {
        set_checked.update(|checked| *checked = !*checked);
    })
}

fn checkboxes_sizes() -> impl View {
    v_stack((
        sheader("Checkbox sizes", None),
        sdivider(),
        h_stack((
            checkbox_with_state(Some(CheckboxProps {
                size: SemSize::Large,
                ..Default::default()
            })),
            checkbox_with_state(None),
            checkbox_with_state(Some(CheckboxProps {
                size: SemSize::Small,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                size: SemSize::Tiny,
                ..Default::default()
            })),
        ))
        .style(|s| s.gap(4.)),
    ))
}

fn checkboxes_sem_color() -> impl View {
    v_stack((
        sheader("Checkbox variants", None),
        sdivider(),
        h_stack((
            checkbox_with_state(None),
            checkbox_with_state(Some(CheckboxProps {
                color: SemColor::Neutral,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                color: SemColor::Primary,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                color: SemColor::Secondary,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                color: SemColor::Accent,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                color: SemColor::Ghost,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                color: SemColor::Link,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                color: SemColor::Info,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                color: SemColor::Success,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                color: SemColor::Warning,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                color: SemColor::Error,
                ..Default::default()
            })),
        ))
        .style(|s| s.gap(4.)),
    ))
}

fn labeled_checkboxes() -> impl View {
    let (checked, set_checked) = create_signal(true);
    v_stack((
        sheader("Labeled checkboxes", None),
        sdivider(),
        v_stack((
            checkbox_labeled(checked, || "I am the default!", None).on_click_stop(move |_| {
                set_checked.update(|checked| *checked = !*checked);
            }),
            checkbox_labeled(
                checked,
                || "I am primary!",
                Some(CheckboxProps {
                    color: SemColor::Primary,
                    ..Default::default()
                }),
            )
            .on_click_stop(move |_| {
                set_checked.update(|checked| *checked = !*checked);
            }),
        ))
        .style(|s| s.gap(8.)),
    ))
    .style(|s| s.gap(4.))
}

pub fn checkboxs_view() -> impl View {
    v_stack( (
        checkboxes_sem_color(),
        checkboxes_sizes(),
        labeled_checkboxes(), 
    ))
    .style(move |s| s.width_full().height_full())
}
