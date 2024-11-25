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

    pub fn is_loading(&self) -> bool {
        match self {
            DataState::Loading => true,
            _ => false,
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
