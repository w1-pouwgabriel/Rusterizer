use std::str::FromStr;

extern crate minifb;

pub struct Window {
    window: minifb::Window,
    frame_buffer: FrameBuffer
}
impl Window {
    pub fn new(name: String, width: usize, height: usize) -> Self {
        let mut options = minifb::WindowOptions::default();
        options.resize = true;
        let window = minifb::Window::new(name.as_str(), width, height, options).expect("Failed to create new window.");
        let mut window = Window {
            window: window,
            frame_buffer: FrameBuffer::new(width, height)
        };
        if let Ok(icon) = minifb::Icon::from_str("assets/rust.ico") {
            window.window.set_icon(icon);
        }else{
            println!("Could not load icon");
        }
        window
    }
    
    pub fn limit_fps(&mut self, fps_limit: Option<usize>) {
        let fps_limit = match fps_limit {
            Some(fps_limit) => {
                let fps_limit = 1.0 / (fps_limit as f32) * 1000000.0;
                Some(std::time::Duration::from_micros(fps_limit as u64))
            },
            None => None
        };
        self.window.limit_update_rate(fps_limit);
    }
    
    pub fn should_close(&mut self) -> bool {
        !self.window.is_open()
    }
    
    pub fn frame_buffer(&mut self) -> &mut FrameBuffer {
        &mut self.frame_buffer
    }
    
    pub fn display(&mut self) {
        let (width, height) = self.frame_buffer.size();
        self.window.update_with_buffer(&self.frame_buffer.data, width, height).expect("Failed to display frame buffer.");
        let (width, height) = self.window.get_size();
        if width != self.frame_buffer.width || height != self.frame_buffer.height {
            self.on_resize(width, height);
        }
    }
    
    fn on_resize(&mut self, width: usize, height: usize) {
        self.frame_buffer = FrameBuffer::new(width, height);
    }
}
pub struct FrameBuffer {
    data: Vec<u32>,
    width: usize,
    height: usize
}
impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        FrameBuffer {
            data: vec![0; width * height],
            width: width,
            height: height
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, u32> {
        self.data.iter_mut()
    }
}