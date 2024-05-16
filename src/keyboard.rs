use std::collections::HashSet;

use wayland_client::{protocol::wl_keyboard, WEnum};

#[derive(Debug, Default)]
pub struct KeyboardProcessor {
    handled_keys: HashSet<u32>,
}

impl KeyboardProcessor {
    pub fn handle_event(
        &mut self,
        event: wl_keyboard::Event,
    ) -> (wl_keyboard::Event, KeyboardProcessorResponse) {
        let response = match event {
            wl_keyboard::Event::Key { key, state, .. } => match state {
                WEnum::Value(wl_keyboard::KeyState::Pressed) => match key {
                    // a-z
                    16..=25 | 30..=38 | 44..=50 => {
                        self.handled_keys.insert(key);
                        KeyboardProcessorResponse::Composing
                    }
                    57 => {
                        self.handled_keys.insert(key);
                        KeyboardProcessorResponse::Commit
                    }
                    _ => KeyboardProcessorResponse::Unhandled,
                },
                WEnum::Value(wl_keyboard::KeyState::Released) => {
                    if self.handled_keys.contains(&key) {
                        self.handled_keys.remove(&key);
                        KeyboardProcessorResponse::Ignored
                    } else {
                        KeyboardProcessorResponse::Unhandled
                    }
                }
                _ => KeyboardProcessorResponse::Unhandled,
            },
            _ => KeyboardProcessorResponse::Unhandled,
        };
        (event, response)
    }
}

pub enum KeyboardProcessorResponse {
    Composing,
    Commit,
    Ignored,
    Unhandled,
}
