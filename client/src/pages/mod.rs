pub mod btc;
pub mod home;
pub mod login;
pub mod register;

pub use self::{btc::*, home::*, login::*, register::*};

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default]
    Home,
    Login,
    Register,
    Btc,
}

impl Page {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::Login => "/login",
            Self::Register => "/register",
            Self::Btc => "/btc",
        }
    }
}
