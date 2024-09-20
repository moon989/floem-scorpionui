use super::icon_button::{sicon_button, IconButtonProps};
use floem::{
    event::{Event, EventListener},
    kurbo::Size,
    reactive::{RwSignal, Scope, SignalGet, SignalUpdate},
    views::{
        container, drag_resize_window_area, drag_window_area, empty, h_stack, stack, svg, v_stack,
        Decorators,
    },
    window::{ResizeDirection, WindowConfig, WindowId},
    IntoView, View,
};
use scorpion_ui_theme::{get_theme, theme_props::SemSize};

const WINDOW_CLOSE_ICON: &str = include_str!("./../../../icons/codicons/chrome-close.svg");
const WINDOW_MAX_ICON: &str = include_str!("./../../../icons/codicons/chrome-maximize.svg");
const WINDOW_MIN_ICON: &str = include_str!("./../../../icons/codicons/chrome-minimize.svg");
const WINDOW_RESTORE_ICON: &str = include_str!("./../../../icons/codicons/chrome-restore.svg");

pub struct SApplicaton {
    pub window_maximized: RwSignal<bool>,
    pub logo: Option<&'static str>,
    pub flag_allow_customer_window_bar: RwSignal<bool>,
    pub window_bar_height: f64,
}

impl SApplicaton {
    pub fn new(
        cx: Scope,
        logo: Option<&'static str>,
        flag_allow_customer_window_bar: RwSignal<bool>,
        window_bar_height: f64,
    ) -> Self {
        Self {
            window_maximized: cx.create_rw_signal(false),
            logo,
            flag_allow_customer_window_bar,
            window_bar_height,
        }
    }

    pub fn create_app_with_customer_bar<V: IntoView + 'static>(
        &self,
        app_business_view: impl FnOnce(WindowId) -> V + 'static,
        customer_window_bar_view: impl View + 'static,
        config: Option<WindowConfig>,
        window_size: RwSignal<Size>,
        do_before_close: impl Fn(WindowId) -> bool + 'static,
    ) -> floem::Application {
        let mut app = floem::Application::new();
        let window_maximized = self.window_maximized;
        let flag_allow_customer_window_bar = self.flag_allow_customer_window_bar;
        let str_log = self.logo;
        let height = self.window_bar_height;

        app = app.window(
            move |window_id| {
                v_stack((
                    h_stack((
                        left_logo_view(str_log),
                        h_stack((
                            customer_window_bar_view,
                            drag_window_area(empty()).style(|s| {
                                s.height_full()
                                    .flex_basis(0.0)
                                    .flex_grow(1.0)
                                    .margin_horiz(5.)
                            }),
                        ))
                        .style(|s| s.width_full().height_full().justify_start().items_center()),
                        window_system_button_view(
                            window_id,
                            window_maximized,
                            flag_allow_customer_window_bar,
                            do_before_close,
                        ),
                    ))
                    .style(move |s| s.justify_start().items_center().height(height).width_full()),
                    app_business_view(window_id),
                    window_drag_resize_area_all(window_size, flag_allow_customer_window_bar),
                ))
                .style(move |s| s.width_full().height_full())
                .on_event_cont(EventListener::WindowMaximizeChanged, move |event| {
                    if let Event::WindowMaximizeChanged(maximized) = event {
                        window_maximized.set(*maximized);
                    }
                })
                .style(|s| s.background(get_theme().color.background))
            },
            config,
        );
        app
    }
}

fn window_system_button_view(
    window_id: WindowId,
    window_maximized: RwSignal<bool>,
    customer_window_bar: RwSignal<bool>,
    do_before_close: impl Fn(WindowId) -> bool + 'static,
) -> impl View {
    h_stack((
        sicon_button(
            move || WINDOW_MIN_ICON.to_string(),
            Some(|| {
                floem::action::minimize_window();
            }),
            || false,
            || false,
            Some(IconButtonProps {
                size: SemSize::Small,
                ..Default::default()
            }),
        ),
        sicon_button(
            move || {
                if window_maximized.get() {
                    WINDOW_RESTORE_ICON.to_owned()
                } else {
                    WINDOW_MAX_ICON.to_owned()
                }
            },
            Some(move || {
                floem::action::set_window_maximized(!window_maximized.get_untracked());
            }),
            || false,
            || false,
            Some(IconButtonProps {
                size: SemSize::Small,
                ..Default::default()
            }),
        ),
        sicon_button(
            move || WINDOW_CLOSE_ICON.to_string(),
            Some(move || {
                if do_before_close(window_id) {
                    floem::close_window(window_id)
                }
            }),
            || false,
            || false,
            Some(IconButtonProps {
                size: SemSize::Small,
                ..Default::default()
            }),
        )
        .style(|s| s.margin_right(3.0)),
    ))
    .style(move |s| {
        s.items_center()
            .apply_if(
                cfg!(target_os = "macos") || !customer_window_bar.get_untracked(),
                |s| s.hide(),
            )
            .gap(5.0)
    })
}

fn window_drag_resize_area_all(
    window_size: RwSignal<Size>,
    customer_window_bar: RwSignal<bool>,
) -> impl View {
    stack((
        drag_resize_window_area(ResizeDirection::West, empty())
            .style(|s| s.absolute().width(4.0).height_full()),
        drag_resize_window_area(ResizeDirection::North, empty())
            .style(|s| s.absolute().width_full().height(4.0)),
        drag_resize_window_area(ResizeDirection::East, empty()).style(move |s| {
            s.absolute()
                .margin_left(window_size.get().width as f32 - 4.0)
                .width(4.0)
                .height_full()
        }),
        drag_resize_window_area(ResizeDirection::South, empty()).style(move |s| {
            s.absolute()
                .margin_top(window_size.get().height as f32 - 4.0)
                .width_full()
                .height(4.0)
        }),
        drag_resize_window_area(ResizeDirection::NorthWest, empty())
            .style(|s| s.absolute().width(20.0).height(4.0)),
        drag_resize_window_area(ResizeDirection::NorthWest, empty())
            .style(|s| s.absolute().width(4.0).height(20.0)),
        drag_resize_window_area(ResizeDirection::NorthEast, empty()).style(move |s| {
            s.absolute()
                .margin_left(window_size.get().width as f32 - 20.0)
                .width(20.0)
                .height(4.0)
        }),
        drag_resize_window_area(ResizeDirection::NorthEast, empty()).style(move |s| {
            s.absolute()
                .margin_left(window_size.get().width as f32 - 4.0)
                .width(4.0)
                .height(20.0)
        }),
        drag_resize_window_area(ResizeDirection::SouthWest, empty()).style(move |s| {
            s.absolute()
                .margin_top(window_size.get().height as f32 - 4.0)
                .width(20.0)
                .height(4.0)
        }),
        drag_resize_window_area(ResizeDirection::SouthWest, empty()).style(move |s| {
            s.absolute()
                .margin_top(window_size.get().height as f32 - 20.0)
                .width(4.0)
                .height(20.0)
        }),
        drag_resize_window_area(ResizeDirection::SouthEast, empty()).style(move |s| {
            s.absolute()
                .margin_left(window_size.get().width as f32 - 20.0)
                .margin_top(window_size.get().height as f32 - 4.0)
                .width(20.0)
                .height(4.0)
        }),
        drag_resize_window_area(ResizeDirection::SouthEast, empty()).style(move |s| {
            s.absolute()
                .margin_left(window_size.get().width as f32 - 4.0)
                .margin_top(window_size.get().height as f32 - 20.0)
                .width(4.0)
                .height(20.0)
        }),
    ))
    .debug_name("Drag Resize Areas")
    .style(move |s| {
        s.absolute().size_full().apply_if(
            cfg!(target_os = "macos") || !customer_window_bar.get_untracked(),
            |s| s.hide(),
        )
    })
}

fn left_logo_view(svg_logo: Option<&'static str>) -> Box<dyn View> {
    let is_macos = cfg!(target_os = "macos");

    if svg_logo.is_none() {
        empty().into_any()
    } else {
        stack((
            empty().style(move |s| {
                let should_hide = if is_macos { false } else { true };
                s.width(75.0).apply_if(should_hide, |s| s.hide())
            }),
            container(
                svg(move || svg_logo.unwrap().to_string())
                    .style(move |s| s.size(24.0, 24.0).color(get_theme().color.foreground)),
            )
            .style(move |s| s.margin_horiz(10.0).apply_if(is_macos, |s| s.hide())),
            drag_window_area(empty()).style(|s| {
                s.height_pct(100.0)
                    .width_full()
                    .absolute()
                    .margin_horiz(10.)
            }),
        ))
        .style(move |s| {
            s.height_pct(100.0)
                .flex_basis(0.0)
                .flex_grow(1.0)
                .items_center()
        })
        .debug_name("Left Logo of TOP BAR")
        .into_any()
    }
}
