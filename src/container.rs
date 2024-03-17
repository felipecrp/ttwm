use crate::{layout::Layout, window::Window};

/// A container that can contain a set of windows or layouts
pub enum Container {
    Layout(Layout),
    Window(Box<dyn Window>)
}


