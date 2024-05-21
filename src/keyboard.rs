use std::collections::HashSet;

use liushu_core::engine::candidates::Candidate;
use wayland_client::{protocol::wl_keyboard, WEnum};

#[derive(Debug, Default)]
pub struct KeyboardProcessor {
    handled_keys: HashSet<u32>,
}

impl KeyboardProcessor {
    pub fn handle_event(&mut self, event: wl_keyboard::Event) -> KeyboardProcessorResponse {
        match event {
            wl_keyboard::Event::Key { key, state, .. } => match state {
                WEnum::Value(wl_keyboard::KeyState::Pressed) => match key {
                    // a-z
                    16..=25 | 30..=38 | 44..=50 => {
                        self.handled_keys.insert(key);
                        KeyboardProcessorResponse::Composing(key)
                    }
                    57 => {
                        self.handled_keys.insert(key);
                        KeyboardProcessorResponse::Commit
                    }
                    _ => KeyboardProcessorResponse::Unhandled(event),
                },
                WEnum::Value(wl_keyboard::KeyState::Released) => {
                    if self.handled_keys.contains(&key) {
                        self.handled_keys.remove(&key);
                        KeyboardProcessorResponse::Ignored
                    } else {
                        KeyboardProcessorResponse::Unhandled(event)
                    }
                }
                _ => KeyboardProcessorResponse::Unhandled(event),
            },
            _ => KeyboardProcessorResponse::Unhandled(event),
        }
    }
}

pub enum KeyboardProcessorResponse {
    Composing(u32),
    Commit,
    Ignored,
    Unhandled(wl_keyboard::Event),
    Result(String, Vec<Candidate>),
}
