extern crate graphics;
extern crate piston;
extern crate piston_window;
extern crate specs;

mod render;

use piston_window::*;
use render::Sprite;
use specs::prelude::*;
use specs::Component;
use std::sync::{Arc, Mutex};

#[derive(Default, Component, Debug)]
#[storage(NullStorage)]
struct MouseTracker {}

#[derive(Default, Component, Debug)]
#[storage(VecStorage)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Debug, Default)]
struct GameState {
    exit: bool,
    delta: f64,
    mouse_position: Position,
    last_event: Option<Event>,
}

struct PhysicsSystem {}

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (mut pos_store, vel_store): Self::SystemData) {
        for (pos, vel) in (&mut pos_store, &vel_store).join() {
            (*pos).x += vel.x;
            (*pos).y += vel.y;
        }
    }
}

struct MouseTrackSystem {}
impl<'a> System<'a> for MouseTrackSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, MouseTracker>,
        Read<'a, GameState>,
    );
    fn run(&mut self, (mut pos_store, _track, gs): Self::SystemData) {
        for pos in (&mut pos_store).join() {
            (*pos).x = gs.mouse_position.x;
            (*pos).y = gs.mouse_position.y;
        }
    }
}

struct RenderSystem {
    win: Arc<Mutex<PistonWindow>>,
}

struct InputSystem {
    win: Arc<Mutex<PistonWindow>>,
}

fn handle_mouse_cursor(position: [f64; 2], gs: &mut GameState) {
    gs.mouse_position = Position {
        x: position[0] as f32,
        y: position[1] as f32,
    }
}

impl<'a> System<'a> for InputSystem {
    type SystemData = Write<'a, GameState>;
    fn run(&mut self, mut gs: Self::SystemData) {
        let mut win = self.win.lock().unwrap();
        match win.next() {
            Some(event) => {
                match &event {
                    Event::Loop(Loop::Update(args)) => gs.delta = args.dt,
                    Event::Input(_input, _opts) => match event.mouse_cursor_args() {
                        Some(cursor) => handle_mouse_cursor(cursor, &mut gs),
                        None => (),
                    },
                    _discard => {}
                }
                gs.last_event = Some(event)
            }
            None => gs.exit = true,
        }
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Sprite>,
        Read<'a, GameState>,
    );

    fn run(&mut self, (positions, sprites, gs): Self::SystemData) {
        match &gs.last_event {
            None => {}
            Some(event) => {
                let mut win = self.win.lock().unwrap();

                win.draw_2d(event, |context, graphics, _device| {
                    clear([1.; 4], graphics);
                    for (pos, sprite) in (&positions, &sprites).join() {
                        let (w, h) = (sprite.size.w, sprite.size.h);
                        println!("dt: {:?}", gs.delta);
                        rectangle(
                            sprite.color.to_array(),
                            [
                                (pos.x - sprite.pivot.x) as f64,
                                (pos.y - sprite.pivot.y) as f64,
                                w as f64,
                                h as f64,
                            ],
                            context.transform,
                            graphics,
                        );
                    }
                });
            }
        }
    }
}

fn create_world() -> World {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<MouseTracker>();
    world.register::<Sprite>();
    world
}

fn main() {
    use render::{Color, Pivot, Size};
    let mut world = create_world();
    world
        .create_entity()
        .with(Position { x: 0.0, y: 0.0 })
        .with(MouseTracker {})
        .with(Sprite {
            color: Color {
                r: 1.,
                g: 1.,
                ..Default::default()
            },
            size: Size { w: 50., h: 50. },
            pivot: Pivot { x: 25., y: 25. },
        })
        .build();

    world.insert(GameState {
        delta: 0.,
        ..Default::default()
    });

    let window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let win = Arc::new(Mutex::new(window));
    let mut dispatcher = DispatcherBuilder::new()
        .with(PhysicsSystem {}, "physics", &[])
        .with(MouseTrackSystem {}, "mouse_tracker", &[])
        .with_thread_local(InputSystem { win: win.clone() })
        .with_thread_local(RenderSystem { win: win.clone() })
        .build();

    loop {
        dispatcher.dispatch(&mut world);
        dispatcher.setup(&mut world);
        let gs = world.fetch::<GameState>();
        if gs.exit {
            println!("exit");
            break;
        }
    }
}
