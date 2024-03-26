use liushu_core::engine::Engine;
use wayland_client::{
    event_created_child,
    protocol::{
        wl_keyboard::{self, KeyState},
        wl_registry,
    },
    Connection, Dispatch, QueueHandle, WEnum,
};
use wayland_protocols::wp::input_method::zv1::client::{
    zwp_input_method_context_v1,
    zwp_input_method_v1::{self, EVT_ACTIVATE_OPCODE},
};

fn main() {
    let conn = Connection::connect_to_env().unwrap();

    let mut event_queue = conn.new_event_queue();
    let qhandle = event_queue.handle();

    let display = conn.display();
    display.get_registry(&qhandle, ());

    let mut state = AppState {
        running: true,
        ..Default::default()
    };

    while state.running {
        event_queue.blocking_dispatch(&mut state).unwrap();
    }
}

#[derive(Default)]
struct AppState {
    running: bool,
    input: String,
    input_method: Option<zwp_input_method_v1::ZwpInputMethodV1>,
    context: Option<zwp_input_method_context_v1::ZwpInputMethodContextV1>,
    input_serial: u32,
    engine: Engine,
}

impl Dispatch<wl_registry::WlRegistry, ()> for AppState {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: <wl_registry::WlRegistry as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name, interface, ..
        } = event
        {
            println!("{} {}", name, interface);
            match &interface[..] {
                "zwp_input_method_v1" => {
                    let input_method = registry
                        .bind::<zwp_input_method_v1::ZwpInputMethodV1, _, _>(name, 1, qh, ());
                    state.input_method = Some(input_method);
                }
                _ => {}
            }
        }
    }
}

impl Dispatch<zwp_input_method_v1::ZwpInputMethodV1, ()> for AppState {
    fn event(
        state: &mut Self,
        _proxy: &zwp_input_method_v1::ZwpInputMethodV1,
        event: zwp_input_method_v1::Event,
        _data: &(),
        _conn: &Connection,
        qhandle: &QueueHandle<Self>,
    ) {
        println!("current event is {:#?}", event);
        match event {
            zwp_input_method_v1::Event::Activate { id } => {
                println!("method activate");
                id.grab_keyboard(qhandle, ());
                state.context = Some(id);
                println!("grab keyboard");
            }
            zwp_input_method_v1::Event::Deactivate { context } => {
                state.context = None;
                context.destroy();
                println!("method inactive");
            }
            _ => {}
        }
    }

    event_created_child!(AppState, zwp_input_method_v1::ZwpInputMethodV1, [
        EVT_ACTIVATE_OPCODE => (zwp_input_method_context_v1::ZwpInputMethodContextV1, ()),
    ]);
}

impl Dispatch<zwp_input_method_context_v1::ZwpInputMethodContextV1, ()> for AppState {
    fn event(
        state: &mut Self,
        _context: &zwp_input_method_context_v1::ZwpInputMethodContextV1,
        event: zwp_input_method_context_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        println!("current content event is {:#?}", event);
        match event {
            zwp_input_method_context_v1::Event::SurroundingText {
                text,
                cursor,
                anchor,
            } => {
                println!("{} {} {}", text, cursor, anchor);
            }
            zwp_input_method_context_v1::Event::CommitState { serial } => {
                state.input_serial = serial
            }
            _ => {}
        }
    }
}

impl Dispatch<wl_keyboard::WlKeyboard, ()> for AppState {
    fn event(
        state: &mut Self,
        _proxy: &wl_keyboard::WlKeyboard,
        event: wl_keyboard::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        match event {
            wl_keyboard::Event::Key {
                key,
                state: WEnum::Value(KeyState::Pressed),
                ..
            } => {
                let key_str = match key {
                    16 => "q",
                    17 => "w",
                    18 => "e",
                    19 => "r",
                    20 => "t",
                    21 => "y",
                    22 => "u",
                    23 => "i",
                    24 => "o",
                    25 => "p",
                    26 => "[",
                    27 => "]",
                    28 => "\n",
                    30 => "a",
                    31 => "s",
                    32 => "d",
                    33 => "f",
                    34 => "g",
                    35 => "h",
                    36 => "j",
                    37 => "k",
                    38 => "l",
                    39 => ";",
                    40 => "'",
                    41 => "`",
                    42 => "\\",
                    44 => "z",
                    45 => "x",
                    46 => "c",
                    47 => "v",
                    48 => "b",
                    49 => "n",
                    50 => "m",
                    51 => ",",
                    52 => ".",
                    53 => "/",
                    _ => "",
                };
                state.input.push_str(key_str);

                if let Some(context) = &state.context {
                    context.commit_string(state.input_serial, key_str.to_string());
                }
            }
            _ => {}
        }
    }
}
