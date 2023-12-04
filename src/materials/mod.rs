use cgmath::{InnerSpace, Matrix3, Rad, Vector3};

use crate::{image::RGB, ray::Ray, scene::{Scene, ObjectRay}};

const REFRACTIVE_INDEX: f32 = 1.5168;

pub enum Material {
    Color(RGB),
    Normal,
    Mirror,
    Glass
}

fn move_ray_refraction(ray: &Vector3<f32>, normal: &Vector3<f32>, n1: f32, n2: f32) -> Vector3<f32> {
    let angle1 = (-*normal).angle(*ray).0;

    let angle2 = (angle1.sin()*n1/n2).asin();
    
    let axis = -normal.cross(*ray);
    let rotation = Matrix3::from_axis_angle(axis.normalize(), Rad(angle2));

    (rotation*normal).normalize()
}
impl Material {
    pub fn get_color(&self, ray: &Ray, normal: &Ray, scene: &Scene, bounce: i32, object: &Box<dyn ObjectRay>) -> RGB {
        match self {
            Material::Color(c) => c.clone(),
            Material::Normal => RGB{
                r: (255.0-normal.dir.x*255.0) as u8,
                g: (normal.dir.y*255.0) as u8,
                b: (normal.dir.z*255.0) as u8,
            },
            Material::Mirror => {
                let point = normal.pos;
                let normal = normal.dir;

                let distance = (point - ray.pos).magnitude();

                let a = ray.pos - point;
                let b = (a.dot(normal)) * normal;
                let c = b-a;
                let d = (b+c).normalize();

                let mut res = scene.launch_ray(Ray{dir: d, pos: point}, bounce);
                res.distance = distance;

                let reduction = 1.0-0.04;
                res.rgb.r = (res.rgb.r as f32 * reduction) as u8;
                res.rgb.g = (res.rgb.g as f32 * reduction) as u8;
                res.rgb.b = (res.rgb.b as f32 * reduction) as u8;
                
                res.rgb
            },
            Material::Glass => {
                let ray_inside = move_ray_refraction(&ray.dir, &-normal.dir, 1.0, REFRACTIVE_INDEX);

                let ray_inter = Ray { pos: ray.pos, dir: ray_inside };

                let ray_outside = match object.intersect(&ray_inter){
                    None => ray_inter,
                    Some(r) => Ray{
                        dir: move_ray_refraction(&ray_inter.dir, &normal.dir, REFRACTIVE_INDEX, 1.0),
                        pos: r.pos,
                    }
                };

                scene.launch_ray(ray_outside, bounce).rgb
            }
        }
    }
}