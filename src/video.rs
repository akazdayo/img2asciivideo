pub struct Video {
    pub fps: u32,
    pub path: String,
}

impl Video {
    pub fn new(fps: u32, path: String) -> Video {
        Video {
            fps: fps,
            path: path,
        }
    }
}
