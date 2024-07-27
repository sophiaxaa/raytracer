mod vec3; 
use vec3::{Color, Point3, Vec3, write_color};

pub struct Ray{
    //fields of the ray struct
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray{
    //constructor
    //point3 is just an alias for vec3
    pub fn new(origin: Point3, direction: Vec3) -> Self{
        orig:origin,
        dir:direction,
    }

    //getters
    pub fn origin(&self) -> &Point3{
        &self.orig

    }

    pub fn direction(&self) -> &Vec3{
        &self.dir
    }

    pub fn at(&self, num: f64) -> Point3 {
        self.orig + num * self.dir
    }
}