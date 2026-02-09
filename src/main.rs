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

    camera.aspect_ratio = 1.0; //16.0 / 9.0;
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
    camera.image_width = 600;
    camera.samples_per_pixel = 200;
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
    camera.image_width = 600;
    camera.samples_per_pixel = 200;
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

    world.add(Box::new(Quad::new(Point3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Vec3::new(0.0, 555.0, 0.0), green))); // left
    world.add(Box::new(Quad::new(Point3::new(0.0, 0.0, 555.0), Vec3::new(0.0, 0.0, -555.0), Vec3::new(0.0, 555.0, 0.0), red))); // right
    world.add(Box::new(Quad::new(Point3::new(0.0, 555.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), white.clone()))); // floor
    world.add(Box::new(Quad::new(Point3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -555.0), white.clone()))); // floor
    world.add(Box::new(Quad::new(Point3::new(555.0, 0.0, 555.0), Vec3::new(-555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), white.clone()))); // back

    // light
    world.add(Box::new(Quad::new(Point3::new(213.0, 554.0, 227.0), Vec3::new(130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 105.0), light))); // light

    // box 1
    let box1 = Quad::make_box(&Point3::new(0.0, 0.0, 0.0), &Point3::new(165.0, 330.0, 165.0), white.clone());
    let box1 = RotateY::new(Arc::new(box1), 15.0);
    let box1 = Translate::new(Arc::new(box1), Vec3::new(265.0, 0.0, 295.0));

    // box 2
    let box2 = Quad::make_box(&Point3::new(0.0, 0.0, 0.0), &Point3::new(165.0, 165.0, 165.0), white.clone());
    let box2 = RotateY::new(Arc::new(box2), -18.0);
    let box2 = Translate::new(Arc::new(box2), Vec3::new(130.0, 0.0, 65.0));

    world.add(Box::new(box1));
    world.add(Box::new(box2));

    // Camera
    let mut camera = Camera::new();

    camera.aspect_ratio = 1.0;
    camera.image_width = 600;
    camera.samples_per_pixel = 200;
    camera.max_depth = 30;

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

fn cornell_smoke(){
    // World
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));

    world.add(Box::new(Quad::new(Point3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), green))); // left
    world.add(Box::new(Quad::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), red))); // right
    world.add(Box::new(Quad::new(Point3::new(113.0, 554.0, 127.0), Vec3::new(330.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 305.0), light))); // light
    world.add(Box::new(Quad::new(Point3::new(0.0, 555.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), white.clone()))); // floor
    world.add(Box::new(Quad::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), white.clone()))); // floor
    world.add(Box::new(Quad::new(Point3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), white.clone()))); // back

    // add boxes
    let box1 = Quad::make_box(&Point3::new(0.0, 0.0, 0.0), &Point3::new(165.0, 330.0, 165.0), white.clone());
    let box1 = RotateY::new(Arc::new(box1), 15.0);
    let box1 = Translate::new(Arc::new(box1), Vec3::new(265.0, 0.0, 295.0));
    let smoke_box1 = constant_medium::from_color(Arc::new(box1), 0.01, &Color::new(0.0, 0.0, 0.0));

    let box2 = Quad::make_box(&Point3::new(0.0, 0.0, 0.0), &Point3::new(165.0, 165.0, 165.0), white.clone());
    let box2 = RotateY::new(Arc::new(box2), -18.0);
    let box2 = Translate::new(Arc::new(box2), Vec3::new(130.0, 0.0, 65.0));
    let smoke_box2 = constant_medium::from_color(Arc::new(box2), 0.01, &Color::new(1.0, 1.0, 1.0));

    world.add(Box::new(smoke_box1));
    world.add(Box::new(smoke_box2));

    // Camera
    let mut camera = Camera::new();

    camera.aspect_ratio = 1.0;
    camera.image_width = 600;//600;
    camera.samples_per_pixel = 200;//200;
    camera.max_depth = 50;

    camera.vfov = 40.0;
    camera.look_from = Point3::new(278.0, 278.0, -800.0);
    camera.look_at = Point3::new(278.0, 278.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0; // degrees

    camera.background_color = Color::new(0.0, 0.0, 0.0);

    camera.render(&world);
}


fn final_scene(image_width: u32, samples_per_pixel: u32, max_depth: u32){
    //let mut boxes1 = HittableList::new();

    let mut world = HittableList::new();

    let ground = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side{
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_f64_range(1.0, 101.0);
            let z1 = z0 + w;

            world.add(Box::new(Quad::make_box(&Point3::new(x0, y0, z0), &Point3::new(x1, y1, z1), ground.clone())));
        }
    }


    let light = Arc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));
    world.add(Box::new(Quad::new(Point3::new(123.0, 554.0, 147.0), Vec3::new(300.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 265.0), light.clone()))); // light

    // Moving sphere
    let center = Point3::new(400.0, 400.0, 200.0);
    let direction = Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    world.add(Box::new(Sphere::new(Ray::new(center, direction), 50.0, moving_sphere_material.clone())));

    // dielectric (glass) sphere
    let pos1 = Ray::new(Point3::new(260.0, 150.0, 45.0), Vec3::init_zero());
    world.add(Box::new(Sphere::new(pos1, 50.0, Arc::new(Dielectric::new(1.5)))));

    // metal sphere
    let pos2 = Ray::new(Point3::new(0.0, 150.0, 145.0), Vec3::init_zero());
    let albedo = Color::new(0.8, 0.8, 0.9);
    let fuzz = 0.8;
    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
    world.add(Box::new(Sphere::new(pos2, 50.0, sphere_material)));

    
    // blue glass ball with smoke inside
    let boundary = Sphere::new(Ray::new(Point3::new(360.0, 150.0, 145.0), Vec3::init_zero()), 70.0, Arc::new(Dielectric::new(1.5)));
    world.add(Box::new(boundary.clone()));
    let medium1 = constant_medium::from_color(Arc::new(boundary.clone()), 0.2, &Color::new(0.2, 0.4, 0.9));
    world.add(Box::new(medium1));


    // large sphere with low-density smoke cowering entire scene, creating a foggy atmosphere
    let boundary2 = Sphere::new(Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::init_zero()), 5000.0, Arc::new(Dielectric::new(1.5)));
    let medium2 = constant_medium::from_color(Arc::new(boundary2.clone()), 0.0001, &Color::new(1.0, 1.0, 1.0));
    world.add(Box::new(medium2));  

    // Earth sphere with texture mapping
    let earth_texture: Arc<dyn Texture> = Arc::new(ImageTexture::new("textures/earthmap.jpg"));
    let earth_material = Arc::new(Lambertian::from_texture(earth_texture));
    world.add(Box::new(Sphere::new(Ray::new(Point3::new(400.0, 200.0, 400.0), Vec3::init_zero()), 100.0, earth_material.clone())));
    
    // Perlin noise textured sphere
    // this might be slowing down the rendering significantly, especially at higher resolutions and sample counts
    let pertex = Arc::new(NoiseTexture::new(0.2));
    world.add(Box::new(Sphere::new(Ray::new(Point3::new(220.0, 280.0, 300.0), Vec3::init_zero()), 80.0, Arc::new(Lambertian::from_texture(pertex)))));

    // Cluster of small spheres
    let mut boxes2 = HittableList::new();

    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));

    let ns = 1000;
    for _ in 0..ns {
        let random_position = Point3::new(
            random_f64_range(0.0, 165.0),
            random_f64_range(0.0, 165.0),
            random_f64_range(0.0, 165.0),
        );
        boxes2.add(Box::new(Sphere::new(
            Ray::new(random_position, Vec3::init_zero()),
            10.0,
            white.clone(),
        )));
    }

    // Build a BVH for the cluster BEFORE rotating/translating it
    let boxes2_bvh = Arc::new(BVHNode::new(&boxes2));

    world.add(Box::new(
        Translate::new(
            Arc::new(RotateY::new(boxes2_bvh, 15.0)),
            Vec3::new(-100.0, 270.0, 395.0),
        )
    ));

    // Camera
    let mut camera = Camera::new();

    camera.aspect_ratio = 1.0;
    camera.image_width = image_width;
    camera.samples_per_pixel = samples_per_pixel;
    camera.max_depth = max_depth;

    camera.vfov = 40.0;
    camera.look_from = Point3::new(478.0, 278.0, -600.0);
    camera.look_at = Point3::new(278.0, 278.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.background_color = Color::new(0.0, 0.0, 0.0);

    camera.defocus_angle = 0.0; // degrees

    let root = BVHNode::new(&world); // create BVH for the entire scene
    //camera.render(&world);
    camera.render(&root);

}

fn metaballs(){

    let mut world = HittableList::new();
    
    let mat = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 0.3));
    let metaballs1 = Metaballs::new(
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(2.0, 0.0, 0.0),
            Point3::new(4.0, 0.0, 0.0),
        ],
        vec![1.0, 0.9, 0.8],
        1.0, // threshold
        mat,
    );
    world.add(Box::new(metaballs1));

    let mat2 = Arc::new(Metal::new(Color::new(0.4, 0.4, 0.1), 1.0));
    let metaballs2 = Metaballs::new(
        vec![
            Point3::new(0.0, 3.0, 0.0),
            Point3::new(2.0, 3.0, 0.0),
            Point3::new(4.0, 3.0, 0.0),
        ],
        vec![2.0, 1.0, 0.5],
        3.0, // threshold
        mat2,
    );
    world.add(Box::new(metaballs2));

    // Camera
    let mut camera = Camera::new();

    camera.aspect_ratio = 1.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 50;
    camera.max_depth = 10;

    camera.vfov = 80.0;
    camera.look_from = Point3::new(0.0, 0.0, -10.0);
    camera.look_at = Point3::new(0.0, 0.0, -1.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0; // degrees

    camera.background_color = Color::new(0.7, 0.8, 1.0); // light blue background

    let root = BVHNode::new(&world); // create BVH for the entire scene
    //camera.render(&world);
    camera.render(&root);
}


fn cornell_teardrops() {
    // World
    let mut world = HittableList::new();

    // Cornell materials
    let red   = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));

    // Light
    let light = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));
    world.add(Box::new(Quad::new(
        Point3::new(213.0, 554.0, 227.0),
        Vec3::new(130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 105.0),
        light,
    )));

    // Cornell box walls
    world.add(Box::new(Quad::new(Point3::new(555.0, 0.0,   0.0), Vec3::new(0.0, 0.0, 555.0), Vec3::new(0.0, 555.0, 0.0), green))); // left
    world.add(Box::new(Quad::new(Point3::new(0.0,   0.0, 555.0), Vec3::new(0.0, 0.0,-555.0), Vec3::new(0.0, 555.0, 0.0), red)));   // right
    world.add(Box::new(Quad::new(Point3::new(0.0,   0.0,   0.0), Vec3::new(555.0,0.0, 0.0),   Vec3::new(0.0, 0.0, 555.0), white.clone()))); // floor
    world.add(Box::new(Quad::new(Point3::new(0.0, 555.0,   0.0), Vec3::new(555.0,0.0, 0.0),   Vec3::new(0.0, 0.0, 555.0), white.clone()))); // ceiling
    world.add(Box::new(Quad::new(Point3::new(0.0,   0.0, 555.0), Vec3::new(555.0,0.0, 0.0),   Vec3::new(0.0, 555.0, 0.0), white.clone()))); // back

    // --- Teardrops (metaballs) ---
    // We will "scale up" your small teardrop by choosing Cornell-sized centers/radii directly.
    let matte_blue  = Arc::new(Lambertian::new(Color::new(0.2, 0.4, 0.9)));
    let matte_red   = Arc::new(Lambertian::new(Color::new(0.9, 0.1, 0.1)));
    let glass = Arc::new(Dielectric::new(1.5));
    let metal = Arc::new(Metal::new(Color::new(0.9, 0.0, 0.1), 0.1));

    // Left teardrop: glass boundary + blue smoke inside
    // Shape: 3 metaballs stacked vertically, radii decreasing to form a teardrop-ish taper.
    let tear_left = Metaballs::new(
        vec![
            Point3::new(185.0,  75.0, 280.0), // bottom
            Point3::new(185.0, 180.0, 280.0), // mid
            Point3::new(185.0, 220.0, 280.0), // top
        ],
        vec![50.0, 18.0, 6.0],
        0.55, // threshold (keep as 1.0 for your current field definition)
        glass.clone(),
    );
    world.add(Box::new(tear_left.clone()));

    // Blue fog inside the glass teardrop
    // Density: Cornell box scale usually wants smaller densities than your tiny -2..2 scene.
    // Start around 0.02–0.06. Increase for thicker color.
    let blue_smoke = constant_medium::from_color(
        Arc::new(tear_left.clone()),
        0.9,
        &Color::new(0.2, 0.4, 0.9),
    );
    
    world.add(Box::new(blue_smoke));

    
    // Right teardrop: metal (no smoke)
    let tear_right = Metaballs::new(
        vec![
            Point3::new(370.0,  100.0, 280.0),
            Point3::new(370.0, 200.0, 280.0),
            Point3::new(370.0, 245.0, 280.0),
        ],
        vec![50.0, 20.0, 4.0],
        0.4, // higher it is the more compact the ball
        metal.clone(),
    );
    world.add(Box::new(tear_right));
    

    // Camera (Cornell-style)
    let mut camera = Camera::new();
    camera.aspect_ratio = 1.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100; // volumes need samples; 200 is ok for preview
    camera.max_depth = 30;          // glass + volume needs depth

    camera.vfov = 40.0;
    camera.look_from = Point3::new(278.0, 278.0, -800.0);
    camera.look_at   = Point3::new(278.0, 278.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;
    camera.background_color = Color::new(0.0, 0.0, 0.0);

    // BVH
    let root = BVHNode::new(&world);
    camera.render(&root);
}

fn final_scene_rain_metaballs(image_width: u32, samples_per_pixel: u32, max_depth: u32) {
    let mut world = HittableList::new();

    // --- Ground box grid (same as final_scene) ---
    let ground = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_f64_range(1.0, 101.0);
            let z1 = z0 + w;

            world.add(Box::new(Quad::make_box(
                &Point3::new(x0, y0, z0),
                &Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    // --- Light (same as final_scene) ---
    let light = Arc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));
    world.add(Box::new(Quad::new(
        Point3::new(100.0, 554.0, 100.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 300.0),
        light.clone(),
    )));

    // large sphere with low-density smoke cowering entire scene, creating a foggy atmosphere
    let boundary2 = Sphere::new(Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::init_zero()), 5000.0, Arc::new(Dielectric::new(1.5)));
    let medium2 = constant_medium::from_color(Arc::new(boundary2.clone()), 0.0001, &Color::new(0.9, 0.9, 0.9));
    world.add(Box::new(medium2)); 


    let metal = Arc::new(Metal::new(Color::new(0.9, 0.0, 0.1), 0.1));
    // Right teardrop: metal (no smoke)
    let tear_right = Metaballs::new(
        vec![
            Point3::new(200.0,  200.0, 300.0),
            Point3::new(200.0, 300.0, 300.0),
            Point3::new(200.0, 345.0, 300.0),
        ],
        vec![50.0, 20.0, 4.0],
        0.4, // higher it is the more compact the ball
        metal.clone(),
    );
    world.add(Box::new(tear_right));


    // ============================================================
    // RAIN METABALLS (glass + blue smoke) in final_scene setting
    // ============================================================

    let glass = Arc::new(Dielectric::new(1.5));
    let fog_color = Color::new(0.2, 0.4, 0.9);

    // proportions copied from your metal teardrop:
    // y offsets: 0, 100, 145 ; radii: 50, 20, 4 ; threshold: 0.4
    let base_offsets = [0.0, 100.0, 145.0];
    let base_radii   = [50.0, 20.0,  4.0];
    let threshold = 0.4;

    let mut rain = HittableList::new();

    // "Hundreds"
    let num_drops = 400;

    // Place rain roughly above the "interesting" area of final_scene.
    // Most of your action is around x,z ~ [0..500], but we widen it.
    let x_min = -200.0;
    let x_max = 700.0;
    let z_min = 100.0;
    let z_max = 700.0;

    // Put them high, falling downward through the scene volume (static distribution for now)
    let y_min = 0.0;
    let y_max = 500.0;

    for _ in 0..num_drops {
        // small droplet scale
        let s = random_f64_range(0.06, 0.16);

        let x = random_f64_range(x_min, x_max);
        let z = random_f64_range(z_min, z_max);
        let y = random_f64_range(y_min, y_max);

        // slight tilt/sway so they aren't perfectly vertical
        let sway_x = random_f64_range(-8.0, 8.0) * s;
        let sway_z = random_f64_range(-8.0, 8.0) * s;

        let centers = vec![
            Point3::new(x + 0.0 * sway_x, y + base_offsets[0] * s, z + 0.0 * sway_z),
            Point3::new(x + 0.5 * sway_x, y + base_offsets[1] * s, z + 0.5 * sway_z),
            Point3::new(x + 1.0 * sway_x, y + base_offsets[2] * s, z + 1.0 * sway_z),
        ];

        let radii = vec![
            base_radii[0] * s,
            base_radii[1] * s,
            base_radii[2] * s,
        ];

        let boundary = Metaballs::new(centers, radii, threshold, glass.clone());
        rain.add(Box::new(boundary.clone()));

        // IMPORTANT: small objects need higher density to be visible (short path length).
        // Keep it bounded so it doesn't turn into solid paint.
        let mut density = 0.25 / s;
        density = density.clamp(0.8, 6.0);

        let medium = constant_medium::from_color(Arc::new(boundary), density, &fog_color);
        rain.add(Box::new(medium));
    }

    // BVH for rain to keep it fast
    let rain_bvh = BVHNode::new(&rain);
    world.add(Box::new(rain_bvh));

    // --- Camera (same as final_scene) ---
    let mut camera = Camera::new();
    camera.aspect_ratio = 1.0;
    camera.image_width = image_width;
    camera.samples_per_pixel = samples_per_pixel;
    camera.max_depth = max_depth;

    camera.vfov = 40.0;
    camera.look_from = Point3::new(478.0, 278.0, -600.0);
    camera.look_at = Point3::new(278.0, 278.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.background_color = Color::new(0.0, 0.0, 0.0);
    camera.defocus_angle = 0.0;

    let root = BVHNode::new(&world);
    camera.render(&root);
}

fn solar_system(image_width: u32, samples_per_pixel: u32, max_depth: u32) {
    let mut world = HittableList::new();

    // Textbook Diagram Scaling Constants
    // ----------------------------------
    // 1. Size Scaling: Square Root
    //    We use the square root of the relative radius. This makes the Sun manageable (~10x Earth)
    //    and the Gas Giants clearly larger than Earth but not 11x larger.
    let earth_radius_base = 2.5;
    
    // 2. Distance Scaling: Logarithmic
    //    We use Log10(AU) to space the planets. This separates the inner planets 
    //    and brings the outer planets closer, a standard technique in scientific visualization.
    //    Formula: Base_Offset + (Log10(AU) * Spread_Factor)
    let dist_spread_factor = 70.0;
    let dist_base_offset = 50.0; // Distance from Sun center to Mercury's orbit start

    // spacedust to brigthen up the scene and make the orbits more visible
    //let spacedust_ball = Sphere::new(Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::init_zero()), 500.0, Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73))));
    //let spacedust = constant_medium::from_color(Arc::new(spacedust_ball), 0.01, &Color::new(0.73, 0.73, 0.73));
    //world.add(Box::new(spacedust));


    // The Sun
    // Real radius is 109x Earth. Diagram radius is ~10.4x Earth (sqrt(109)).
    let sun_radius = 109.0f64.sqrt() * earth_radius_base;
    let sun_light = Arc::new(DiffuseLight::new(Color::new(255.0, 255.0, 94.0))); 
    world.add(Box::new(Sphere::new(Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::init_zero()), sun_radius, sun_light)));

    // Helper closure to add planets
    let mut add_planet = |world: &mut HittableList, dist_au: f64, radius_rel: f64, texture_path: &str| {
        // Size: Sqrt(Relative) * Base
        let radius = radius_rel.sqrt() * earth_radius_base * 2.0; // multiplied by 2 to make Earth planets more visible

        // Distance: Logarithmic scaling
        // log10(0.39) is ~ -0.41. We shift this so Mercury starts at dist_base_offset.
        // We add 0.41 to normalize Mercury to near-zero before scaling.
        let log_au = dist_au.log10(); 
        let distance = dist_base_offset + (log_au + 0.41) * dist_spread_factor;

        // Helper debug print
        let planet_name = texture_path.split('/').last().unwrap_or("unknown");
        eprint!("Adding {}: AU={:.2} -> LogDist={:.1}, RelRad={:.2} -> VisRad={:.1}\n", 
            planet_name, dist_au, distance, radius_rel, radius);

        // Random orbital angle
        //let angle = random_f64_range(0.0, 2.0 * std::f64::consts::PI);
        //let x = distance * angle.cos();
        //let z = distance * angle.sin();

        let angle_r = (radius / 2.0) % (2.0 * std::f64::consts::PI) - std::f64::consts::PI / 2.0; // use radius to get a consistent angle for each planet
        eprintln!("Angle_r: {:.2}", angle_r);
        let x = distance * angle_r.cos();
        let z = distance * angle_r.sin();

        // lined up along x-axis for better visibility of textures and orbits
        //let x = distance;
        //let z: f64 = 0.0;

        let pos = Point3::new(x, 0.0, z);
        
        let tex = Arc::new(ImageTexture::new(texture_path));
        let mat = Arc::new(Lambertian::from_texture(tex));
        
        world.add(Box::new(Sphere::new(Ray::new(pos, Vec3::init_zero()), radius, mat)));
    };

    // Inner Planets
    add_planet(&mut world, 0.39, 0.38, "textures/2k_mercury.jpg");       // Mercury
    add_planet(&mut world, 0.72, 0.95, "textures/2k_venus_surface.jpg"); // Venus
    add_planet(&mut world, 1.00, 1.00, "textures/earthmap.jpg");         // Earth
    add_planet(&mut world, 1.52, 0.53, "textures/2k_mars.jpg");          // Mars

    // Outer Planets
    add_planet(&mut world, 5.20, 11.2, "textures/2k_jupiter.jpg");       // Jupiter
    add_planet(&mut world, 9.58, 9.45, "textures/2k_saturn.jpg");        // Saturn
    add_planet(&mut world, 19.2, 4.00, "textures/2k_uranus.jpg");        // Uranus
    add_planet(&mut world, 30.1, 3.88, "textures/2k_neptune.jpg");       // Neptune

    // Camera setup
    let mut camera = Camera::new();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = image_width;
    camera.samples_per_pixel = samples_per_pixel;
    camera.max_depth = max_depth;

    // Position camera high up and back to see the entire orbital plane
    // With log scaling, Neptune is at ~185 units, Sun is at 0.
    camera.vfov = 45.0;
    camera.look_from = Point3::new(0.0, 200.0, 300.0); 
    camera.look_at = Point3::new(0.0, 0.0, -1.0);      // Look slightly offset to center the orbits
    camera.vup = Vec3::new(0.0, 1.0, 0.0);             // Y-up is standard for this scene

    camera.background_color = Color::new(0.001, 0.001, 0.001); // Deep space grey/black
    camera.defocus_angle = 0.0; 

    let root = BVHNode::new(&world);
    camera.render(&root);
}


fn main() {
    let option = 13;

    match option {
        1 => bouncing_spheres(),
        2 => checkered_sphere(),
        3 => earth(),
        4 => perlin_sphere(),
        5 => quads(),
        6 => simple_light(),
        7 => conrell_box(),
        8 => cornell_smoke(),
        9 => final_scene(800, 10000, 40), // took over an hour
        10 => metaballs(),
        11 => cornell_teardrops(),
        12 => final_scene_rain_metaballs(800, 5000, 40), // 800, 5000, 40 ~2hours
        13 => solar_system(1200, 2000, 100), // 800, 5000, 500 ~ 1 min / 2400, 20k, 1k ~2min
        _ => { eprintln!("running scene default\n");
            final_scene(400, 500, 10);} // ~ less than a minute
    }
}
