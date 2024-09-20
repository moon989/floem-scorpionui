use floem::{
    reactive::{create_rw_signal, RwSignal},
    views::{stack, v_stack, Decorators},
    View,
};
use im::vector;
use scorpion_ui::{
    theme_props::{SemColor, SemSize},
    widgets::{
        combobox::{scombobox, ComboBoxItems, ComboBoxProps},
        divider::sdivider,
        header::sheader,
    },
};

#[derive(Clone)]
struct MyString(String);

impl ComboBoxItems for MyString {
    fn unique_id(&self) -> String {
        self.0.clone()
    }

    fn caption(&self) -> String {
        self.0.clone()
    }
}

fn combox_sem_color(
    items: RwSignal<im::Vector<MyString>>,
    active_value: RwSignal<Option<MyString>>,
) -> impl View {
    v_stack((
        sheader("Sementic Color Buttons", None),
        sdivider(),
        stack((
            scombobox(
                items,
                active_value,
                Some(ComboBoxProps {
                    color: SemColor::Default,
                    ..Default::default()
                }),
            ),
            scombobox(
                items,
                active_value,
                Some(ComboBoxProps {
                    color: SemColor::Primary,
                    ..Default::default()
                }),
            ),
            scombobox(
                items,
                active_value,
                Some(ComboBoxProps {
                    color: SemColor::Secondary,
                    ..Default::default()
                }),
            ),
            scombobox(
                items,
                active_value,
                Some(ComboBoxProps {
                    color: SemColor::Accent,
                    ..Default::default()
                }),
            ),
            scombobox(
                items,
                active_value,
                Some(ComboBoxProps {
                    color: SemColor::Ghost,
                    ..Default::default()
                }),
            ),
            scombobox(
                items,
                active_value,
                Some(ComboBoxProps {
                    color: SemColor::Link,
                    ..Default::default()
                }),
            ),
            scombobox(
                items,
                active_value,
                Some(ComboBoxProps {
                    color: SemColor::Info,
                    ..Default::default()
                }),
            ),
            scombobox(
                items,
                active_value,
                Some(ComboBoxProps {
                    color: SemColor::Success,
                    ..Default::default()
                }),
            ),
            scombobox(
                items,
                active_value,
                Some(ComboBoxProps {
                    color: SemColor::Warning,
                    ..Default::default()
                }),
            ),
            scombobox(
                items,
                active_value,
                Some(ComboBoxProps {
                    color: SemColor::Error,
                    ..Default::default()
                }),
            ),
        ))
        .style(|s| s.gap(4.0)),
    ))
}

fn combox_sem_size(
    items: RwSignal<im::Vector<MyString>>,
    active_value: RwSignal<Option<MyString>>,
) -> impl View {
    v_stack((
        sheader("Sementic Size Buttons", None),
        sdivider(),
        stack((
            scombobox(
                items,
                active_value,
                Some(ComboBoxProps {
                    size: SemSize::Large,
                    ..Default::default()
                }),
            ),
            scombobox(
                items,
                active_value,
                Some(ComboBoxProps {
                    color: SemColor::Primary,
                    size: SemSize::Normal,
                }),
            ),
            scombobox(
                items,
                active_value,
                Some(ComboBoxProps {
                    color: SemColor::Secondary,
                    size: SemSize::Small,
                }),
            ),
            scombobox(
                items,
                active_value,
                Some(ComboBoxProps {
                    color: SemColor::Success,
                    size: SemSize::Tiny,
                }),
            ),
        ))
        .style(|s| s.gap(4.0)),
    ))
}
pub fn combobox_view() -> impl View {
    let values = vector![
        MyString("one".to_owned()),
        MyString("two".to_owned()),
        MyString("three".to_owned()),
        MyString("1".to_owned()),
        MyString("2".to_owned()),
        MyString("3".to_owned()),
        MyString("4".to_owned()),
        MyString("5".to_owned()),
        MyString("6".to_owned()),
        MyString("Long 中文东方闪电东方闪电第三方水电费第三方".to_owned()),
    ];

    let items = create_rw_signal(values);
    let active = create_rw_signal(Some(MyString("two".to_owned())));

    let active1 = create_rw_signal(None);

    v_stack( (
        combox_sem_color(items, active),
        combox_sem_size(items, active1),
        
    ))
    .style(move |s| s.width_full().height_full())
}
