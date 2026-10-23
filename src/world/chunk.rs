use super::map::Content;

#[derive(Clone, Copy, PartialEq)]
pub struct Chunk {
    pub x: i32,
    pub y: i32,
    pub content: Content,
    pub smell: f32,
}

impl Chunk {
    
}