mod vector3;
mod ray;
mod color;
//mod color; // do we need this class? me thinks yes but idk

use ray::Ray;
use vector3::Vector3;
use color:: Color;
use color::Print;



/**
 *
 * Be sure to run ./bin/make_image.sh and match it up to the picture in
 * raytracing in one weekend. 
 *
 * Current stopping point is Section 6
 */


fn dot_product(vec1 :&Vector3<f64>, vec2: &Vector3<f64>) -> f64 {
    return (vec1.x * vec2.x) +
(vec1.y * vec2.y) +
(vec1.z * vec2.z) 
}

fn unit_vector(vec: Vector3<f64> ) -> Vector3<f64> {
    let x_2 = vec.x * vec.x;
    let y_2 = vec.y * vec.y;
    let z_2 = vec.z * vec.z;
    let all_2 = x_2 + y_2 + z_2;

    let length = all_2.sqrt();
    return Vector3::new(vec.x/length, vec.y/length, vec.z/length);
}

fn ray_color(r :Ray<f64>) -> Vector3<f64>{
    let t : f64 = hit_sphere(Vector3::new(0.0,0.0,-1.0), 0.5, &r);
    if t > 0.0 {
        let sub = r.direction - Vector3::new(0.0,0.0,-1.0);
        let n : Vector3<f64> = unit_vector(sub);
        
        return Vector3::<f64>::new(n.x + 1.0, n.y+1.0, n.z+1.0) * 0.5;
    }
    let direction = unit_vector(r.direction);
    let t = 0.5 * (direction.y + 1.0);
    return (Vector3::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Vector3::new(0.5, 0.7, 1.0) * t);
}

fn hit_sphere(center: Vector3<f64>, radius : f64, r: &Ray<f64>) -> f64 {
    let oc: Vector3<f64> = r.origin - center;

    //let a = r.direction * r.direction;
    let a = dot_product(&r.direction, &r.direction);
    let b = 2.0 * dot_product(&oc, &r.direction);
    let c = dot_product(&oc, &oc) - (radius * radius);

    let discriminate = b * b - 4.0*a*c;


    if discriminate < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminate.sqrt())  / (a *2.0);
    }

}


fn main() {

    //Image
    const ASPECT_RATIO :f64 = 16.0/9.0;
    const IMAGE_WIDTH :u16 = 400;
    const IMAGE_HEIGHT :u16 = (IMAGE_WIDTH as f64/ASPECT_RATIO) as u16;

    // Camera
    let viewport_height :f64 = 2.0;
    let viewport_width :f64 = ASPECT_RATIO * viewport_height;
    let focal_length :f64 = 1.0;

    let origin = Vector3::new(0.0,0.0,0.0);
    let horizontal = Vector3::new(viewport_width, 0.0,0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);

    let lower_left_corner : Vector3<f64> = origin - horizontal/2.0  - vertical/2.0 - Vector3::new(0.0,0.0,focal_length);




    println!("P3");
    println!("{} {}", IMAGE_WIDTH + 1, IMAGE_HEIGHT + 1);
    println!("255");

    for j in 0..=IMAGE_HEIGHT {
    eprintln!("Scanlines remaining: {}", 256 - j);
        for i in 0..=IMAGE_WIDTH{
            let u :f64 = ((i as f64 / (IMAGE_WIDTH -1) as f64 ) ).into();
            let v :f64 = ((j as f64 / (IMAGE_HEIGHT -1) as f64 ) ).into();
            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v - origin);

            let color = ray_color(r);
            Color::print(&color);
        }
    }

    eprintln!("Done.");

}


