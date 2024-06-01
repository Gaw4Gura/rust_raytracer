use rand::Rng;

use rust_raytracer::vec3::Vec3;
use rust_raytracer::sphere::Sphere;
use rust_raytracer::materials::Materials;
use rust_raytracer::materials::Lambertian;
use rust_raytracer::materials::Metal;
use rust_raytracer::materials::Glass;
use rust_raytracer::camera::Camera;

fn main() {
    let cam = Camera::new(
        16.0 / 9.0,
        2560,
        512,
        50,
        20.0,
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.6,
        10.0,
    );

    let mut hit_world: Vec<Sphere> = vec![];    
    let ground = Materials::Lambertian(
        Lambertian::new(
            Vec3::new(0.5, 0.5, 0.5),
        ),
    );

    hit_world.push(
        Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            ground,
        ),
    );

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Vec3::new(
                (a as f64) + 0.9 * rng.gen::<f64>(),
                0.2,
                (b as f64) + 0.9 * rng.gen::<f64>(),
            );
            let reference = Vec3::new(4.0, 0.2, 0.0);

            if (center - reference).len() > 0.9 {
                if choose_mat < 0.8 {
                    let mat = Materials::Lambertian(
                        Lambertian::new(
                            Vec3::new(
                                rng.gen::<f64>() * rng.gen::<f64>(),
                                rng.gen::<f64>() * rng.gen::<f64>(),
                                rng.gen::<f64>() * rng.gen::<f64>(),
                            ),
                        ),
                    );

                    hit_world.push(
                        Sphere::new(center, 0.2, mat),
                    );
                } else if choose_mat < 0.95 {
                    let mat = Materials::Metal(
                        Metal::new(
                            Vec3::new(
                                rng.gen_range(0.5..1.0),
                                rng.gen_range(0.5..1.0),
                                rng.gen_range(0.5..1.0),
                            ),
                            rng.gen_range(0.0..0.5),
                        ),
                    );

                    hit_world.push(
                        Sphere::new(center, 0.2, mat),
                    );
                } else {
                    let mat = Materials::Glass(
                        Glass::new(1.5),
                    );

                    hit_world.push(
                        Sphere::new(center, 0.2, mat),
                    );
                }
            } else {
                continue;
            }
        }
    }

    let glass = Materials::Glass(
        Glass::new(1.5),
    );

    hit_world.push(
        Sphere::new(
            Vec3::new(0.0, 1.0, 0.0),
            1.0,
            glass,
        ),
    );

    let lambertian = Materials::Lambertian(
        Lambertian::new(
            Vec3::new(0.4, 0.2, 0.1),
        ),
    );

    hit_world.push(
        Sphere::new(
            Vec3::new(-4.0, 1.0, 0.0),
            1.0,
            lambertian,
        ),
    );

    let metal = Materials::Metal(
        Metal::new(
            Vec3::new(0.7, 0.6, 0.5),
            0.0,
        ),
    );

    hit_world.push(
        Sphere::new(
            Vec3::new(4.0, 1.0, 0.0),
            1.0,
            metal,
        ),
    );

    cam.render(&hit_world);

    println!("Hello, world!");
}
