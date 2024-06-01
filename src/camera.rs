use std::time::Instant;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::sphere::Sphere;

use crate::utils::write_img;
use crate::utils::clamp;
use crate::utils::sample_sqr;
use crate::utils::random_in_unit_disk;
use crate::utils::ray_color;

use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use palette::Pixel;
use palette::Srgb;

pub struct Camera {
    pub aspect_ratio: f64,
    pub img_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: f64,
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,

    img_height: i32,
    pixel_samples_scale: f64,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    fn defocus_disk_sample(&self) -> Vec3 {
        let p = random_in_unit_disk();
        
        self.center + (self.defocus_disk_u * p.x()) + (self.defocus_disk_v * p.y())
    }

    fn render_line(
        &self,
        pixels: &mut [u8],
        hit_world: &Vec<Sphere>,
        i: usize,
    ) {
        let w = self.img_width as usize;
    
        for j in 0..w {
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            let samples = self.samples_per_pixel;

            for _ in 0..samples {
                let ray = self.get_ray(j as i32, i as i32);

                color = color + ray_color(&ray, &hit_world, self.max_depth);
            }

            color = color * self.pixel_samples_scale;

            let color_clamped = Vec3::new(
                clamp(color.x()).sqrt(),
                clamp(color.y()).sqrt(),
                clamp(color.z()).sqrt(),
            );
    
            let srgb_color = Srgb::new(
                color_clamped.x() as f32,
                color_clamped.y() as f32,
                color_clamped.z() as f32,
            );
            let pixel: [u8; 3] = srgb_color.into_format().into_raw();
    
            pixels[j * 3] = pixel[0];
            pixels[j * 3 + 1] = pixel[1];
            pixels[j * 3 + 2] = pixel[2];
        }
    }
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        img_width: i32,
        samples_per_pixel: i32,
        max_depth: i32,
        vfov: f64,
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Camera {
        let img_height = (((img_width as f64) / aspect_ratio) as i32).max(1);
        let pixel_samples_scale = 1.0 / (samples_per_pixel as f64);
        let center = lookfrom;
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewpoint_height = 2.0 * h * focus_dist;
        let viewpoint_width = viewpoint_height * ((img_width as f64) / (img_height as f64));
        let w = (lookfrom - lookat).unit_vec();
        let u = vup.cross(&w).unit_vec();
        let v = w.cross(&u);
        let viewpoint_u = u * viewpoint_width;
        let viewpoint_v = -v * viewpoint_height;
        let pixel_delta_u = viewpoint_u / (img_width as f64);
        let pixel_delta_v = viewpoint_v / (img_height as f64);
        let viewpoint_upper_left = center - (w * focus_dist) - viewpoint_u / 2.0 - viewpoint_v / 2.0;
        let pixel00_loc = viewpoint_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;
        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            aspect_ratio,
            img_width,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
            img_height,
            pixel_samples_scale,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = sample_sqr();
        let pixel_sample = self.pixel00_loc + (self.pixel_delta_u * ((i as f64) + offset.x())) + (self.pixel_delta_v * ((j as f64) + offset.y()));
        let ray_orig = if self.defocus_angle <= 0.0 { self.center } else { self.defocus_disk_sample() };
        let ray_dir = pixel_sample - ray_orig;

        Ray::new(ray_orig, ray_dir)
    }
}

impl Camera {
    pub fn render(&self, hit_world: &Vec<Sphere>) {
        let mut pixels = vec![0; (self.img_width * self.img_height * 3) as usize];
        let bands: Vec<(usize, &mut [u8])> = pixels.chunks_mut((self.img_width * 3) as usize).enumerate().collect();

        let start = Instant::now();

        bands.into_par_iter().for_each(
            |(i, band)| {
                self.render_line(
                    band,
                    &hit_world,
                    i,
                );
            }
        );

        println!("Render time: {}s.", start.elapsed().as_secs());

        write_img("rust_raytracer.png", &pixels, (self.img_width as usize, self.img_height as usize)).expect("err writing img");
    }
}