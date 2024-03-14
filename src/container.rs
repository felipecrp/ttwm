

struct Dimension {
    width: u8,
    height: u8
}

struct Monitor {
    size: Dimension,
    workspaces: Vec<Workspace>
}

struct Workspace {
    windows: Container
}

enum Container {
    Window(WindowContainer),
    WindowGroup(WindowGroupContainer),
}

struct WindowGroupContainer {
    size: Dimension,
    containers: Vec<Container>
}

pub struct WindowContainer {
    name: String,
    size: Dimension
}

pub fn test() -> () {
    let mut window1 = WindowContainer {
        name: String::from("window1"),
        size: Dimension { width: 10, height: 10  } 
    };

    let mut window2 = WindowContainer { 
        name: String::from("window2"),
        size: Dimension { width: 10, height: 10  } 
    };

    let mut group1 = WindowGroupContainer {
        size: Dimension { width: 10, height: 10  },
        containers: Vec::new()
    };

    let mut group2 = WindowGroupContainer {
        size: Dimension { width: 10, height: 10  },
        containers: Vec::new()
    };
    
    group1.containers.push(Container::Window(window1));
    group2.containers.push(Container::Window(window2));
    group1.containers.push(Container::WindowGroup(group2));

}
