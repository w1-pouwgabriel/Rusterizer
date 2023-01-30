use glam::{UVec3, Vec2};

use crate::{vertex::Vertex, triangle::Triangle, texture::Texture, window::FrameBuffer};

pub struct Mesh{
    triangles: Vec<UVec3>,
    vertices: Vec<Vertex>,
}

impl Mesh{
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
            vertices: Vec::new(),
        }
    }

    pub fn triangles(&self) -> &Vec<UVec3> {
        &self.triangles
    }
    
    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn get_vertices_from_triangle(&self, triangle: UVec3) -> [&Vertex; 3] {
        [
            &self.vertices[triangle.x as usize],
            &self.vertices[triangle.y as usize],
            &self.vertices[triangle.z as usize],
        ]
    }

    // quite unsafe but for now it will do :)
    pub fn add_section_from_vertices(
        &mut self,
        triangles: &mut Vec<UVec3>,
        vertices: &mut Vec<Vertex>,
    ) {
        self.triangles.append(triangles);
        self.vertices.append(vertices);
    }

    pub fn raster_mesh(
        &self,
        texture: &Texture,
        buffer: &mut FrameBuffer,
        z_buffer: &mut Vec<f32>,
        viewport_size: Vec2
    ) {
        for triangle in self.triangles() {
            let vertices = self.get_vertices_from_triangle(*triangle);
            let triangle = Triangle::new(*vertices[0], *vertices[1], *vertices[2]);

            triangle.raster_triangle(
                texture,
                buffer,
                z_buffer,
                viewport_size
            );
        }
    }
    
}

// for more on struct initialization check Default trait
impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}