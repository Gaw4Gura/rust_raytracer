use crate::vec3::Vec3;
use crate::materials::Materials;

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Ray {
        Ray{ orig, dir }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
}

pub struct HitRecord<'material> {
    pub pt: Vec3,
    pub norm: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: &'material Materials,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}

#[test]
fn test_ray_new() {
    let x = Vec3::new(0.1, 0.2, 0.3);
    let y = Vec3::new(0.2, 0.3, 0.4);
    let ray = Ray::new(x, y);

    assert_approx_eq!(ray.orig.x(), 0.1);
    assert_approx_eq!(ray.orig.y(), 0.2);
    assert_approx_eq!(ray.orig.z(), 0.3);
    assert_approx_eq!(ray.dir.x(), 0.2);
    assert_approx_eq!(ray.dir.y(), 0.3);
    assert_approx_eq!(ray.dir.z(), 0.4);
}

#[test]
fn test_ray_at() {
    let x = Vec3::new(0.1, 0.2, 0.3);
    let y = Vec3::new(0.2, 0.3, 0.4);
    let ray = Ray::new(x, y);
    let at = ray.at(0.5);

    assert_approx_eq!(at.x(), 0.2);
    assert_approx_eq!(at.y(), 0.35);
    assert_approx_eq!(at.z(), 0.5);
}