use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let t1 = (-b - discriminant.sqrt()) / a;
        if t_min <= t1 && t1 <= t_max {
            let p = ray.point_at_parameter(t1);
            return Some(HitRecord {
                t: t1,
                p,
                normal: (p - self.center) / self.radius,
                material: self.material.as_ref(),
            });
        }
        let t2 = (-b + discriminant.sqrt()) / a;
        if t_min <= t2 && t2 <= t_max {
            let p = ray.point_at_parameter(t2);
            return Some(HitRecord {
                t: t2,
                p,
                normal: (p - self.center) / self.radius,
                material: self.material.as_ref(),
            });
        }
        None
    }
}

pub struct HitList {
    items: Vec<Box<dyn Hit>>,
}

impl HitList {
    pub fn new(items: Vec<Box<dyn Hit>>) -> HitList {
        HitList { items }
    }
}

impl Hit for HitList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut result = None;
        for item in self.items.iter() {
            if let Some(record) = item.hit(ray, t_min, closest_so_far) {
                closest_so_far = record.t;
                result = Some(record);
            }
        }
        result
    }
}
