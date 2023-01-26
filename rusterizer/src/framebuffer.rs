pub struct FrameBuffer {
    data: Vec<u32>,
    width: usize,
    height: usize
}

impl FrameBuffer {
    pub fn get_data(&self) -> &Vec<u32>{
        &self.data
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel_color: u32){
        self.data[y * self.width + x] = pixel_color;
    }
    
    pub fn new(width: usize, height: usize) -> FrameBuffer {
        FrameBuffer {
            data: vec![0; width * height],
            width: width,
            height: height
        }
    }
}