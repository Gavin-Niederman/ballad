pub mod sidebar;
pub mod window;
pub mod symbolic_icon;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Orientation {
    Vertical,
    Horizontal,
}
impl From<Orientation> for gtk::Orientation {
    fn from(orientation: Orientation) -> Self {
        match orientation {
            Orientation::Vertical => gtk::Orientation::Vertical,
            Orientation::Horizontal => gtk::Orientation::Horizontal,
        }
    }
}
