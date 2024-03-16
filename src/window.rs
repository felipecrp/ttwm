use crate::container::WindowShape;

/// Represent a Generic Window
pub trait Window {
    /// Return the window name
    fn get_name(&self) -> String;

    /// Return true if the window is maximized
    fn is_maximized(&self) -> bool;

    /// Return true if the window is minimized
    fn is_minimized(&self) -> bool;

    /// Return the window shape
    fn get_shape(&self) -> WindowShape;

    /// Set the window shape
    fn set_shape(&self, shape: WindowShape) -> ();
}

/// Represent a Generic Window Manager
pub trait WindowManager {
    /// Return all available windows
    fn get_windows(&self) -> Vec<Box<dyn Window>>;
}
