use crate::n::N;
use js_sys::Array;
use legion::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct Game {
    world: World,
}

// Define our entity data types
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Velocity {
    dx: f32,
    dy: f32,
}

#[wasm_bindgen]
impl Game {
    pub fn create_game() -> Game {
        // Create a world to store our entities
        let mut world = World::default();
        world.push((Position {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },));
        Game { world: world }
    }

    pub fn get_positions(&mut self) -> PositionArray {
        let mut query = <&Position>::query();
        let mut vec = Array::new();
        // you can then iterate through the components found in the world
        for position in query.iter(&self.world) {
            console_log!("{:?}", position);
            vec.push(&JsValue::from_serde(&position).unwrap());
        }
        vec.unchecked_into::<PositionArray>()
    }

    pub fn add_position(&mut self, x: f32, y: f32) {
        self.world.push((Position { x: x, y: y, z: 0.0 },));
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(typescript_type = "Array<Position>")]
    pub type PositionArray;
}
