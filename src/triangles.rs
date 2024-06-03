pub struct VertexAttribute {
    index : gl::types::GLuint,
    attrib_size : gl::types::GLint,
    attrib_mem_size : i32,
    attrib_type : gl::types::GLenum,
}

impl VertexAttribute {
    pub fn new(
        index : gl::types::GLuint, 
        attrib_size : gl::types::GLint, 
        attrib_mem_size : i32,
        attrib_type : gl::types::GLenum) -> Self {
        VertexAttribute {index, attrib_size, attrib_mem_size, attrib_type }
    }
}

pub struct VertexAttributeLayout {
    attributes : Vec<VertexAttribute>,
    stride : gl::types::GLsizei,
}

impl VertexAttributeLayout {
    pub fn new(vertex_attributes : Vec<VertexAttribute>) -> Self {
        let mut layout = VertexAttributeLayout{ attributes : Vec::new(), stride : 0};

        layout.add(vertex_attributes);

        layout
    }

    fn add(&mut self, vertex_attributes : Vec<VertexAttribute>) {
        for attrib in vertex_attributes {
            self.stride += attrib.attrib_mem_size;
            self.attributes.push(attrib);
        }
    }

    pub fn enable_attributes(&self) {
        let mut ptr = 0;

        for attrib in &self.attributes {
            unsafe {
                println!("Pointer[ index: {}, size: {}, stride: {}, ptr: {}]", attrib.index, attrib.attrib_size, self.stride, ptr);
                gl::EnableVertexAttribArray(attrib.index);
                gl::VertexAttribPointer(
                    attrib.index,
                    attrib.attrib_size,
                    attrib.attrib_type,
                    gl::FALSE,
                    self.stride,
                    ptr as *mut std::os::raw::c_void
                )
            }
            ptr += attrib.attrib_mem_size;
        }
    }
}

pub struct TriangleMesh {
    pub verticies : Vec<f32>,
    pub vertex_indicies : Vec<u16>,
    pub normal_indicies : Vec<u16>,
    pub texture_indicies : Vec<u16>,
    pub vertex_attrib_layout : VertexAttributeLayout
}

impl TriangleMesh {
    pub fn from_array_indicies(
        verticies : Vec<f32>, 
        vertex_indicies : Vec<u16>,
        normal_indicies : Vec<u16>,
        texture_indicies : Vec<u16>,
        vertex_attrib_layout : VertexAttributeLayout
    ) -> Self {
        TriangleMesh {
            verticies, 
            vertex_indicies,
            normal_indicies,
            texture_indicies,
            vertex_attrib_layout
        }
    }

    pub fn full_quad() -> Self {
        let verticies = vec![
            -1., -1., -0.5,
            0., 0.,
            -1., 1., -0.5,
            0., 1.,
            1., 1., -0.5,
            1., 1.,
            1., -1., -0.5,
            1., 0.,
        ];
        let vertex_indicies = vec![
            0, 1, 2, 0, 2, 3
        ];
        let normal_indicies = Vec::new();
        let texture_indicies = vertex_indicies.clone();
        let vertex_attrib_layout = VertexAttributeLayout::new(
            vec![
                VertexAttribute::new(
                    0, 
                    3, 
                    3 * std::mem::size_of::<f32>() as i32,
                    gl::FLOAT),
                VertexAttribute::new(
                    1, 
                    2, 
                    2 * std::mem::size_of::<f32>() as i32,
                    gl::FLOAT),
            ]
        );

        TriangleMesh {verticies, vertex_indicies, normal_indicies, texture_indicies, vertex_attrib_layout}
    }

    pub fn enable_vertex_attributes(&self) {
        // Specify vertex attribute pointers
        self.vertex_attrib_layout.enable_attributes();
    }
}