use state::State;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod state;

fn main() {
    pollster::block_on(run());
}

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("My new window")
        .build(&event_loop)
        .unwrap();
    let mut state = State::new(&window).await;
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            window_id,
            ref event,
        } => match event {
            WindowEvent::CloseRequested {} => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(physical_size) => {
                state.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                state.resize(**new_inner_size);
            }
            WindowEvent::CursorMoved {
                device_id,
                position,
                modifiers,
            } => {
                state.input();
            }
            _ => {}
        },
        Event::RedrawRequested(window_id) => {
            state.render(1.0,2.0,3.0).unwrap();
        }

        _ => {}
    })
}
