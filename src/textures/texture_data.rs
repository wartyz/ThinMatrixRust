pub struct TextureData {
    width: i32,
    height: i32,
    buffer: f32,
}

impl TextureData {
    pub fn new(buffer: f32, width: i32, height: i32) -> TextureData {
        TextureData {
            width,
            height,
            buffer,
        }
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_buffer(&self) -> f32 {
        self.buffer
    }
}