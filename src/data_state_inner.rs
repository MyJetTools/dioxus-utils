use core::panic;
use std::fmt::Debug;

#[derive(Debug)]
pub enum RenderState<T: Debug> {
    None,
    Loading,
    Loaded(T),
    Error(String),
}

impl<T: Debug> RenderState<T> {
    pub fn new() -> Self {
        RenderState::None
    }

    pub fn is_none(&self) -> bool {
        match self {
            RenderState::None => true,
            _ => false,
        }
    }

    pub fn set_loading(&mut self) {
        *self = Self::Loading;
    }

    pub fn set_loaded(&mut self, value: T) {
        *self = Self::Loaded(value);
    }

    pub fn set_error(&mut self, err: impl Debug) {
        *self = Self::Error(format!("{:?}", err));
    }

    pub fn set_none(&mut self) {
        *self = Self::None;
    }

    pub fn is_loading(&self) -> bool {
        match self {
            Self::Loading => true,
            _ => false,
        }
    }

    pub fn has_value(&self) -> bool {
        match self {
            Self::Loaded(_) => true,
            _ => false,
        }
    }

    pub fn try_unwrap_as_loaded(&self) -> Option<&T> {
        match self {
            Self::Loaded(value) => value.into(),
            _ => None,
        }
    }

    pub fn unwrap_as_loaded(&self) -> &T {
        match self {
            Self::Loaded(value) => value,
            _ => panic!("DataState is not loaded"),
        }
    }

    pub fn set_value(&mut self, value: T) {
        *self = Self::Loaded(value);
    }

    pub fn try_unwrap_as_loaded_mut(&mut self) -> Option<&mut T> {
        match self {
            Self::Loaded(value) => Some(value),
            _ => None,
        }
    }

    pub fn unwrap_as_loaded_mut(&mut self) -> &mut T {
        match self {
            Self::Loaded(value) => value,
            _ => {
                panic!(
                    "Trying unwrap data state as loaded but it is in state {:?}",
                    self
                );
            }
        }
    }

    pub fn as_ref(&self) -> &Self {
        self
    }
}
