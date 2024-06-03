use std::fs;

use crate::triangles::TriangleMesh;

pub fn obj_to_mesh(filename : &str) -> TriangleMesh {

    let content = fs::read_to_string(filename)
        .expect(&format!("Could not read file: {}", filename));

    let rows = content.split("\n");

    let mut verts : Vec<f32> = Vec::new();
    let mut indicies : Vec<u16> = Vec::new();

    for row in rows {
        let elms : Vec<&str> = row.split_whitespace().collect();

        if elms.len() == 0 {continue;}

        match elms[0] {
            "v" => {
                let v = &elms[1..4];
                let v_f32 : Vec<f32> = v.iter()
                    .map(|x| x.parse::<f32>().unwrap())
                    .collect();

                verts.extend(v_f32);
                verts.extend(vec![1., 1., 1.]);
            },
            "f" => {

                let face_elms = &elms[1..4];
                for f in face_elms {
                    let vert_index = f.split("/")
                        .next()
                        .unwrap()
                        .parse::<u16>()
                        .unwrap();

                    indicies.push(vert_index - 1);
                }

            }
            _ => ()
        }
    }

    println!("{:?}", verts);

    TriangleMesh::from_array_indicies(verts, indicies)
}