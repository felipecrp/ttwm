use crate::{container::Container, geometry::{GeometryContainer, Shape}, window::Window};


pub struct Layout {
    pub containers: Vec<Container>,
    shape: Shape
}

impl Layout {
    pub fn new(shape: &Shape) -> Self {
        Self {
            containers: Vec::new(),
            shape: Shape {
                x: shape.x,
                y: shape.y,
                width: shape.width,
                height: shape.height
            }
        }
    }
 
    pub fn add(&mut self, window: Box<dyn Window>) -> () {
        self.containers.push(Container::Window(window));
    }   

    // pub fn update(&self) -> () {
    //     let max_width = 1920;
    //     let width = max_width / self.containers.len() as i32;
    //     let mut w0 = 0i32;
    //     for child in &self.containers {
    //         let w1 = w0 + width;
    //         match child {
    //             Container::Layout(_) => (),
    //             Container::Window(window) => {
    //                 println!("{}", window.get_name());
    //                 // window.mve((w0, 0, width, 1080));
    //             } 
    //         }
    //         w0 = w1;
    //     }
    // }
}

impl GeometryContainer for Layout {
    fn update(&self) -> () {
        let width = self.shape.width / self.containers.len() as i32;
        let mut x = self.shape.x;
        for container in &self.containers {
            match container {
                Container::Layout(_) => (),
                Container::Window(window) => {
                    println!("{}", window.get_name());
                    window.mve((x, 0, width, 1080));
                } 
            }
            x += width;
        }
    }
}