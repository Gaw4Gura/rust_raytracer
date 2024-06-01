use rand::Rng;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::ray::HitRecord;
use crate::utils::reflect;
use crate::utils::refract;
use crate::utils::reflectance;

#[derive(Debug, Clone)]
pub enum Materials {
    Lambertian(Lambertian),
    Metal(Metal),
    Glass(Glass),
}

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Vec3)>;
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Vec3)> {
        let mut scatter_dir = hit_record.norm + Vec3::random_unit_vec();
        scatter_dir = if scatter_dir.near_zero() { hit_record.norm } else { scatter_dir };
        
        let scattered = Ray::new(hit_record.pt, scatter_dir);
        let attenuation = self.albedo;

        Some((Some(scattered), attenuation))
    }
}

#[derive(Debug, Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Vec3)> {
        let mut reflected = reflect(&ray.dir, &hit_record.norm);
        reflected = reflected.unit_vec() + Vec3::random_unit_vec() * self.fuzz;

        let scattered = Ray::new(hit_record.pt, reflected);
        let attenuation = self.albedo;

        if scattered.dir.dot(&hit_record.norm) > 0.0 { return Some((Some(scattered), attenuation)); }
        
        None
    }
}

#[derive(Debug, Clone)]
pub struct Glass {
    pub refract_idx: f64,
}

impl Glass {
    pub fn new(refract_idx: f64) -> Glass {
        Glass { refract_idx }
    }
}

impl Scatterable for Glass {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Vec3)> {
        let attenuation = Vec3::new(1.0, 1.0,1.0);
        let ri = if hit_record.front_face { 1.0 / self.refract_idx } else { self.refract_idx };
        let dir_unit = ray.dir.unit_vec();
        let cos_theta = -dir_unit.dot(&hit_record.norm).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = if ri * sin_theta > 1.0 { true } else { false };
        let mut rng = rand::thread_rng();
        
        if cannot_refract || reflectance(cos_theta, ri) > rng.gen::<f64>() {
            let dir = reflect(&dir_unit, &hit_record.norm);
            let scattered = Ray::new(hit_record.pt, dir);

            return Some((Some(scattered), attenuation));
        }

        let dir = refract(&dir_unit, &hit_record.norm, ri);
        let scattered = Ray::new(hit_record.pt, dir);

        Some((Some(scattered), attenuation))
    }
}

impl Scatterable for Materials {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Vec3)> {
        match self {
            Materials::Lambertian(l) => { l.scatter(ray, hit_record) }
            Materials::Metal(m) => { m.scatter(ray, hit_record) }
            Materials::Glass(g) => { g.scatter(ray, hit_record) }
        }
    }
}