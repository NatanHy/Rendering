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
    pub vertex_indicies : Vec<u32>,
    pub normal_indicies : Vec<u32>,
    pub texture_indicies : Vec<u32>,
    pub vertex_attrib_layout : VertexAttributeLayout
}

impl TriangleMesh {
    pub fn from_array_indicies(
        verticies : Vec<f32>, 
        vertex_indicies : Vec<u32>,
        normal_indicies : Vec<u32>,
        texture_indicies : Vec<u32>,
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

    pub fn enable_vertex_attributes(&self) {
        // Specify vertex attribute pointers
        self.vertex_attrib_layout.enable_attributes();
    }
}