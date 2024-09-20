#[derive(Default, Clone, Copy)]
pub enum SemSize {
    Large,
    #[default]
    Normal,
    Small,
    Tiny,
}

#[derive(Default, Clone, Copy)]
pub enum SemColor {
    #[default]
    Default,

    Neutral,
    Primary,
    Secondary,
    Accent,
    Ghost,
    Link,

    Info,
    Success,
    Warning,
    Error,
}

#[derive(Default, Clone, Copy)]
pub enum SemStyle {
    #[default]
    Line,
    Dot,
    DoubleDot,
}
