use floem::peniko::Color;
use palette::{
    color_difference::Wcag21RelativeContrast, Darken, FromColor, Hsl, Lighten, Srgb,
};

use crate::{
    theme_config::ThemeHex,
    theme_props::{SemColor, SemSize},
};
use anyhow::Result;

pub struct ThemeLayout {
    pub fontsize: SizeConfig,
    pub radius: SizeConfig,
    pub borderwidth: SizeConfig,
}

pub struct SizeConfig {
    pub tiny: f32,
    pub small: f32,
    pub normal: f32,
    pub large: f32,
}

pub struct ThemeColor {
    pub background: Color,
    pub foreground: Color,
    pub divider: Option<Color>,
    pub focus: Option<Color>,
    pub overlay: Option<Color>,

    pub default: Color,
    pub neutral: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub ghost: Color,
    pub link: Color,

    pub info: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
}

pub struct Theme {
    pub name: String,
    pub disabledopacity: f32,
    pub layout: ThemeLayout,
    pub color: ThemeColor,
}

impl Theme {
    
    pub fn new(value: &ThemeHex) -> Result<Theme> {
        let color = value.build_theme_color()?;
        let layout = value.build_theme_layout();
        let name = value.name.clone();
        let disabledopacity = value.disabledopacity;
        let res = Theme {
            name,
            disabledopacity,
            layout,
            color,
        };
        Ok(res)
    }
}

impl Default for Theme {
    
    fn default() -> Self {
        let layout = ThemeLayout {
            fontsize: SizeConfig {
                tiny: 12.,
                small: 14.,
                normal: 16.,
                large: 20.,
            },
            radius: SizeConfig {
                tiny: 4.,
                small: 8.,
                normal: 12.,
                large: 14.,
            },
            borderwidth: SizeConfig {
                tiny: 0.5,
                small: 1.,
                normal: 2.,
                large: 3.,
            },
        };

        let color = ThemeColor {
            background: Color::rgb8(40, 42, 54),
            foreground: Color::rgb8(236, 237, 238),

            divider: Some(Color::rgb8(255, 255, 255).with_alpha_factor(0.15)),
            focus: Some(Color::rgb8(0, 111, 238)),
            overlay: Some(Color::rgb8(0, 0, 0).with_alpha_factor(0.4)),

            default: Color::rgb8(63, 63, 70),
            neutral: Color::rgb8(42, 50, 60),
            primary: Color::rgb8(0, 111, 238),
            secondary: Color::rgb8(147, 83, 211),
            accent: Color::rgb8(255, 113, 215),
            ghost: Color::TRANSPARENT,
            link: Color::rgb8(116, 128, 255),
            info: Color::rgb8(0, 188, 255),
            success: Color::rgb8(23, 201, 100),
            warning: Color::rgb8(245, 165, 36),
            error: Color::rgb8(243, 18, 96),
        };

        Self {
            name: "Scorpion_Ui_Dark_Theme".to_string(),
            layout,
            color,
            disabledopacity: 0.65,
        }
    }
}

impl Theme {
    pub fn font_size(&self, sem_size: SemSize) -> f32 {
        match sem_size {
            SemSize::Large => self.layout.fontsize.large,
            SemSize::Normal => self.layout.fontsize.normal,
            SemSize::Small => self.layout.fontsize.small,
            SemSize::Tiny => self.layout.fontsize.tiny,
        }
    }

    pub fn default_bg_seme_color(&self) -> Color {
        self.bg_sem_color(SemColor::Default)
    }

    pub fn bg_sem_color(&self, sem_color: SemColor) -> Color {
        match sem_color {
            SemColor::Default => self.color.default,
            SemColor::Neutral => self.color.neutral,
            SemColor::Primary => self.color.primary,
            SemColor::Secondary => self.color.secondary,
            SemColor::Accent => self.color.accent,
            SemColor::Ghost => self.color.ghost,
            SemColor::Link => self.color.link,
            SemColor::Info => self.color.info,
            SemColor::Success => self.color.success,
            SemColor::Warning => self.color.warning,
            SemColor::Error => self.color.error,
        }
    }

    pub fn hover_sem_color(&self, sem_color: SemColor) -> Color {
        match sem_color {
            SemColor::Default => hover_color(&self.color.default),
            SemColor::Neutral => hover_color(&self.color.neutral),
            SemColor::Primary => hover_color(&self.color.primary),
            SemColor::Secondary => hover_color(&self.color.secondary),
            SemColor::Accent => hover_color(&self.color.accent),
            SemColor::Ghost => hover_color(&self.color.ghost),
            SemColor::Link => hover_color(&self.color.link),
            SemColor::Info => hover_color(&self.color.info),
            SemColor::Success => hover_color(&self.color.success),
            SemColor::Warning => hover_color(&self.color.warning),
            SemColor::Error => hover_color(&self.color.error),
        }
    }

    pub fn active_sem_color(&self, sem_color: SemColor) -> Color {
        match sem_color {
            SemColor::Default => active_color(&self.color.default),
            SemColor::Neutral => active_color(&self.color.neutral),
            SemColor::Primary => active_color(&self.color.primary),
            SemColor::Secondary => active_color(&self.color.secondary),
            SemColor::Accent => active_color(&self.color.accent),
            SemColor::Ghost => active_color(&self.color.ghost),
            SemColor::Link => active_color(&self.color.link),
            SemColor::Info => active_color(&self.color.info),
            SemColor::Success => active_color(&self.color.success),
            SemColor::Warning => active_color(&self.color.warning),
            SemColor::Error => active_color(&self.color.error),
        }
    }

    pub fn focus_sem_color(&self, sem_color: SemColor) -> Color {
        if self.color.focus.is_some() {
            self.color.focus.unwrap().clone()
        } else {
            match sem_color {
                SemColor::Default => focus_color(&self.color.default),
                SemColor::Neutral => focus_color(&self.color.neutral),
                SemColor::Primary => focus_color(&self.color.primary),
                SemColor::Secondary => focus_color(&self.color.secondary),
                SemColor::Accent => focus_color(&self.color.accent),
                SemColor::Ghost => focus_color(&self.color.ghost),
                SemColor::Link => focus_color(&self.color.link),
                SemColor::Info => focus_color(&self.color.info),
                SemColor::Success => focus_color(&self.color.success),
                SemColor::Warning => focus_color(&self.color.warning),
                SemColor::Error => focus_color(&self.color.error),
            }
        }
    }

    pub fn disable_sem_color(&self, sem_color: SemColor) -> Color {
        let bg = self.bg_sem_color(sem_color);
        bg.with_alpha_factor(self.disabledopacity)
    }

    pub fn content_sem_color(&self, sem_color: SemColor) -> Color {
        let bg_color = self.bg_sem_color(sem_color);
        foreground_color(&bg_color)
    }

    

    


    pub fn base_content_color(&self) -> Color {
        
        foreground_color(&self.color.background)
    }

    pub fn primary_content_color(&self) -> Color {
        foreground_color(&self.color.primary)
    }

    pub fn secondary_content_color(&self) -> Color {
        foreground_color(&self.color.secondary)
    }

    pub fn accent_content_color(&self) -> Color {
        foreground_color(&self.color.accent)
    }

    pub fn neutral_content_color(&self) -> Color {
        foreground_color(&self.color.neutral)
    }

    pub fn info_content_color(&self) -> Color {
        foreground_color(&self.color.info)
    }

    pub fn success_content_color(&self) -> Color {
        foreground_color(&self.color.success)
    }

    pub fn warning_content_color(&self) -> Color {
        foreground_color(&self.color.warning)
    }

    pub fn error_content_color(&self) -> Color {
        
        foreground_color(&self.color.error)
    }

    pub fn foreground_color(&self,color:Color) ->Color {
        foreground_color(&color)
    }
}


fn active_color(color: &Color) -> Color {
    
    let bg_color = Srgb::new(color.r, color.g, color.b);
    let bg_luminance = luminance(&bg_color);

    let mut bg_hsl: Hsl = Hsl::from_color(bg_color.into_format::<f32>());

    
    if bg_luminance > 0.55 {
        
        bg_hsl.lightness = (bg_hsl.lightness * 0.3).min(0.75);
        bg_hsl.saturation = (bg_hsl.saturation * 0.5).min(0.55);
    } else {
        
        bg_hsl.lightness = (bg_hsl.lightness * 1.4).max(0.5);
        bg_hsl.saturation = (bg_hsl.saturation * 0.6).min(0.6);
    }

    let fg_color: Srgb<f32> = Srgb::from_color(bg_hsl);
    let fg = fg_color.into_format::<u8>();
    Color::rgb8(fg.red, fg.green, fg.red)
}

fn focus_color(color: &Color) -> Color {
    let bg_color = Srgb::new(color.r, color.g, color.b);
    let bg_luminance = luminance(&bg_color);
    let mut bg_hsl: Hsl = Hsl::from_color(bg_color.into_format::<f32>());
    if bg_luminance >= 0.5 {
        bg_hsl = bg_hsl.darken(0.015);
        bg_hsl.saturation = (bg_hsl.saturation * 0.95).min(0.2);
    } else {
        bg_hsl = bg_hsl.lighten(0.015);
        bg_hsl.saturation = (bg_hsl.saturation * 0.95).min(0.2);
    }

    let fg_color: Srgb<f32> = Srgb::from_color(bg_hsl);
    let fg = fg_color.into_format::<u8>();
    Color::rgb8(fg.red, fg.green, fg.red)
}

fn hover_color(color: &Color) -> Color {
    let bg_color = Srgb::new(color.r, color.g, color.b);
    let bg_luminance = luminance_ex(color);

    
    let mut bg_hsl: Hsl = Hsl::from_color(bg_color.into_format::<f32>());

    
    if !is_dark(color)  {
        
        bg_hsl = bg_hsl.darken((bg_luminance - 0.5).max(0.04).min(0.01));
        
        bg_hsl.saturation = (bg_hsl.saturation * 0.5).min(0.5);
    } else {
        
        bg_hsl = bg_hsl.lighten((0.5- bg_luminance).max(0.03).min(0.01));
        
        bg_hsl.saturation = (bg_hsl.saturation * 0.9).min(0.6);
    }

    let fg_color: Srgb<f32> = Srgb::from_color(bg_hsl);
    let fg = fg_color.into_format::<u8>();
    Color::rgb8(fg.red, fg.green, fg.red)
}

fn luminance(color: &Srgb<u8>) -> f32 {
    
    let linear_color: Srgb<f32> = color.into_format();
    let luma = linear_color.into_linear().relative_luminance();
    luma.luma
}

fn luminance_ex(c: &Color) -> f32 {
    return 0.2126 * (c.r as f32) + 0.7152 * (c.g as f32) + 0.0722 * (c.b as f32);
}

fn contrast(c1: &Color, c2: &Color) -> f32 {
    let l1 = luminance_ex(c1);
    let l2 = luminance_ex(c2);
    return (l1.max(l2) + 0.05) / (l1.min(l2) + 0.05);
}
fn is_dark(c: &Color) -> bool {
    if contrast(c, &Color::BLACK) < contrast(c, &Color::WHITE) {
        true
    } else {
        false
    }
}



























fn foreground_color(color: &Color) -> Color {
    let bg_color = Srgb::new(color.r, color.g, color.b);
    let bg_luminance = luminance(&bg_color);

    
    let mut bg_hsl: Hsl = Hsl::from_color(bg_color.into_format::<f32>());

    
    if bg_luminance > 0.5 {
        
        
        bg_hsl.lightness = (bg_hsl.lightness * ((1.0 - bg_luminance).min(0.2))).min(0.4);
        
        bg_hsl.saturation = (bg_hsl.saturation * ((1.0 - bg_luminance).min(0.45))).min(0.5);
    } else {
        
        
        bg_hsl.lightness = (bg_luminance * 2.8 * bg_hsl.lightness * 1.4).max(0.8);
        
        bg_hsl.saturation = (bg_hsl.saturation * bg_luminance).min(0.6);
    }

    
    let fg_color: Srgb<f32> = Srgb::from_color(bg_hsl);
    let fg = fg_color.into_format::<u8>();
    Color::rgb8(fg.red, fg.green, fg.red)
}

#[cfg(test)]
mod test {
    use palette::{FromColor, Srgb};

    use super::*;

    #[test]
    fn test_rgb_tooklch() {
        let c1 = Color::rgb(255.0, 255., 255.);
        let rgb: Srgb<f32> = Srgb::new(c1.r, c1.g, c1.b).into_format();
        let my_lch = palette::Oklch::from_color(rgb);
        
        
        println!("OKLCH from RGB: {:?}", my_lch);
    }

    #[test]
    fn test_focus_color() {
        let color = Color::BLUE;
        let bg_color = Srgb::new(color.r, color.g, color.b);
        let bg_luminance = luminance(&bg_color);
        let mut bg_hsl: Hsl = Hsl::from_color(bg_color.into_format::<f32>());
        if bg_luminance > 0.5 {
            println!(" i am light light is: {} ", bg_hsl.lightness);
            bg_hsl = bg_hsl.darken(0.1);
            println!(" i am light light  after darken : {} ", bg_hsl.lightness);
        } else {
            println!(" i am dark light is: {} ", bg_hsl.lightness);
            bg_hsl = bg_hsl.lighten(0.1);
            println!(" i am dark light  after lighten : {} ", bg_hsl.lightness);
        }
    }
}
