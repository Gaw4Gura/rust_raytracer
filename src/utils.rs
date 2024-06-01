use image::png::PNGEncoder;
use image::ColorType;
use std::fs::File;
use rand::Rng;

use crate::materials::Scatterable;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::ray::HitRecord;
use crate::ray::Hittable;
use crate::sphere::Sphere;

pub fn write_img(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize)
) -> Result<(), std::io::Error> {
    let png_file = File::create(filename)?;
    let png_enc = PNGEncoder::new(png_file);

    png_enc.encode(
        pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::RGB(8)
    )?;

    Ok(())
}

pub fn clamp(x: f64) -> f64 {
    if x < 0.0 { return 0.0 };
    if x > 1.0 { return 1.0 };

    x
}

pub fn sample_sqr() -> Vec3 {
    let mut rng = rand::thread_rng();

    Vec3::new(
        rng.gen_range(-0.5..0.5),
        rng.gen_range(-0.5..0.5),
        0.0,
    )
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let mut rng = rand::thread_rng();
        let p = Vec3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            0.0,
        );

        if p.len_sqr() < 1.0 { return p; }
    }
}

fn hit_record<'materials>(hit_world: &'materials Vec<Sphere>, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord<'materials>> {
    let mut closest = ray_tmax;
    let mut hit_record = None;

    for sphere in hit_world {
        if let Some(hit) = sphere.hit(&ray, ray_tmin, closest) {
            closest = hit.t;
            hit_record = Some(hit);
        }
    }

    hit_record
}

pub fn ray_color(ray: &Ray, hit_world: &Vec<Sphere>, depth: i32) -> Vec3 {
    if depth <= 0 { return Vec3::new(0.0, 0.0, 0.0); }

    let hit = hit_record(
        &hit_world,
        &ray,
        0.001,
        f64::MAX,
    );

    match hit {
        Some(hit_record) => {
            let scattered = hit_record.mat.scatter(&ray, &hit_record);

            match scattered {
                Some((sr, albedo)) => {
                    match sr {
                        Some(sr) => {
                            let color = ray_color(&sr, &hit_world, depth - 1);

                            return color * albedo;
                        }
                        None => {
                            return albedo;
                        }
                    }
                }
                None => {
                    return Vec3::new(0.0, 0.0, 0.0);
                }
            }
        }
        None => {
            let unit_dir = ray.dir.unit_vec();
            let a = 0.5 * (unit_dir.y() + 1.0);

            return Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a;
        }
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n * v.dot(&n) * 2.0
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = -uv.dot(&n).min(1.0);
    let r_out_perp = (*uv + *n * cos_theta) * etai_over_etat;
    let r_out_parallel = *n * -(1.0 - r_out_perp.len_sqr()).abs().sqrt();

    r_out_perp + r_out_parallel
}

pub fn reflectance(cos_theta: f64, refract_idx: f64) -> f64 {
    let r0 = (1.0 - refract_idx) / (1.0 + refract_idx);
    let r0_sqr = r0 * r0;

    r0_sqr + (1.0 - r0_sqr) * (1.0 - cos_theta).powi(5)
}