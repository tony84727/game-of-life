use amethyst::{
    assets::{Handle, Prefab, PrefabData, PrefabLoader, ProgressCounter, RonFormat},
    core::Transform,
    derive::PrefabData,
    ecs::{
        Component, DenseVecStorage, Entities, Entity, Join, Read, System, World, Write,
        WriteStorage,
    },
    error::Error,
    prelude::*,
    renderer::{
        formats::GraphicsPrefab,
        rendy::util::types::vertex::{Normal, Position, TexCoord},
    },
};
use serde::{Deserialize, Serialize};

struct CellState {}

struct Cell {
    id: u32,
}

impl Component for Cell {
    type Storage = DenseVecStorage<Self>;
}

pub struct AutomataControl {
    running: bool,
    size: Size,
}

impl Default for AutomataControl {
    fn default() -> Self {
        AutomataControl {
            running: false,
            size: Size(25, 25),
        }
    }
}

pub struct GridID(usize, usize);

impl GridID {
    pub fn row(&self) -> usize {
        self.0
    }
    pub fn column(&self) -> usize {
        self.1
    }
}

pub struct CellTag {
    pub id: GridID,
}

impl Component for CellTag {
    type Storage = DenseVecStorage<Self>;
}

struct Simulation {
    states: Vec<Vec<bool>>,
}

impl Simulation {
    pub fn new(width: u32, height: u32) -> Self {
        let states = vec![vec![false; width as usize]; height as usize];
        Simulation { states }
    }

    pub fn get_size(&self) -> (u32, u32) {
        let columns = self
            .states
            .first()
            .expect("cannot determine the width of the state");
        (columns.len() as u32, self.states.len() as u32)
    }

    fn next(&mut self) {
        let mut scores = vec![
            vec![
                0;
                self.states
                    .first()
                    .expect("cannot determine the width of the state")
                    .len()
            ];
            self.states.len()
        ];
        for r in 0..self.states.len() {
            let columns = &self.states[r];
            for c in 0..columns.len() {
                if self.states[r][c] {
                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }
                            let sx = r as i32 + dx;
                            let sy = c as i32 + dy;
                            if sx >= 0
                                && sx < columns.len() as i32
                                && sy >= 0
                                && sy < self.states.len() as i32
                            {
                                scores[sx as usize][sy as usize] += 1;
                            }
                        }
                    }
                }
            }
        }
        for r in 0..self.states.len() {
            for c in 0..self.states[r].len() {
                let score = scores[r][c];
                self.states[r][c] = score > 1 && score <= 4;
            }
        }
    }
}

pub struct AutomataGrid(Vec<Vec<bool>>);

impl Default for AutomataGrid {
    fn default() -> Self {
        AutomataGrid(Vec::new())
    }
}

pub struct CellSystemDesc;

impl<'a> SystemDesc<'a, 'a, CellSystem> for CellSystemDesc {
    fn build(self, world: &mut World) -> CellSystem {
        world.insert(AutomataGrid);
        CellSystem
    }
}

/// CellSystem is in charge to run the simulation of the cellar automata. It also need to obey
/// [AutomataControl].
pub struct CellSystem;

impl<'a> System<'a> for CellSystem {
    type SystemData = (Read<'a, AutomataControl>, Write<'a, AutomataGrid>);

    fn run(&mut self, (control, mut grid): Self::SystemData) {}
}

#[derive(PartialEq, Copy, Clone)]
struct Size(usize, usize);

/// CellDisplaySystem in charge of spawning entities to represent the cells in the the grid
pub struct CellDisplaySystem {
    previous_grid_size: Size,
    // distance of cells
    distance: f32,
    prefab: Handle<Prefab<CellPrefabData>>,
}

impl<'a> System<'a> for CellDisplaySystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, CellTag>,
        WriteStorage<'a, Handle<Prefab<CellPrefabData>>>,
        WriteStorage<'a, Transform>,
        Read<'a, AutomataControl>,
    );

    fn run(&mut self, (entities, mut tags, mut prefabs, mut transform, control): Self::SystemData) {
        self.maintain_cell_entities(&entities, &mut tags, &mut prefabs, &control);
        for (c, mut transform) in (&tags, &mut transform).join() {
            transform.set_translation_x(c.id.row() as f32 * self.distance);
            transform.set_translation_y(c.id.column() as f32 * self.distance);
            transform.set_translation_z(-50.);
        }
    }
}

impl CellDisplaySystem {
    fn maintain_cell_entities(
        &mut self,
        entities: &Entities<'_>,
        cells: &mut WriteStorage<'_, CellTag>,
        prefab: &mut WriteStorage<'_, Handle<Prefab<CellPrefabData>>>,
        control: &AutomataControl,
    ) {
        if self.is_grid_size_changed(control) {
            self.remove_all_cells(entities, &cells);
            self.add_cells(entities, cells, prefab, control.size);
            self.previous_grid_size = control.size;
        }
    }

    fn is_grid_size_changed(&self, control: &AutomataControl) -> bool {
        control.size != self.previous_grid_size
    }

    fn remove_all_cells(&self, entities: &Entities<'_>, cells: &WriteStorage<'_, CellTag>) {
        for (e, _) in (entities, cells).join() {
            entities.delete(e).unwrap();
        }
    }

    fn add_cells(
        &self,
        entities: &Entities<'_>,
        cells: &mut WriteStorage<'_, CellTag>,
        prefab: &mut WriteStorage<'_, Handle<Prefab<CellPrefabData>>>,
        size: Size,
    ) {
        for r in 0..size.0 {
            for c in 0..size.1 {
                entities
                    .build_entity()
                    .with(CellTag { id: GridID(r, c) }, cells)
                    .with(self.prefab.clone(), prefab)
                    .build();
            }
        }
    }
}

pub struct CellDisplaySystemDesc;

impl<'a> SystemDesc<'a, 'a, CellDisplaySystem> for CellDisplaySystemDesc {
    fn build(self, world: &mut World) -> CellDisplaySystem {
        let prefab = world.exec(|loader: PrefabLoader<CellPrefabData>| {
            loader.load("prefabs/cell.ron", RonFormat, ())
        });
        CellDisplaySystem {
            previous_grid_size: Size(0, 0),
            distance: 3.,
            prefab,
        }
    }
}

#[derive(PrefabData, Serialize, Deserialize)]
pub struct CellPrefabData {
    transform: Transform,
    graphics: GraphicsPrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>,
}

struct GridMap<V> {
    width: usize,
    height: usize,
    storage: Vec<V>,
}

impl<V> GridMap<V>
where
    V: Default + Clone,
{
    fn new(width: usize, height: usize) -> Self {
        let mut storage = Vec::new();
        storage.resize(width * height, V::default());
        GridMap {
            width,
            height,
            storage,
        }
    }
    fn add(&mut self, k: GridID, value: V) {
        let index = self.get_index(k);
        self.storage.insert(index, value);
    }

    fn get(&self, k: GridID) -> &V {
        &self.storage[self.get_index(k)]
    }

    fn get_index(&self, id: GridID) -> usize {
        id.row() * self.width + self.height
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_grid_map() {
        let map = {
            let mut m = GridMap::new(3, 3);
            m.add(GridID(0, 0), 1);
            m.add(GridID(2, 2), 9);
            m
        };
        assert_eq!(1, *map.get(GridID(0, 0)));
        assert_eq!(9, *map.get(GridID(2, 2)));
    }
}
