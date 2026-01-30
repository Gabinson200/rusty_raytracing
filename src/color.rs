use crate::vec3::Color;
use crate::interval::Interval;

impl Color{

    #[inline]
    fn linear_to_gamma(linear_component: f64) -> f64{
        if linear_component > 0.0 {
            return linear_component.sqrt();
        }
        return 0.0;
    }

    pub fn write_color(pixel_color: Color) {
        let mut r = pixel_color.x();
        let mut g = pixel_color.y();
        let mut b = pixel_color.z();

        // Apply gamma correction with gamma=2.0
        let r = Color::linear_to_gamma(r);
        let g = Color::linear_to_gamma(g);
        let b = Color::linear_to_gamma(b);

        // Translate thh [0,1] range to [0,255]
        let intensity:Interval = Interval::new(0.000, 0.999);
        let ir = (256.0 * intensity.clamp(r)) as i32;
        let ig = (256.0 * intensity.clamp(g)) as i32;
        let ib = (256.0 * intensity.clamp(b)) as i32;
        println!("{ir} {ig} {ib}");
    }


}
