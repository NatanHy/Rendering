use std::{collections::HashMap, fs};
use crate::triangles::{TriangleMesh, VertexAttribute, VertexAttributeLayout};

#[derive(Eq, Hash, PartialEq)]
pub enum ObjType {
    VERTEX,
    NORMAL,
    TEXTURE
}

pub struct FaceLayout {
    map : HashMap<ObjType, usize>
}

impl FaceLayout {
    pub fn new(vertex : Option<usize>, normal : Option<usize>, texture : Option<usize>) -> Self {
        let mut map = HashMap::new();

        if let Some(i) = vertex {
            map.insert(ObjType::VERTEX, i);
        }
        if let Some(i) = normal {
            map.insert(ObjType::NORMAL, i);
        }
        if let Some(i) = texture {
            map.insert(ObjType::TEXTURE, i);
        }

        FaceLayout { map }
    }

    fn update_indicies(&self, verts : &mut Vec<u16>, norms : &mut Vec<u16>, tex : &mut Vec<u16>, indicies : &Vec<u16>) {
        if let Some(indx) = self.map.get(&ObjType::VERTEX) {
            verts.push(indicies[*indx])
        }
        if let Some(indx) = self.map.get(&ObjType::NORMAL) {
            norms.push(indicies[*indx])
        }
        if let Some(indx) = self.map.get(&ObjType::TEXTURE) {
            tex.push(indicies[*indx])
        }
    }

    fn make_verticies(&self, verts : &mut Vec<Vec<f32>>, norms : &mut Vec<Vec<f32>>, tex : &mut Vec<Vec<f32>>) -> Vec<f32> {
        let mut verticies = Vec::new();

        for i in 0..verts.len() {
            verticies.extend(verts[i].clone());
            verticies.extend(norm_index(&norms, i));
            verticies.extend(tex_index(&tex, i));
        }

        verticies
    }
}

fn add_coordinate(elms : &Vec<&str>, arr : &mut Vec<Vec<f32>>, dim : usize) {
    let v = &elms[1..=dim];
    let v_f32 : Vec<f32> = v.iter()
        .map(|x| x.parse::<f32>().unwrap())
        .collect();

    arr.push(v_f32);
}

fn tex_index(tex : &Vec<Vec<f32>>, i : usize) -> Vec<f32> {
    if i < tex.len() {
        return tex[i].clone();
    } else {
        return vec![0., 0.]
    }
}

fn norm_index(norm : &Vec<Vec<f32>>, i : usize) -> Vec<f32> {
    if i < norm.len() {
        return norm[i].clone();
    } else {
        return vec![0., 1., 0.]
    }  
}

pub fn obj_to_mesh(filename : &str, face_layout : &FaceLayout) -> TriangleMesh {

    let content = fs::read_to_string(filename)
        .expect(&format!("Could not read file: {}", filename));

    let rows = content.split("\n");

    let mut verts : Vec<Vec<f32>> = Vec::new();
    let mut norms : Vec<Vec<f32>> = Vec::new();
    let mut tex : Vec<Vec<f32>> = Vec::new();

    let mut vertex_indicies : Vec<u16> = Vec::new();
    let mut norm_indicies : Vec<u16> = Vec::new();
    let mut tex_indicies : Vec<u16> = Vec::new();

    for row in rows {
        let elms : Vec<&str> = row.split_whitespace().collect();

        if elms.len() == 0 {continue;}

        match elms[0] {
            "v" => {
                add_coordinate(&elms, &mut verts, 3);
            },
            "vn" => {
                add_coordinate(&elms, &mut norms, 3);
            },
            "vt" => {
                add_coordinate(&elms, &mut tex, 2);
            }
            "f" => {
                let face_elms = &elms[1..4];

                for f in face_elms {
                    let indicies : Vec<u16> = f
                        .split("/")
                        .map(|x| 
                            if let Ok(n) = x.parse::<u16>() {
                                n - 1
                            } else {
                                0
                            })
                        .collect()
                        ;

                    face_layout.update_indicies(
                        &mut vertex_indicies,
                        &mut norm_indicies,
                        &mut tex_indicies,
                        &indicies
                    );
                }

            }
            _ => ()
        }
    }

    let verticies = face_layout.make_verticies(
        &mut verts, &mut norms, &mut tex
    );

    let vec3_size = 3 * std::mem::size_of::<f32>() as i32;
    let vec2_size = 2 * std::mem::size_of::<f32>() as i32;

    let layout = VertexAttributeLayout::new(
        vec![
            VertexAttribute::new(0, 3, vec3_size, gl::FLOAT),
            VertexAttribute::new(1, 3, vec3_size, gl::FLOAT),
            VertexAttribute::new(2, 2, vec2_size, gl::FLOAT)
        ]
    );

    TriangleMesh::from_array_indicies(
        verticies,
        vertex_indicies,
        norm_indicies,
        tex_indicies,
        layout
    )
}