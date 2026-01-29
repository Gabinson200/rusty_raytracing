use crate::vec3::Color;
use crate::interval::Interval;

impl Color{

    pub fn write_color(pixel_color: Color) {
        let r = pixel_color.x();
        let g = pixel_color.y();
        let b = pixel_color.z();

        // Translate thh [0,1] range to [0,255]
        let intensity:Interval = Interval::new(0.000, 0.999);
        let ir = (256.0 * intensity.clamp(r)) as i32;
        let ig = (256.0 * intensity.clamp(g)) as i32;
        let ib = (256.0 * intensity.clamp(b)) as i32;
        println!("{ir} {ig} {ib}");
    }


}

// talking ot grandma gimme a sec
