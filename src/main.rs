use std::fs::File;
use std::os::fd::AsFd;

use liushu_core::engine::{Engine, InputMethodEngine};
use wayland_client::{
    delegate_noop, event_created_child,
    protocol::{
        wl_buffer, wl_compositor,
        wl_keyboard::{self, KeyState},
        wl_registry, wl_shm, wl_shm_pool, wl_surface,
    },
    Connection, Dispatch, QueueHandle, WEnum,
};
use wayland_protocols::{
    wp::input_method::zv1::client::{
        zwp_input_method_context_v1,
        zwp_input_method_v1::{self, EVT_ACTIVATE_OPCODE},
        zwp_input_panel_surface_v1, zwp_input_panel_v1,
    },
    xdg::shell::client::{xdg_surface, xdg_toplevel, xdg_wm_base},
};

fn main() {
    let conn = Connection::connect_to_env().unwrap();

    let mut event_queue = conn.new_event_queue();
    let qhandle = event_queue.handle();

    let display = conn.display();
    display.get_registry(&qhandle, ());

    let mut state = AppState {
        running: true,
        engine: Engine::new("/home/elliot/.config/liushu/sunman.trie").expect("Open dict error"),
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
    surface: Option<wl_surface::WlSurface>,
    buffer: Option<wl_buffer::WlBuffer>,
    input_panel_surface: Option<zwp_input_panel_surface_v1::ZwpInputPanelSurfaceV1>,
    configured: bool,
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
                "wl_compositor" => {
                    let compositor =
                        registry.bind::<wl_compositor::WlCompositor, _, _>(name, 5, qh, ());
                    let surface = compositor.create_surface(qh, ());
                    state.surface = Some(surface);
                }
                "wl_shm" => {
                    let shm = registry.bind::<wl_shm::WlShm, _, _>(name, 1, qh, ());

                    let (init_w, init_h) = (320, 240);

                    let mut file = tempfile::tempfile().unwrap();
                    draw(&mut file, (init_w, init_h));
                    let pool = shm.create_pool(file.as_fd(), (init_w * init_h * 4) as i32, qh, ());
                    let buffer = pool.create_buffer(
                        0,
                        init_w as i32,
                        init_h as i32,
                        (init_w * 4) as i32,
                        wl_shm::Format::Argb8888,
                        qh,
                        (),
                    );
                    state.buffer = Some(buffer.clone());
                }
                "zwp_input_method_v1" => {
                    let input_method = registry
                        .bind::<zwp_input_method_v1::ZwpInputMethodV1, _, _>(name, 1, qh, ());
                    state.input_method = Some(input_method);
                }
                "zwp_input_panel_v1" => {
                    let input_panel =
                        registry.bind::<zwp_input_panel_v1::ZwpInputPanelV1, _, _>(name, 1, qh, ());
                    if state.surface.is_some() && state.input_panel_surface.is_none() {
                        let base_surface = state.surface.as_ref().unwrap();
                        let input_panel_surface =
                            input_panel.get_input_panel_surface(base_surface, qh, ());
                        input_panel_surface.set_overlay_panel();
                        state.configured = true;
                        state.input_panel_surface = Some(input_panel_surface);
                    }
                }
                _ => {}
            }
        }
    }
}

delegate_noop!(AppState: ignore wl_compositor::WlCompositor);
delegate_noop!(AppState: ignore wl_surface::WlSurface);
delegate_noop!(AppState: ignore wl_shm::WlShm);
delegate_noop!(AppState: ignore wl_shm_pool::WlShmPool);
delegate_noop!(AppState: ignore wl_buffer::WlBuffer);

fn draw(tmp: &mut File, (buf_x, buf_y): (u32, u32)) {
    use std::{cmp::min, io::Write};
    let mut buf = std::io::BufWriter::new(tmp);
    for y in 0..buf_y {
        for x in 0..buf_x {
            let a = 0xFF;
            let r = min(((buf_x - x) * 0xFF) / buf_x, ((buf_y - y) * 0xFF) / buf_y);
            let g = min((x * 0xFF) / buf_x, ((buf_y - y) * 0xFF) / buf_y);
            let b = min(((buf_x - x) * 0xFF) / buf_x, (y * 0xFF) / buf_y);
            buf.write_all(&[b as u8, g as u8, r as u8, a as u8])
                .unwrap();
        }
    }
    buf.flush().unwrap();
}

impl Dispatch<xdg_wm_base::XdgWmBase, ()> for AppState {
    fn event(
        _: &mut Self,
        wm_base: &xdg_wm_base::XdgWmBase,
        event: xdg_wm_base::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let xdg_wm_base::Event::Ping { serial } = event {
            wm_base.pong(serial);
        }
    }
}

impl Dispatch<xdg_surface::XdgSurface, ()> for AppState {
    fn event(
        state: &mut Self,
        xdg_surface: &xdg_surface::XdgSurface,
        event: xdg_surface::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let xdg_surface::Event::Configure { serial, .. } = event {
            xdg_surface.ack_configure(serial);
            state.configured = true;
            let surface = state.surface.as_ref().unwrap();
            if let Some(ref buffer) = state.buffer {
                surface.attach(Some(buffer), 0, 0);
                surface.commit();
            }
        }
    }
}

impl Dispatch<xdg_toplevel::XdgToplevel, ()> for AppState {
    fn event(
        state: &mut Self,
        _: &xdg_toplevel::XdgToplevel,
        event: xdg_toplevel::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        // todo
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
                if key == 16 && state.configured {
                    let surface = state.surface.as_ref().unwrap();
                    if let Some(ref buffer) = state.buffer {
                        surface.attach(Some(buffer), 0, 0);
                        surface.commit();
                    }
                }
                state.input.push_str(key_str);
                if let Ok(res) = state.engine.search(&state.input) {
                    println!("{:?}", res);
                }

                if let Some(context) = &state.context {
                    context.commit_string(state.input_serial, key_str.to_string());
                }
            }
            _ => {}
        }
    }
}

impl Dispatch<zwp_input_panel_v1::ZwpInputPanelV1, ()> for AppState {
    fn event(
        _: &mut Self,
        _: &zwp_input_panel_v1::ZwpInputPanelV1,
        _: zwp_input_panel_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        println!("input panel event");
    }
}

impl Dispatch<zwp_input_panel_surface_v1::ZwpInputPanelSurfaceV1, ()> for AppState {
    fn event(
        state: &mut Self,
        proxy: &zwp_input_panel_surface_v1::ZwpInputPanelSurfaceV1,
        event: <zwp_input_panel_surface_v1::ZwpInputPanelSurfaceV1 as wayland_client::Proxy>::Event,
        data: &(),
        conn: &Connection,
        qhandle: &QueueHandle<Self>,
    ) {
        println!("{:?}", event);
    }
}
