use voxl::{
    core::{
        ecs::{
            events::{Event as EventShrev, EventChannel, ReaderId},
            Resources, Schedule, World,
        },
        systems::{
            input::{input_system, MovementBindings},
            render::{camera_system, render_system},
            window::{screen_size_system, window_system},
        },
        threading::create_pool,
    },
    graph::{
        cgmath::Point3,
        gfx::{swap_chain, Render},
        uniforms::{Camera, Uniforms},
        wgpu::BackendBit,
        winit::{
            dpi::PhysicalSize,
            event::{KeyboardInput, VirtualKeyCode},
            event_loop::EventLoop,
            window::Window,
        },
    },
    time::DeltaTime,
};

fn main() {
    env_logger::init();

    let mut world = World::default();
    let mut resources = Resources::default();
    // Termination
    resources.insert(true);

    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();

    let sc_desc = swap_chain(&window.inner_size());
    let render = Render::new(BackendBit::PRIMARY, &window);
    let render_bunch = render.bunch(&sc_desc);

    let camera = Camera::new(&sc_desc);
    world.push((camera, Point3::<f32>::new(0., 0., 10.)));
    // Uniforms and SwapChainDescriptor
    resources.insert(Uniforms::default());
    resources.insert(sc_desc);

    let screen_size_reader = event_channel_init::<PhysicalSize<u32>>(&mut resources);
    let keyboard_reader = event_channel_init::<KeyboardInput>(&mut resources);

    let mut schedule = Schedule::builder()
        .add_thread_local(window_system(DeltaTime::default(), event_loop, window))
        .add_system(screen_size_system(screen_size_reader))
        .add_system(camera_system(DeltaTime::default()))
        .add_system(render_system(DeltaTime::default(), render, render_bunch))
        .add_system(input_system(
            DeltaTime::default(),
            keyboard_reader,
            MovementBindings::new(
                VirtualKeyCode::Space,
                VirtualKeyCode::LShift,
                VirtualKeyCode::Left,
                VirtualKeyCode::Right,
                VirtualKeyCode::Up,
                VirtualKeyCode::Down,
            ),
        ))
        .build();

    let pool = create_pool(22);

    while *resources
        .get::<bool>()
        .expect("please insert a `bool` resource.")
    {
        schedule.execute_in_thread_pool(&mut world, &mut resources, &pool);
    }
}

pub fn event_channel_init<T: EventShrev>(resources: &mut Resources) -> ReaderId<T> {
    let mut channel = EventChannel::<T>::with_capacity(32);
    let reader = channel.register_reader();
    resources.insert(channel);
    reader
}
