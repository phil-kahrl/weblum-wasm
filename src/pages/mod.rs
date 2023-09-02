pub mod home;

pub use self::{home::*};

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default]
    Home,
}

