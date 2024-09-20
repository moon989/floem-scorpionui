use floem::peniko::Color;
use palette::{FromColor, Hsl, Srgb};

fn print_h_s_l(color: &Color) {
    
    let scolor = Srgb::new(color.r, color.g, color.b);

    let base_hsl: Hsl = Hsl::from_color(scolor.into_format::<f32>());

    
    let h = base_hsl.hue.into_degrees();
    let s = base_hsl.saturation;
    let l = base_hsl.lightness;

    println!("h:{}    s:{}   l:{}", h, s, l);
}

fn generate_color_palette(color: &Color, steps: usize) {
    
    let scolor = Srgb::new(color.r, color.g, color.b);
    let base_hsl: Hsl = Hsl::from_color(scolor.into_format::<f32>());
    for i in 0..steps {
        let _tip = 50 + 500;
        
        
        
        
        
        if i % 10 == 0 {
            
            let lightness = 1.0 - (i as f32 * (0.75 / (steps as f32 - 1.0)));
            let hug =
                base_hsl.hue.into_degrees() - (0.01 + (i as f32 * (0.7 / (steps as f32 - 1.0))));
            let mut color = base_hsl.clone();
            color.lightness = lightness;
            color.saturation = color.saturation + hug;

            
            println!(
                "h:{}     s:{}    l:{} ",
                color.hue.into_degrees(),
                color.saturation,
                color.lightness
            );
        }
    }
}

#[test]
fn test_color_pallete() {
    let test_colors = vec![
        "#fef2f2", "#fee2e2", "#fecaca", "#fca5a5", "#f87171", "#ef4444", "#dc2626", "#b91c1c",
        "#991b1b", "#7f1d1d",
    ];
    let c_500 = vec!["#ef4444", "#f97316", "#78716c", "#f59e0b"];
    let c_base = vec!["#FF0000", "#FFFFFF", "#000000", "#0000FF"];
    println!("\n --- red palette ---  \n");
    for item in test_colors {
        let co = Color::parse(item).unwrap();
        print_h_s_l(&co);
    }
    println!("\n --- created color palette ---  \n");

    generate_color_palette(&Color::RED, 100);
    println!("\n-------\n");
    for item in c_500 {
        let co = Color::parse(item).unwrap();
        print_h_s_l(&co);
    }

    println!("\n-------\n");
    for item in c_base {
        let co = Color::parse(item).unwrap();
        print_h_s_l(&co);
    }
}
