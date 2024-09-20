use palette::{FromColor, Lch, Srgb};


struct InterpolationParams {
    start_color: Lch,
    end_color: Lch,
    t: f32, 
}


fn interpolate_color(params: InterpolationParams) -> Srgb {
    let InterpolationParams {
        start_color,
        end_color,
        t,
    } = params;

    
    let start_l = start_color.l;
    let start_c = start_color.chroma;
    let start_h = start_color.hue.into_radians();

    let end_l = end_color.l;
    let end_c = end_color.chroma;
    let end_h = end_color.hue.into_radians();

    
    let interpolated_l = start_l + (end_l - start_l) * t;
    let interpolated_c = start_c + (end_c - start_c) * t;
    let interpolated_h = start_h + (end_h - start_h) * t;

    
    let interpolated_color = Lch::new(interpolated_l, interpolated_c, interpolated_h);
    Srgb::from_color(interpolated_color)
}


#[test]
fn test() {
    let start_color = Lch::new(70.0, 50.0, 120.0);
    let end_color = Lch::new(50.0, 70.0, 300.0);
    let t = 0.5; 

    let params = InterpolationParams {
        start_color,
        end_color,
        t,
    };

    let interpolated_color = interpolate_color(params);

    println!("Interpolated color: {:?}", interpolated_color);
}
