use  crate::geometry::Shape;
/// Represent a Generic Window Manager
pub trait WindowManager {
    /// Return all available windows
    fn get_windows(&self) -> Vec<Box<dyn Window>>;
}

/// Represent a Generic Window
pub trait Window {
    /// Return the window name
    fn get_name(&self) -> String;

    /// Return true if the window is maximized
    fn is_maximized(&self) -> bool;

    /// Return true if the window is minimized
    fn is_minimized(&self) -> bool;

    /// Return the window shape
    fn get_shape(&self) -> Shape;

    /// Set the window shape
    fn set_shape(&self, geometry: Shape) -> ();

    fn mve(&self, geo: (i32, i32, i32, i32)) -> ();
}

