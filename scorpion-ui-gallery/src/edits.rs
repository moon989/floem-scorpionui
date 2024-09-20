use floem::{
    reactive::create_rw_signal,
    views::{h_stack, label, v_stack, Decorators},
    View,
};
use scorpion_ui::{
    theme_props::{SemColor, SemSize},
    widgets::{
        divider::sdivider,
        edit::{sedit, EditProps},
        header::sheader,
    },
};

pub fn edit_sem_color() -> impl View {
    let default_text = create_rw_signal(String::from("I am default!"));
    let primary_text = create_rw_signal(String::from("I am primary!"));
    let secondary_text = create_rw_signal(String::from("I am secondary!"));

    v_stack((
        sheader("Text input variants", None),
        sdivider(),
        label(|| "Supports same variants as buttons. Only a few shown here."),
        h_stack((
            sedit(default_text, None),
            sedit(
                primary_text,
                Some(EditProps {
                    color: SemColor::Primary,
                    ..Default::default()
                }),
            ),
            sedit(
                secondary_text,
                Some(EditProps {
                    color: SemColor::Secondary,
                    ..Default::default()
                }),
            ),
        ))
        .style(|s| s.gap(5.)),
    ))
    .style(|s| s.justify_start().items_start().gap(5.))
}

fn edit_sizes() -> impl View {
    let large_text = create_rw_signal(String::from("I am large!"));
    let normal_text = create_rw_signal(String::from("I am normal!"));
    let small_text = create_rw_signal(String::from("I am small!"));
    let tiny_text = create_rw_signal(String::from("I am tiny!"));

    v_stack((
        sheader("Text input sizes", None),
        sdivider(),
        h_stack((
            sedit(
                large_text,
                Some(EditProps {
                    size: SemSize::Large,
                    ..Default::default()
                }),
            ),
            sedit(normal_text, None),
            sedit(
                small_text,
                Some(EditProps {
                    size: SemSize::Small,
                    ..Default::default()
                }),
            ),
            sedit(
                tiny_text,
                Some(EditProps {
                    size: SemSize::Tiny,
                    ..Default::default()
                }),
            ),
        ))
        .style(|s| s.gap(5.)),
    ))
}

pub fn edits_view() -> impl View {
    v_stack(
         (edit_sem_color(), edit_sizes() ),
    )
    .style(move |s| s.width_full().height_full())
}
