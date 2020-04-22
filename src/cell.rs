use amethyst::{
    ecs::{Read, System},
    input::{InputHandler, StringBindings},
};

pub struct CellSystem {
    width: u32,
    height: u32,
}

impl CellSystem {
    pub fn new(width: u32, height: u32) -> Self {
        CellSystem { width, height }
    }
}

struct CellState {}

impl<'a> System<'a> for CellSystem {
    type SystemData = (Read<'a, InputHandler<StringBindings>>);

    fn run(&mut self, data: Self::SystemData) {
    }
}
