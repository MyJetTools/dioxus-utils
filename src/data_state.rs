use core::panic;
use std::fmt::Debug;

use crate::RenderState;

#[derive(Debug)]
pub struct DataState<T: Debug> {
    inner: RenderState<T>,
    pub had_data_loaded_once: bool,
}

impl<T: Debug> DataState<T> {
    pub fn new() -> Self {
        Self {
            inner: RenderState::None,
            had_data_loaded_once: false,
        }
    }

    pub fn is_none(&self) -> bool {
        self.inner.is_none()
    }

    pub fn set_loading(&mut self) {
        self.inner.set_loading();
    }

    pub fn set_loaded(&mut self, value: T) {
        self.inner.set_loaded(value);
    }

    pub fn set_error(&mut self, err: impl Debug) {
        self.inner.set_error(err);
    }

    pub fn reset(&mut self) {
        self.inner = RenderState::None;
    }

    pub fn is_loading(&self) -> bool {
        self.inner.is_loading()
    }

    pub fn has_value(&self) -> bool {
        self.inner.has_value()
    }

    pub fn try_unwrap_as_loaded(&self) -> Option<&T> {
        match &self.inner {
            RenderState::Loaded(value) => value.into(),
            _ => None,
        }
    }

    pub fn unwrap_as_loaded(&self) -> &T {
        match &self.inner {
            RenderState::Loaded(value) => value,
            _ => panic!("DataState is not loaded"),
        }
    }

    pub fn to_not_loaded_cases(&self) -> Option<NotLoadedCases> {
        match &self.inner {
            RenderState::None => NotLoadedCases::None.into(),
            RenderState::Loading => NotLoadedCases::Loading.into(),
            RenderState::Loaded(_) => None,
            RenderState::Error(_) => None,
        }
    }

    pub fn set_value(&mut self, value: T) {
        self.inner = RenderState::Loaded(value);
        self.had_data_loaded_once = true;
    }

    pub fn try_unwrap_as_loaded_mut(&mut self) -> Option<&mut T> {
        match &mut self.inner {
            RenderState::Loaded(value) => Some(value),
            _ => None,
        }
    }

    pub fn unwrap_as_loaded_mut(&mut self) -> &mut T {
        match &mut self.inner {
            RenderState::Loaded(value) => {
                return value;
            }
            RenderState::None => {
                panic!("Trying unwrap data state as loaded but it is in state None");
            }
            RenderState::Loading => {
                panic!("Trying unwrap data state as loaded but it is in state Loading");
            }
            RenderState::Error(err) => {
                panic!(
                    "Trying unwrap data state as loaded but it is in state Error: {:?}",
                    err
                );
            }
        }
    }

    pub fn as_ref(&self) -> &RenderState<T> {
        &self.inner
    }
}

impl<T: Debug> Default for DataState<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum NotLoadedCases {
    None,
    Loading,
}
