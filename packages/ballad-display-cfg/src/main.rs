use std::cell::Cell;

use smithay_client_toolkit::{delegate_output, delegate_registry, output::{OutputHandler, OutputState}, reexports::protocols_wlr::gamma_control::v1::client::{zwlr_gamma_control_manager_v1::ZwlrGammaControlManagerV1, zwlr_gamma_control_v1::{self, ZwlrGammaControlV1}}, registry::{ProvidesRegistryState, RegistryState}, registry_handlers};
use wayland_client::{
    delegate_noop, globals::{registry_queue_init, GlobalList}, Connection, Dispatch, QueueHandle
};

/// Wayland client state
struct ClientState {
    registry_state: RegistryState,
    output_state: OutputState,
    pub running: bool,
}
impl ClientState {
    fn new(registry: &GlobalList, queue_handle: &QueueHandle<Self>) -> Self {
        Self {
            output_state: OutputState::new(registry, queue_handle),
            registry_state: RegistryState::new(registry),
            running: true,
        }
    }
}

struct GammaControlState {
    gamma_step_count: Cell<u32>,
}

impl Dispatch<ZwlrGammaControlV1, GammaControlState> for ClientState {
    fn event(
        state: &mut Self,
        proxy: &ZwlrGammaControlV1,
        event: <ZwlrGammaControlV1 as wayland_client::Proxy>::Event,
        data: &GammaControlState,
        _conn: &Connection,
        _qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        match event 
        {
            zwlr_gamma_control_v1::Event::GammaSize { size } => {
                data.gamma_step_count.set(size);
            },
            zwlr_gamma_control_v1::Event::Failed => {
                println!("Failed to set gamma control.");
                proxy.destroy()
            },
            _ => unreachable!(),
        }
    }
}

impl ProvidesRegistryState for ClientState {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }
    registry_handlers![];
}

delegate_registry!(ClientState);

impl OutputHandler for ClientState {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.output_state
    }

    fn new_output(
        &mut self,
        conn: &Connection,
        qh: &wayland_client::QueueHandle<Self>,
        output: wayland_client::protocol::wl_output::WlOutput,
    ) {
        println!("New output: {:?}", output);
    }

    fn update_output(
        &mut self,
        conn: &Connection,
        qh: &wayland_client::QueueHandle<Self>,
        output: wayland_client::protocol::wl_output::WlOutput,
    ) {
        
    }

    fn output_destroyed(
        &mut self,
        conn: &Connection,
        qh: &wayland_client::QueueHandle<Self>,
        output: wayland_client::protocol::wl_output::WlOutput,
    ) {
        
    }
}

delegate_output!(ClientState);
delegate_noop!(ClientState: ZwlrGammaControlManagerV1);

fn main() {
    let connection = Connection::connect_to_env().expect("Failed to find a Wayland socket.");

    
    let (registry, mut event_queue) =
    registry_queue_init(&connection).expect("Failed to init registry");

    let mut client_state = ClientState::new(&registry, &event_queue.handle());

    while client_state.running {
        event_queue
            .blocking_dispatch(&mut client_state)
            .expect("Wayland connection lost!");
    }
}
