use crate::geometry::Shape;
use crate::{container::Container, window::Window};
use crate::layout::Layout;

/// A workspace that can manage a set of layouts
pub struct Workspace {
    containers: Vec<Layout>,
    shape: Shape
}

impl Workspace {
    pub fn new() -> Self {
        let mut containers: Vec<Layout> = Vec::new();
        let shape = Shape {
            x: 0,
            y: 0,
            width: 1920,
            height: 1080
        };
        containers.push(Layout::new(&shape));
        
        Self {
            containers: containers,
            shape: shape
        }
    }
    
    // pub fn add_window(&mut self, window: Box<dyn Window>) -> () {
    //     let container = &mut self.containers[0];
    //     let children = &mut container.containers;
    //     children.push(Container::Window(window));
    //     container.update();
    // }
    
    pub fn get_active_container(&mut self) -> &mut Layout {
        &mut self.containers[0]
    }
}


