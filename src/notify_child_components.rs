use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct NotifyChildComponent<TValue: 'static> {
    signal: Signal<Option<TValue>>,
}

impl<TValue: 'static> PartialEq for NotifyChildComponent<TValue> {
    fn eq(&self, other: &Self) -> bool {
        self.signal == other.signal
    }
}

impl<TValue: 'static> NotifyChildComponent<TValue> {
    pub fn new() -> Self {
        Self {
            signal: use_signal(|| None),
        }
    }

     pub fn new_with_value(value: TValue) -> Self {
        Self {
            signal: use_signal(|| Some(value)),
        }
    }

    pub fn notify_other_components(&self, value: TValue) {
        let mut signal = self.signal;
        *signal.write() = Some(value);
    }

    pub fn on_notify<F>(&self, mut callback: F)
    where
        F: FnMut(TValue) + 'static,
    {
        let mut signal = self.signal;
        use_effect(move || {
            if signal.read().is_some() {
                if let Some(value) = signal.write().take() {
                    callback(value);
                }
            }
        });
    }
}
