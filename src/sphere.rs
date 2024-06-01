use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::ray::HitRecord;
use crate::ray::Hittable;
use crate::materials::Materials;

#[cfg(test)]
use crate::materials::Lambertian;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat: Materials,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: Materials) -> Sphere {
        Sphere{ center, radius, mat }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc = self.center - ray.orig;
        let a = ray.dir.len_sqr();
        let h = oc.dot(&ray.dir);
        let c = oc.len_sqr() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant > 0.0 {
            let sqrtd = discriminant.sqrt();
            let root1 = (h - sqrtd) / a;
            let root2 = (h + sqrtd) / a;

            if root1 <= ray_tmax && root1 >= ray_tmin {
                let p = ray.at(root1);
                let mut norm = (p - self.center) / self.radius;
                let front_face = if ray.dir.dot(&norm) < 0.0 { true } else { false };
                
                norm = if front_face { norm } else { -norm };

                return Some(HitRecord{
                    pt: p,
                    norm,
                    t: root1,
                    front_face,
                    mat: &self.mat,
                })
            }

            if root2 <= ray_tmax && root2 >= ray_tmin {
                let p = ray.at(root2);
                let mut norm = (p - self.center) / self.radius;
                let front_face = if ray.dir.dot(&norm) < 0.0 { true } else { false };

                norm = if front_face { norm } else { -norm };

                return Some(HitRecord{
                    pt: p,
                    norm,
                    t: root2,
                    front_face,
                    mat: &self.mat,
                })
            }
        } else if discriminant == 0.0 {
            let sqrtd = discriminant.sqrt();
            let root = (h - sqrtd) / a;

            if root <= ray_tmax && root >= ray_tmin {
                let p = ray.at(root);
                let mut norm = (p - self.center) / self.radius;
                let front_face = if ray.dir.dot(&norm) < 0.0 { true } else { false };

                norm = if front_face { norm } else { -norm };

                return Some(HitRecord{
                    pt: p,
                    norm,
                    t: root,
                    front_face,
                    mat: &self.mat,
                })
            }
        }

        None
    }
}

#[test]
fn test_hit() {
    let sphere = Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
        Materials::Lambertian(Lambertian::new(Vec3::new(0.0, 0.0, 0.0))),
    );
    let ray = Ray::new(
        Vec3::new(0.0, 0.0, -5.0),
        Vec3::new(0.0, 0.0, 1.0),
    );
    let hit = sphere.hit(&ray, 0.0, f64::INFINITY);

    assert_eq!(hit.unwrap().t, 4.0);
}