use crate::container::WindowShape;


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
    fn set_shape(&self, shape: WindowShape) -> &Self;
}
