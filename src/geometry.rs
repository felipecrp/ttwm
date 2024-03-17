use crate::container::Container;


pub struct Shape {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32
}

pub trait GeometryContainer {
    // fn get_x(&self) -> i32;
    // fn get_y(&self) -> i32;
    // fn get_width(&self) -> i32;
    // fn get_height(&self) -> i32;
    // fn get_container(&self) -> Container;
    fn update(&self) -> ();
}

