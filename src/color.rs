use crate::vec3::Color;

impl Color{

    pub fn write_color(pixel_color: Color) {
        let ir = (255.999 * pixel_color.x()) as i32;
        let ig = (255.999 * pixel_color.y()) as i32;
        let ib = (255.999 * pixel_color.z()) as i32;

        println!("{ir} {ig} {ib}");
    }


}

// talking ot grandma gimme a sec
