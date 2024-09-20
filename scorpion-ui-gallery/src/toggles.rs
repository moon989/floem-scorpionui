use floem::{
    reactive::{create_signal, SignalUpdate},
    views::{h_stack, v_stack, Decorators},
    View,
};
use scorpion_ui::{
    theme_props::{SemColor, SemSize},
    widgets::{
        divider::sdivider,
        header::sheader,
        toggle::{stoggle, ToggleProps},
    },
};

fn toggle_with_state(props: Option<ToggleProps>) -> impl View {
    let (checked, set_checked) = create_signal(true);

    stoggle(checked, props).on_click_stop(move |_| {
        set_checked.update(|checked| *checked = !*checked);
    })
}

fn toggle_sizes() -> impl View {
    v_stack((
        sheader("Toggle sizes", None),
        sdivider(),
        h_stack((
            toggle_with_state(Some(ToggleProps {
                size: SemSize::Large,
                ..Default::default()
            })),
            toggle_with_state(None),
            toggle_with_state(Some(ToggleProps {
                size: SemSize::Small,
                ..Default::default()
            })),
            toggle_with_state(Some(ToggleProps {
                size: SemSize::Tiny,
                ..Default::default()
            })),
        ))
        .style(|s| s.gap(4.)),
    ))
}

fn toggle_sem_color() -> impl View {
    v_stack((
        sheader("Toggle variants", None),
        sdivider(),
        h_stack((
            toggle_with_state(None),
            toggle_with_state(Some(ToggleProps {
                color: SemColor::Neutral,
                ..Default::default()
            })),
            toggle_with_state(Some(ToggleProps {
                color: SemColor::Primary,
                ..Default::default()
            })),
            toggle_with_state(Some(ToggleProps {
                color: SemColor::Secondary,
                ..Default::default()
            })),
            toggle_with_state(Some(ToggleProps {
                color: SemColor::Accent,
                ..Default::default()
            })),
            toggle_with_state(Some(ToggleProps {
                color: SemColor::Ghost,
                ..Default::default()
            })),
            toggle_with_state(Some(ToggleProps {
                color: SemColor::Link,
                ..Default::default()
            })),
            toggle_with_state(Some(ToggleProps {
                color: SemColor::Info,
                ..Default::default()
            })),
            toggle_with_state(Some(ToggleProps {
                color: SemColor::Success,
                ..Default::default()
            })),
            toggle_with_state(Some(ToggleProps {
                color: SemColor::Warning,
                ..Default::default()
            })),
            toggle_with_state(Some(ToggleProps {
                color: SemColor::Error,
                ..Default::default()
            })),
        ))
        .style(|s| s.gap(4.)),
    ))
}

pub fn toggles_view() -> impl View {
    v_stack(
         (toggle_sem_color(), toggle_sizes() ),
    )
    .style(move |s| s.width_full().height_full())
}
