use vec3D::Vec3D;

pub struct Ray {
    origin: Vec3D,
    direction: Vec3D
}

impl Ray {
    pub fn new(orig: Vec3D, dir: Vec3D) -> Self {
        Self {
            origin: orig,
            direction: dir,
        }
    }

    pub fn origin(&self) -> &Vec3D {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3D {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Vec3D {
        return self.origin + self.direction*t;
    }
}