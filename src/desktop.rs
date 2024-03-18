use crate::{api::win::WinWindowManager, geometry::GeometryContainer, layout::Layout, window::{Window, WindowManager}, window_event::WindowEventListener, workspace::Workspace};


pub struct Desktop {
    workspaces: Vec<Workspace>,
    window_manager: WinWindowManager,
    // window_event_listener: WindowEventListener,
}

impl Desktop {
    pub fn new() -> Self {
        let desktop = Self {
            workspaces: vec![Workspace::new()],
            window_manager: WinWindowManager::new()
        };

        // WindowEventListener 
        desktop
    }
    
    pub fn test(&self) -> () {}

    pub fn init(&mut self) -> () {
        self.window_manager.register(WindowEventListener {});
        self.window_manager.evt(&|evt| self.test());

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

impl Default for Desktop {
    fn default() -> Self {
        Self::new()
    }
}



// let windows = window_manager.get_windows();
// for window in windows {
//     println!("{}", window.get_name());
//     workspace.add_window(window);
//     // thread::sleep(time::Duration::from_millis(1000));
// }