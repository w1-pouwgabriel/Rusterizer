extern crate minifb;

#[path = "resources/texture.rs"] pub mod texture;
#[path = "math/transform.rs"] mod transform;
#[path = "math/boundingbox2D.rs"] mod boundingbox2D;
mod window;
mod utils;
#[path = "graphics/triangle.rs"] mod triangle;
#[path = "graphics/vertex.rs"] mod vertex;
#[path = "graphics/mesh.rs"] mod mesh;
#[path = "graphics/camera.rs"] mod camera;

use minifb::Key;
use transform::Transform;
use triangle::Triangle;
use window::Window;
use vertex::Vertex;
use texture::Texture;
use mesh::Mesh;
use camera::Camera;

use std::path::{Path};

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

pub fn process_input_camera(window: &minifb::Window, camera: &mut Camera, time: f32) {
    let mut axis = glam::vec2(0.0, 0.0);
    // we will make registering later

    if window.is_key_down(Key::A) {
        axis.x -= 1.0 * time;
    }
    if window.is_key_down(Key::D) {
        axis.x += 1.0 * time;
    }
    if window.is_key_down(Key::W) {
        axis.y += 1.0 * time;
    }
    if window.is_key_down(Key::S) {
        axis.y -= 1.0 * time;
    }
    camera.transform.translation += camera.transform.right() * camera.speed * axis.x
        + camera.transform.forward() * camera.speed * axis.y;
    //camera.transform.translation += Vec3::new(axis.x, 0.0, axis.y) * camera.speed;
}

fn main() {
    let mut window = Window::new("Rusterizer - ESC to exit".to_string(), WIDTH, HEIGHT);

    let mut z_buffer;                                               //Maybe add this variable to some sort of graphics pipeline???
    let texture = Texture::load(Path::new("assets/uv_mapper.jpg"));  //TODO: Add some resource manager

    let v0 = Vertex {
        position: glam::vec3(-1.0, -1.0, 1.0),
        color: glam::vec3(0.0, 1.0, 1.0),
        uv: glam::vec2(0.0, 1.0),
    };
    let v1 = Vertex {
        position: glam::vec3(-1.0, 1.0, 1.0),
        color: glam::vec3(1.0, 0.0, 0.0),
        uv: glam::vec2(0.0, 0.0),
    };
    let v2 = Vertex {
        position: glam::vec3(1.0, 1.0, 1.0),
        color: glam::vec3(0.0, 1.0, 0.0),
        uv: glam::vec2(1.0, 0.0),
    };
    let v3 = Vertex {
        position: glam::vec3(1.0, -1.0, 1.0),
        color: glam::vec3(0.0, 1.0, 1.0),
        uv: glam::vec2(1.0, 1.0),
    };

    let triangles = vec![glam::uvec3(2, 1, 0), glam::uvec3(3, 2, 0)]; 
    let vertices = vec![v0, v1, v2, v3];

    let triangle1: Triangle = Triangle { v0: v0, v1: v1, v2: v2 };

    let mesh = Mesh::from_vertices(&triangles, &vertices);

    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;

    let mut camera = Camera {
        aspect_ratio,
        transform: Transform::from_translation(glam::vec3(0.0, 0.0, 8.0)),
        frustum_far: 100.0,
        ..Default::default()
    };

    // Limit to max ~60 fps update rate
    //window.limit_fps(Some(60));

    //+z
    let transform0 = Transform::IDENTITY;
    //-z
    let transform1 = Transform::from_rotation(glam::Quat::from_euler(
        glam::EulerRot::XYZ,
        -std::f32::consts::PI,
        0.0,
        0.0,
    ));
    //+y
    let transform2 = Transform::from_rotation(glam::Quat::from_euler(
        glam::EulerRot::XYZ,
        std::f32::consts::FRAC_PI_2,
        0.0,
        0.0,
    ));
    //-y
    let transform3 = Transform::from_rotation(glam::Quat::from_euler(
        glam::EulerRot::XYZ,
        -std::f32::consts::FRAC_PI_2,
        0.0,
        0.0,
    ));
    //+x
    let transform4 = Transform::from_rotation(glam::Quat::from_euler(
        glam::EulerRot::XYZ,
        0.0,
        -std::f32::consts::FRAC_PI_2,
        0.0,
    ));
    //-x
    let transform5 = Transform::from_rotation(glam::Quat::from_euler(
        glam::EulerRot::XYZ,
        0.0,
        std::f32::consts::FRAC_PI_2,
        0.0,
    ));

    let mut rot = std::f32::consts::FRAC_PI_4;

    let mut delta_time = 0f32;

    let mut now = std::time::Instant::now();
    let mut last = 0f32;

    while !window.should_close() {
        delta_time = now.elapsed().as_secs_f32() - last;
        println!("Time to render a frame in seconds: {}", delta_time);
        last = now.elapsed().as_secs_f32();

        let (width, height) = window.frame_buffer().size();
        //Clear buffer
        z_buffer = vec![f32::INFINITY; width * height];
        window.frame_buffer().clear();

        process_input_camera(&window.window(), &mut camera, delta_time); //

        let window_size = glam::vec2(width as f32, height as f32);
        let parent_local =
        Transform::from_rotation(glam::Quat::from_euler(glam::EulerRot::XYZ, rot, rot, 0.2))
            .local(); // Model
            
        let view = camera.view();
        let proj = camera.projection();

        mesh.raster_mesh(        
            &(proj * view * parent_local * transform0.local()),
            Some(&texture),
            window.frame_buffer(),
            &mut z_buffer,
            window_size);

        mesh.raster_mesh(        
            &(proj * view * parent_local * transform1.local()),
            Some(&texture),
            window.frame_buffer(),
            &mut z_buffer,
            window_size);
            
        mesh.raster_mesh(        
            &(proj * view * parent_local * transform2.local()),
            Some(&texture),
            window.frame_buffer(),
            &mut z_buffer,
            window_size);
            
        mesh.raster_mesh(
            &(proj * view * parent_local * transform3.local()),
            Some(&texture),
            window.frame_buffer(),
            &mut z_buffer,
            window_size);
            
        mesh.raster_mesh(
            &(proj * view * parent_local * transform4.local()),
            Some(&texture),
            window.frame_buffer(),
            &mut z_buffer,
            window_size);
            
        mesh.raster_mesh(
            &(proj * view * parent_local * transform5.local()),
            Some(&texture),
            window.frame_buffer(),
            &mut z_buffer,
            window_size);

            rot += 0.05;
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.display();
    }
}