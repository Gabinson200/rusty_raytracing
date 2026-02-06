use std::io::{self, Write};
use std::sync::Arc;

// Import modules
use rusty_raytracing::utils::prelude::*;

fn bouncing_spheres() {

    // World
    let mut world = HittableList::new();

    // Ground
    //let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let checkered_ground_texture = Arc::new(CheckerTexture::from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
    world.add(Box::new(Sphere::new(Ray::new(Point3::new(0.0, -1000.0, 0.0), Vec3::new(0.0, 0.0, 0.0)), 1000.0, Arc::new(Lambertian::from_texture(checkered_ground_texture)))));

    // Spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center:Ray = Ray::new(Point3::new(a as f64 + 0.9*random_f64(), 0.2, b as f64 + 0.9*random_f64()), Vec3::new(0.0, 0.0, 0.0));
            if (center.origin() - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::new(random_f64()*random_f64(), random_f64()*random_f64(), random_f64()*random_f64());
                    sphere_material = Arc::new(Lambertian::new(albedo));
                    let moving_center: Ray = Ray::new(center.origin(), Vec3::new(0.5, random_f64_range(0.0, 0.2), 0.0));
                    world.add(Box::new(Sphere::new(moving_center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::new(random_f64_range(0.5, 1.0), random_f64_range(0.5, 1.0), random_f64_range(0.5, 1.0));
                    let fuzz = random_f64_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(Ray::new(Point3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0)), 1.0, material1)));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(Ray::new(Point3::new(-4.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0)), 1.0, material2)));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(Ray::new(Point3::new(4.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0)), 1.0, material3)));


    let bvh_root = BVHNode::new(&world);

    // Camera
    let mut camera = Camera::new();

    camera.aspect_ratio = 1.0;//16.0 / 9.0;
    camera.image_width = 600; //400;
    camera.samples_per_pixel = 200; //50;
    camera.max_depth = 50;
    camera.vfov = 20.0;
    camera.look_from = Point3::new(13.0, 2.0, 3.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.6; // degrees
    camera.focus_distance = 10.0;

    camera.background_color = Color::new(0.7, 0.8, 1.0); // light blue background

    //camera.render(&world);
    camera.render(&bvh_root);

    //eprintln!("Image dimensions: {}x{}\n", camera.image_width, camera.image_height);
}


fn checkered_sphere(){
    // World
    let mut world = HittableList::new();

    let checker = CheckerTexture::from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    world.add(Box::new(Sphere::new(Ray::new(Point3::new(0.0, -10.0, 0.0), Vec3::new(0.0, 0.0, 0.0)), 10.0, Arc::new(Lambertian::from_texture(Arc::new(checker.clone()))))));
    world.add(Box::new(Sphere::new(Ray::new(Point3::new(0.0, 10.0, 0.0), Vec3::new(0.0, 0.0, 0.0)), 10.0, Arc::new(Lambertian::from_texture(Arc::new(checker.clone()))))));

    // Camera
    let mut camera = Camera::new();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.vfov = 20.0;
    camera.look_from = Point3::new(13.0, 2.0, 3.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.background_color = Color::new(0.7, 0.8, 1.0);

    camera.defocus_angle = 0.0; // degrees

    //camera.render(&world);
    camera.render(&world);
}

fn earth(){
    // World
    let mut world = HittableList::new();

    let earth_texture: Arc<dyn Texture> = Arc::new(ImageTexture::new("textures/earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::from_texture(earth_texture));
    let globe = Sphere::new(Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)), 2.0, earth_surface);
    world.add(Box::new(globe));

    
    // Camera
    let mut camera = Camera::new();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.vfov = 20.0;
    camera.look_from = Point3::new(0.0, 0.0, 12.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.background_color = Color::new(0.7, 0.8, 1.0);

    camera.defocus_angle = 0.0; // degrees

    //camera.render(&world);
    camera.render(&world);
}


fn perlin_sphere(){
    // World
    let mut world = HittableList::new();

    let perlin_texture: Arc<dyn Texture> = Arc::new(NoiseTexture::new(4.0));
    let perlin_surface = Arc::new(Lambertian::from_texture(perlin_texture));
    let perlin_globe = Sphere::new(Ray::new(Point3::new(0.0, -1000.0, 0.0), Vec3::new(0.0, 0.0, 0.0)), 1000.0, perlin_surface.clone());
    let perlin_sphere = Sphere::new(Ray::new(Point3::new(0.0, 2.0, 0.0), Vec3::new(0.0, 0.0, 0.0)), 2.0, perlin_surface.clone());
    world.add(Box::new(perlin_globe));
    world.add(Box::new(perlin_sphere));

    
    // Camera
    let mut camera = Camera::new();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.vfov = 20.0;
    camera.look_from = Point3::new(13.0, 2.0, 3.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0; // degrees

    camera.background_color = Color::new(0.7, 0.8, 1.0);

    //camera.render(&world);
    camera.render(&world);

}

fn quads(){
    // World
    let mut world = HittableList::new();

    let left_red = Arc::new(Lambertian::new(Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::new(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::new(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::new(Color::new(0.2, 0.8, 0.8)));

    world.add(Box::new(Quad::new(Point3::new(-3.0, -2.0, 5.0), Vec3::new(0.0, 0.0, -4.0), Vec3::new(0.0, 4.0, 0.0), left_red)));
    world.add(Box::new(Quad::new(Point3::new(-2.0, -2.0, 0.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 4.0, 0.0), back_green)));
    world.add(Box::new(Quad::new(Point3::new(3.0, -2.0, 1.0), Vec3::new(0.0, 0.0, 4.0), Vec3::new(0.0, 4.0, 0.0), right_blue)));
    world.add(Box::new(Quad::new(Point3::new(-2.0, 3.0, 1.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0 , 4.0), upper_orange)));
    world.add(Box::new(Quad::new(Point3::new(-2.0, -3.0, 5.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0 , -4.0), lower_teal)));

    
    // Camera
    let mut camera = Camera::new();

    camera.aspect_ratio = 1.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.vfov = 100.0;
    camera.look_from = Point3::new(0.0, 0.0, 9.0);
    camera.look_at = Point3::new(0.0, 0.0, -1.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.background_color = Color::new(0.7, 0.8, 1.0);

    camera.defocus_angle = 0.0; // degrees

    //camera.render(&world);
    camera.render(&world);

}

fn simple_light(){
    // World
    let mut world = HittableList::new();

    let perlin_texture: Arc<dyn Texture> = Arc::new(NoiseTexture::new(4.0));
    let perlin_surface = Arc::new(Lambertian::from_texture(perlin_texture));
    let perlin_globe = Sphere::new(Ray::new(Point3::new(0.0, -1000.0, 0.0), Vec3::new(0.0, 0.0, 0.0)), 1000.0, perlin_surface.clone());
    let perlin_sphere = Sphere::new(Ray::new(Point3::new(0.0, 2.0, 0.0), Vec3::new(0.0, 0.0, 0.0)), 2.0, perlin_surface.clone());
    world.add(Box::new(perlin_globe));
    world.add(Box::new(perlin_sphere));

    let difflight = Arc::new(DiffuseLight::new(Color::new(4.0, 4.0, 4.0)));
    world.add(Box::new(Quad::new(Point3::new(3.0, 1.0, -2.0), Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 2.0, 0.0), difflight.clone())));
    
    let sphere_light = Sphere::new(Ray::new(Point3::new(0.0, 7.0, 0.0), Vec3::new(0.0, 0.0, 0.0)), 2.0, difflight.clone());
    world.add(Box::new(sphere_light));
    
    // Camera
    let mut camera = Camera::new();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.vfov = 20.0;
    camera.look_from = Point3::new(26.0, 3.0, 6.0);
    camera.look_at = Point3::new(0.0, 2.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0; // degrees

    camera.background_color = Color::new(0.0, 0.0, 0.0);
    //camera.background_color = Color::new(0.7, 0.8, 1.0);

    //camera.render(&world);
    camera.render(&world);
}


fn conrell_box(){
    // World
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    world.add(Box::new(Quad::new(Point3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), green))); // left
    world.add(Box::new(Quad::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), red))); // right
    world.add(Box::new(Quad::new(Point3::new(343.0, 554.0, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), light))); // light
    world.add(Box::new(Quad::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), white.clone()))); // floor
    world.add(Box::new(Quad::new(Point3::new(555.0, 555.0, 555.0), Vec3::new(-555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -555.0), white.clone()))); // floor
    world.add(Box::new(Quad::new(Point3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), white.clone()))); // back

    // add boxes
    
    world.add(Box::new(Quad::make_box(&Point3::new(130.0, 0.0, 65.0), &Point3::new(295.0, 165.0, 230.0), white.clone())));
    world.add(Box::new(Quad::make_box(&Point3::new(265.0, 0.0, 295.0), &Point3::new(430.0, 330.0, 460.0), white.clone())));

    // Camera
    let mut camera = Camera::new();

    camera.aspect_ratio = 1.0;
    camera.image_width = 600;
    camera.samples_per_pixel = 200;
    camera.max_depth = 50;

    camera.vfov = 40.0;
    camera.look_from = Point3::new(278.0, 278.0, -800.0);
    camera.look_at = Point3::new(278.0, 278.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0; // degrees

    camera.background_color = Color::new(0.0, 0.0, 0.0);
    //camera.background_color = Color::new(0.7, 0.8, 1.0);

    //camera.render(&world);
    camera.render(&world);
}

fn main() {
    let option = 7;

    match option {
        1 => bouncing_spheres(),
        2 => checkered_sphere(),
        3 => earth(),
        4 => perlin_sphere(),
        5 => quads(),
        6 => simple_light(),
        7 => conrell_box(),
        _ => println!("Invalid option"),
    }
}
