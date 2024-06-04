use std::fs;

use crate::opengl_handler::CameraHandler;

struct BoundingBox {
    x_min : f32,
    x_max : f32,
    y_min : f32,
    y_max : f32,
    z_min : f32,
    z_max : f32,
}

impl BoundingBox {
    fn mean_x(&self) -> f32 {
        (self.x_min + self.x_max) / 2.
    }
    fn mean_y(&self) -> f32 {
        (self.y_min + self.y_max) / 2.
    }
    fn mean_z(&self) -> f32 {
        (self.z_min + self.z_max) / 2.
    }
    fn max_dim(&self) -> f32 {
        let dx = self.x_max - self.x_min;
        let dy = self.y_max - self.y_min;
        let dz = self.z_max - self.z_min;

        dx.max(dy).max(dz)
    }
}

fn get_bounding_box(obj_file_path : &str) -> BoundingBox {
    let content = fs::read_to_string(obj_file_path)
        .expect(&format!("Could not read file: {}", obj_file_path));

    let rows = content.split("\n");

    let mut x_min = f32::MAX;
    let mut x_max = f32::MIN;
    let mut y_min = f32::MAX;
    let mut y_max = f32::MIN;
    let mut z_min = f32::MAX;
    let mut z_max = f32::MIN;

    let mut x_tot = 0.;
    let mut y_tot = 0.;
    let mut z_tot = 0.;
    let mut n = 0.;

    for elm in rows {
        let items : Vec<&str> = elm.split_whitespace().collect();

        if items.len() == 0 {
            continue;
        }

        if items[0] == "v" {
            let x = items[1].parse::<f32>().unwrap();
            let y = items[2].parse::<f32>().unwrap();
            let z = items[3].parse::<f32>().unwrap();

            x_min = x.min(x_min);
            x_max = x.max(x_max);
            y_min = y.min(y_min);
            y_max = y.max(y_max);
            z_min = z.min(z_min);
            z_max = z.max(z_max);

            x_tot += x;
            y_tot += y;
            z_tot += z;
            n += 1.;
        }
    }

    println!("{}, {}, {}", y_min, y_max, y_tot / n);

    BoundingBox{x_min, x_max, y_min, y_max, z_min, z_max}
}

pub fn center_obj_fn(obj_file_path : &str, x_adjust : f32, y_adjust : f32, z_adjust : f32) -> impl Fn(&mut CameraHandler) -> () {
    let bounding_box = get_bounding_box(obj_file_path);

    let scaling = 2. / bounding_box.max_dim();
    let dx = -bounding_box.mean_x() / scaling;
    let dy = -bounding_box.mean_y() / scaling;
    let dz = -bounding_box.mean_z() / scaling;

    println!("{}, {}, {}, {}", scaling, dx, dy, dz);

    move |camera_handler| {
        camera_handler.translate(dx + x_adjust, dy + y_adjust, dz + z_adjust);
        camera_handler.scale(scaling, scaling, scaling)
    }
}