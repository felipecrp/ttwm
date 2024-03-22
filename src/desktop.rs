use crate::{api::win::WinWindowManager, geometry::GeometryContainer, layout::Layout, window::{Window, WindowManager}, window_event::WindowEventListener, workspace::Workspace};


pub struct Desktop {
    workspaces: Vec<Workspace>,
    window_manager: WinWindowManager
}

impl Desktop {
    pub fn new() -> Self {
        let window_manager = WinWindowManager::new();
        let desktop = Self {
            workspaces: vec![Workspace::new()],
            window_manager: window_manager,
        };

        // WindowEventListener 
        desktop
    }
    
    pub fn test(&self) -> () {}

    pub fn init(&mut self) -> () {
        let windows = self.window_manager.get_windows();
        for window in windows {
            self.add(window);
        }
        self.update();
    }
    
    fn add(&mut self, window: Box<dyn Window>) -> () {
        self.get_active_container().add(window);
    }
    
    fn get_active_container(&mut self) -> &mut Layout {
        self.get_active_workspace().get_active_container()
    }
    
    fn get_active_workspace(&mut self) -> &mut Workspace {
        &mut self.workspaces[0]
    }
    
    fn update(&mut self) -> () {
        let container: &dyn GeometryContainer = self.get_active_container() as &dyn GeometryContainer ;
        container.update(); 
    }
}


