pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod camera;
pub mod rand;
pub mod material;
pub mod lambertian;
pub mod metal;
pub mod dielectric;

pub use crate::vec3::vec3::Vec3;//文件位置::mod名::struct名
pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::ray::Ray;
pub use crate::hittable::Hitrecord;
pub use crate::sphere::Sphere;
pub use crate::hittable_list::Object;
pub use crate::hittable_list::Hittablelist;
pub use crate::hittable::Hit;
pub use crate::camera::Camera;
pub use crate::material::Material;
pub use crate::material::Scatter;
pub use crate::lambertian::Lambertian;
pub use crate::metal::Metal;
pub use crate::dielectric::Dielectric;


pub fn ray_color(r:&Ray,world:&Hittablelist,depth:i32) -> Color {
    let mut rec=Hitrecord::default_new();
    let inf:f64=1.79769e+308;

    if depth<=0 {return Color::new(0.0,0.0,0.0);}

    if world.hit(&r,0.001,inf,&mut rec) {
        let mut scattered=Ray::default_new();
        let mut attenuation=Color::default_new();
        if let Some(in_mat_ptr) = &rec.mat_ptr {
            if in_mat_ptr.scatter(&r,&rec,&mut attenuation,&mut scattered) {
                return ray_color(&scattered,&world,depth-1)*attenuation;
            } else {
                return Color::new(0.0,0.0,0.0);
            }
        }
        
    }
    let unit_direction=Vec3::unit_vector(&r.direction());
    let t=0.5*(unit_direction.y()+1.0);
    Color::new(1.0,1.0,1.0)*(1.0-t)+Color::new(0.5,0.7,1.0)*t
}

pub fn random_scene() -> Hittablelist {
    let mut world=Hittablelist::default_new();

    let ground_material=Some(Box::new(Material::Lambertian(Lambertian::new(&Color::new(0.5,0.5,0.5)))));
    world.add(Box::new(Object::Sphere(Sphere::new(&Point3::new(0.0,-1000.0,0.0),1000.0,&ground_material))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat=rand::random_double();
            let center=Point3::new((a as f64)+0.9*rand::random_double(),0.2,(b as f64)+0.9*rand::random_double());

            if (center.copy()-Point3::new(4.0,0.2,0.0)).length()>0.9 {
                if choose_mat<0.8 {
                    let albedo=Color::random()*Color::random();
                    let sphere_material=Some(Box::new(Material::Lambertian(Lambertian::new(&albedo.copy()))));
                    world.add(Box::new(Object::Sphere(Sphere::new(&center.copy(),0.2,&sphere_material))));
                } else if choose_mat<0.95 {
                    let albedo=Color::random_between(0.5,1.0);
                    let fuzz=rand::random_double_between(0.0,0.5);
                    let sphere_material=Some(Box::new(Material::Metal(Metal::new(&albedo,fuzz))));
                    world.add(Box::new(Object::Sphere(Sphere::new(&center.copy(),0.2,&sphere_material))));
                } else {
                    let sphere_material=Some(Box::new(Material::Dielectric(Dielectric::new(1.5))));
                    world.add(Box::new(Object::Sphere(Sphere::new(&center.copy(),0.2,&sphere_material))));
                }
            }
        }
    }

    let material1=Some(Box::new(Material::Dielectric(Dielectric::new(1.5))));
    world.add(Box::new(Object::Sphere(Sphere::new(&Point3::new(0.0,1.0,0.0),1.0,&material1))));

    let material2=Some(Box::new(Material::Lambertian(Lambertian::new(&Color::new(0.4,0.2,0.1)))));
    world.add(Box::new(Object::Sphere(Sphere::new(&Point3::new(-4.0,1.0,0.0),1.0,&material2))));

    let material3=Some(Box::new(Material::Metal(Metal::new(&Color::new(0.7,0.6,0.5),0.0))));
    world.add(Box::new(Object::Sphere(Sphere::new(&Point3::new(4.0,1.0,0.0),1.0,&material3))));

    world
}

fn main() {

    //Image
    let aspect_ratio = 3.0/2.0;
    let image_width = 1200;
    let image_height = ((image_width as f64)/aspect_ratio) as i32;
    let samples_per_pixel = 5;//记得改成500
    let max_depth = 50;

    //World
    let world=random_scene();
    /*
    let r=(std::f64::consts::PI/4.0).cos();
    let mut world=Hittablelist::default_new();

    let material_ground=Some(Box::new(Material::Lambertian(Lambertian::new(&Color::new(0.8,0.8,0.0)))));
    let material_center=Some(Box::new(Material::Lambertian(Lambertian::new(&Color::new(0.1,0.2,0.5)))));
    //let material_center=Some(Box::new(Material::Dielectric(Dielectric::new(1.5))));
    let material_left=Some(Box::new(Material::Dielectric(Dielectric::new(1.5))));
    //let material_left=Some(Box::new(Material::Lambertian(Lambertian::new(&Color::new(0.0,0.0,1.0)))));
    let material_right=Some(Box::new(Material::Metal(Metal::new(&Color::new(0.8,0.6,0.2),0.0))));

    world.add(Box::new(Object::Sphere(Sphere::new(&Point3::new(0.0,-100.5,-1.0),100.0,&material_ground))));
    world.add(Box::new(Object::Sphere(Sphere::new(&Point3::new(0.0,0.0,-1.0),0.5,&material_center))));
    world.add(Box::new(Object::Sphere(Sphere::new(&Point3::new(-1.0,0.0,-1.0),0.5,&material_left))));
    world.add(Box::new(Object::Sphere(Sphere::new(&Point3::new(-1.0,0.0,-1.0),-0.45,&material_left))));
    world.add(Box::new(Object::Sphere(Sphere::new(&Point3::new(1.0,0.0,-1.0),0.5,&material_right))));
    */

    //Camera
    let lookfrom=Point3::new(13.0,2.0,3.0);
    let lookat=Point3::new(0.0,0.0,0.0);
    let vup=Vec3::new(0.0,1.0,0.0);
    let dist_to_focus=10.0;
    let aperture=0.1;
    let cam=Camera::new(&lookfrom,&lookat,&vup,20.0,aspect_ratio,aperture,dist_to_focus);

    //Render
    print!("P3\n{} {}\n255\n", image_width, image_height);
    
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let mut pixel_color=Color::new(0.0,0.0,0.0);
            for s in 0..samples_per_pixel {
                let u=(((i as f64) + rand::random_double())/((image_width-1) as f64)) as f64;
                let v=(((j as f64) + rand::random_double())/((image_height-1) as f64)) as f64;
                let r=cam.get_ray(u,v);
                pixel_color+=ray_color(&r,&world,max_depth);
            }
            color::write_color(pixel_color,samples_per_pixel);
        }
    }
    
    eprint!("\nDone.\n");
}
/*
fn hit_sphere(center:&Point3,radius:f64,r:&Ray) -> f64 {
    let center_copy=Point3::copy_new(&center);
    let oc=r.origin()-center_copy;
    let a=r.direction().length_squared();
    let half_b=Vec3::dot(&oc,&r.direction());
    let c=oc.length_squared()-radius*radius;
    let discriminant=half_b*half_b-a*c;
    if discriminant<0.0 {
        -1.0
    } else {
        (-half_b-discriminant.sqrt())/a
    }
}
fn ray_color(r:&Ray) -> Color {
    
    let t=hit_sphere(&Point3::new(0.0,0.0,-1.0),0.5,r);
    if t>0.0 {
        let n=Vec3::unit_vector(&(r.at(t)-Vec3::new(0.0,0.0,-1.0)));
        return Color::new(n.x()+1.0,n.y()+1.0,n.z()+1.0)*0.5;
    }
    let unit_direction=Vec3::unit_vector(&r.direction());
    let t=0.5*(unit_direction.y()+1.0);
    Color::new(1.0,1.0,1.0)*(1.0-t)+Color::new(0.5,0.7,1.0)*t
    
    //let mut rec=hit_record::default_new(); 
}
*/
            /*
            let u=((i as f64)/((image_width-1) as f64)) as f64;
            let v=((j as f64)/((image_height-1) as f64)) as f64;

            let origin = Point3::new(0.0,0.0,0.0);
            let horizontal = Vec3::new(viewport_width,0.0,0.0);
            let vertical = Vec3::new(0.0,viewport_height,0.0);
            let lower_left_corner = origin-horizontal/2.0-vertical/2.0-Vec3::new(0.0,0.0,focal_length);

            let origin = Point3::new(0.0,0.0,0.0);
            let origin_copy = Point3::new(0.0,0.0,0.0);
            let horizontal = Vec3::new(viewport_width,0.0,0.0);
            let vertical = Vec3::new(0.0,viewport_height,0.0);

            let r=Ray::new(&origin,&(lower_left_corner+horizontal*u+vertical*v-origin_copy));
            let pixel_color=ray_color(&r,&world);
            color::write_color(pixel_color);
            */
            /*
            let pixel_color=Color::new((i as f64)/((image_width-1) as f64),(j as f64)/((image_height-1) as f64),0.25);
            color::write_color(pixel_color);
            */