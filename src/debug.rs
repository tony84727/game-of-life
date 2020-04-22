use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System},
};

use crate::cell::CellTag;

pub struct PrintCellsTransform {
    counter: u32,
}

impl PrintCellsTransform {
    pub fn new() -> Self {
        PrintCellsTransform { counter: 0 }
    }
}

impl<'a> System<'a> for PrintCellsTransform {
    type SystemData = (ReadStorage<'a, CellTag>, ReadStorage<'a, Transform>);

    fn run(&mut self, (cells, transforms): Self::SystemData) {
        self.counter = self.counter + 1;
        if self.counter < 100 {
            return;
        }
        self.counter = 0;
        for (cell, transform) in (&cells, &transforms).join() {
            let translate = transform.translation();
            println!(
                "cell: ({}, {}), {}",
                cell.id.row(),
                cell.id.column(),
                translate
            );
        }
    }
}
