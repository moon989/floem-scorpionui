use floem::{
    reactive::{ReadSignal, SignalGet},
    style::Style,
    views::{svg, Decorators},
    View,
};

use scorpion_ui_theme::get_theme;
use scorpion_ui_theme::theme_props::{SemColor, SemSize};

#[derive(Default, Clone, Copy)]
pub struct ToggleProps {
    pub color: SemColor,
    pub outlined: bool,
    pub size: SemSize,
}

fn get_toggle_style(props: ToggleProps) -> Box<dyn Fn(Style) -> Style> {
    let theme = get_theme();
    let bg_sem_color = theme.bg_sem_color(props.color);
    let t_w = crate::BASIS_WIDTH_UNIT;
    let t_h = crate::BASIS_HEIGHT_UNIT;
    let style_creator = move |s: Style| {
        let base_toggle_style = s.border_radius(31).color(bg_sem_color);
        let pick_size_style = |s: Style| match props.size {
            
            SemSize::Large => s.min_width(3.2 * t_w).min_height(3.2 * t_h),
            SemSize::Normal => s.min_width(2.2 * t_w).min_height(2.2 * t_h),
            SemSize::Small => s.min_width(1.8 * t_w).min_height(1.5 * t_h),
            SemSize::Tiny => s.min_width(1.2 * t_w).min_height(1.2 * t_h),
        };
        let toggle_style = pick_size_style(base_toggle_style);
        toggle_style
    };

    Box::new(style_creator)
}

fn toggle_ball_svg(enabled: ReadSignal<bool>, props: Option<ToggleProps>) -> impl View {
    const OFF_SVG: &str = r#"<svg viewBox="0 0 24 12" xmlns="http:
    <rect x="0.5" y="0.5" width="23" height="11" rx="6" stroke="currentColor" stroke-width="0.5" />
    <circle cx="6" cy="6" r="4.5" fill="currentColor"/>
    </svg>
    "#;
    const ON_SVG: &str = r#"<svg viewBox="0 0 24 12" xmlns="http:
    <rect x="0.5" y="0.5" width="23" height="11" rx="6" stroke="currentColor" stroke-width="0.5" />
    <circle cx="18" cy="6" r="4.5" fill="currentColor"/>
    </svg>
    "#;

    let svg_str = move || if enabled.get() { ON_SVG } else { OFF_SVG }.to_string();

    let base_widget = svg(svg_str);

    let props = props.unwrap_or(ToggleProps::default());

    let styled_toggle = base_widget.style(move |s| get_toggle_style(props)(s));
    styled_toggle
}

pub fn stoggle(enabled: ReadSignal<bool>, props: Option<ToggleProps>) -> impl View {
    toggle_ball_svg(enabled, props)
}
