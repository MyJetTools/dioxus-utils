use core::panic;
use std::fmt::Debug;

#[derive(Debug)]
pub enum DataState<T: Debug> {
    None,
    Loading,
    Loaded(T),
    Error(String),
}

impl<T: Debug> DataState<T> {
    pub fn new() -> Self {
        DataState::None
    }

    pub fn is_none(&self) -> bool {
        match self {
            DataState::None => true,
            _ => false,
        }
    }

    pub fn set_loading(&mut self) {
        *self = DataState::Loading;
    }

    pub fn set_loaded(&mut self, value: T) {
        *self = DataState::Loaded(value);
    }

    pub fn set_error(&mut self, err: impl Debug) {
        *self = DataState::Error(format!("{:?}", err));
    }

    pub fn set_none(&mut self) {
        *self = DataState::None;
    }

    pub fn is_loading(&self) -> bool {
        match self {
            DataState::Loading => true,
            _ => false,
        }
    }

    pub fn has_value_loaded(&self) -> bool {
        match self {
            DataState::Loaded(_) => true,
            _ => false,
        }
    }

    pub fn try_unwrap_as_loaded(&self) -> Option<&T> {
        match self {
            DataState::Loaded(value) => value.into(),
            _ => None,
        }
    }

    pub fn unwrap_as_loaded(&self) -> &T {
        match self {
            DataState::Loaded(value) => value,
            _ => panic!("DataState is not loaded"),
        }
    }

    pub fn to_not_loaded_cases(&self) -> Option<NotLoadedCases> {
        match self {
            DataState::None => NotLoadedCases::None.into(),
            DataState::Loading => NotLoadedCases::Loading.into(),
            DataState::Loaded(_) => None,
            DataState::Error(_) => None,
        }
    }

    pub fn set_value(&mut self, value: T) {
        *self = DataState::Loaded(value);
    }

    pub fn try_unwrap_as_loaded_mut(&mut self) -> Option<&mut T> {
        match self {
            DataState::Loaded(value) => Some(value),
            _ => None,
        }
    }

    pub fn unwrap_as_loaded_mut(&mut self) -> &mut T {
        match self {
            DataState::Loaded(value) => value,
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

impl<T: Debug> From<T> for DataState<T> {
    fn from(value: T) -> Self {
        Self::Loaded(value)
    }
}

#[derive(Debug, Clone)]
pub enum NotLoadedCases {
    None,
    Loading,
}
