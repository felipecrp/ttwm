use crate::window::Window;



pub struct WindowShape {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32
}

pub struct WindowGeometry(Coordinate, Coordinate);

pub struct Coordinate {
    x: i32,
    y: i32
}

pub struct Workspace {
    containers: Vec<WindowContainer>
}

impl Workspace {
    pub fn new() -> Self {
        let mut containers: Vec<WindowContainer> = Vec::new();
        containers.push(WindowContainer::new());
        
        Self {
            containers: containers
        }
    }
    
    pub fn add_window(&mut self, window: Box<dyn Window>) -> () {
        let container = &mut self.containers[0];
        let children = &mut container.children;
        children.push(Container::Window(window));
        container.update();
    }
}

struct WindowContainer {
    children: Vec<Container>
}

impl WindowContainer {
    fn new() -> Self {
        Self {
            children: Vec::new()
        }
    }
    
    fn update(&self) -> () {
        let max_width = 1920;
        let width = max_width / self.children.len() as i32;
        let mut w0 = 0i32;
        for child in &self.children {
            let w1 = w0 + width;
            match child {
                Container::Container(_) => (),
                Container::Window(window) => {
                    println!("{}", window.get_name());
                    window.mve((w0, 0, width, 1080));
                } 
            }
            w0 = w1;
        }
    }
}

enum Container {
    Container(WindowContainer),
    Window(Box<dyn Window>)
}
