extern crate piston_window;
extern crate specs;
extern crate piston;

use piston_window::*;
use specs::prelude::*;
use specs::{Component};

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
    delta: std::time::Duration,
    mouse_position: Position,
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
impl <'a> System<'a> for MouseTrackSystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, MouseTracker>, Read<'a, GameState>);
    fn run(&mut self, (mut pos_store, _track, gs): Self::SystemData) {
        for pos in (&mut pos_store).join() {
            (*pos).x = gs.mouse_position.x;
            (*pos).y = gs.mouse_position.y;
        }
    }
}

struct RenderSystem {
    win : PistonWindow,
}

fn handle_mouse_cursor(position: [f64; 2], gs: &mut GameState) {
    gs.mouse_position = Position{
        x: position[0] as f32,
        y: position[1] as f32,
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (ReadStorage<'a, Position>, Write<'a, GameState>);

    fn run(&mut self, (positions, mut gs): Self::SystemData) {
        match self.win.next() {
            Some(event) => {
                match event.mouse_cursor_args() {
                    Some(cursor) => { handle_mouse_cursor(cursor, &mut gs) }
                    None => ()
                }
                self.win.draw_2d(&event, |context, graphics, _device| {
                    clear([1.; 4], graphics);
                    for pos in positions.join() {
                        let (w, h) = (100.0, 100.0);
                        rectangle(
                            [0.0, 0.0, 0.0, 1.0], // red
                            [(pos.x - w/2.0) as f64, (pos.y - h/2.0) as f64, w as f64, h as f64],
                            context.transform,
                            graphics,
                        );
                    }
                });
            }
            None => {
                gs.exit = true
            }
        }
    }
}

fn create_world() -> World {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<MouseTracker>();
    world
}

fn main() {
    let mut world = create_world();
    world
        .create_entity()
        .with(Position { x: 0.0, y: 0.0 })
        .with(MouseTracker{})
        .build();
    
        world.insert(GameState{
            exit: false,
            delta: std::time::Duration::new(0, 0),
            mouse_position: Position{x: 0.0, y: 0.0},
        });

    let window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut dispatcher = DispatcherBuilder::new()
        .with(PhysicsSystem {}, "physics", &[])
        .with(MouseTrackSystem {}, "mouse_tracker", &[])
        .with_thread_local(RenderSystem { win: window })
        .build();

    loop {
        dispatcher.dispatch(&mut world);
        dispatcher.setup(&mut world);
        let gs = world.fetch::<GameState>();
        if gs.exit {
            println!("exit");
            break
        }
    }
}
