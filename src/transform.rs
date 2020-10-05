use bevy::prelude::Transform;

pub trait TransformUpgrade {
    fn facing(&self, forward: Vec3) -> Vec3;
}

impl TransformUpgrade for Transform {
    fn facing(&self, forward: Vec3) -> Vec3 {
        self.rotation().mul_vec3(forward)
    }
}