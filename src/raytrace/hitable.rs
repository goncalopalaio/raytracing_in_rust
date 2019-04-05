use super::material::Material;
use super::ray::Ray;
use super::vec::Vec3;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Box<Material>,
}

pub trait Hitable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
