use glam::{UVec3, Vec2, Mat4};

use crate::{vertex::Vertex, triangle::Triangle, texture::Texture, window::FrameBuffer};
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone)]
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

    pub fn from_vertices(triangles: &[UVec3], vertices: &[Vertex]) -> Self {
        let mut mesh = Mesh::new();
        mesh.add_section_from_vertices(triangles, vertices);
        mesh
    }

    pub fn add_section_from_vertices(&mut self, triangles: &[UVec3], vertices: &[Vertex]) {
        let offset = self.vertices.len() as u32;
        let triangles: Vec<UVec3> = triangles.iter().map(|tri| *tri + offset).collect();
        self.triangles.extend_from_slice(&triangles);
        self.vertices.extend_from_slice(vertices);
    }

    pub fn raster_mesh(
        &self,
        mvp: &Mat4,
        texture: Option<&Texture>,
        buffer: &mut FrameBuffer,
        z_buffer: &mut Vec<f32>,
        viewport_size: Vec2,

    ) {
        for triangle in self.triangles() {
            let vertices = self.get_vertices_from_triangle(*triangle);

            //Go over every vertex to check if they need to be culled or not    (1)
            //  If 2 vertices fall inside the box than clip                     (2)
            //      rasterize                                                   (3)

            let triangle = Triangle::new(*vertices[0], *vertices[1], *vertices[2]);

            triangle.raster_triangle(
                mvp,
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

impl Add for Mesh {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut result = Self::from_vertices(self.triangles(), self.vertices());
        result.add_section_from_vertices(rhs.triangles(), rhs.vertices());
        result
    }
}

impl AddAssign for Mesh {
    fn add_assign(&mut self, rhs: Self) {
        self.add_section_from_vertices(rhs.triangles(), rhs.vertices());
    }
}