use crate::{timer::Timer, Cell};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub struct Universe {
    pub width: u32,
    pub height: u32,
    cells: Vec<Cell>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
impl Universe {
    pub fn new(width: u32, height: u32) -> Self {
        debug!(%width, %height, "Creating new universe");

        let cells = (0..width * height)
            .map(|#[cfg_attr(target_arch = "wasm32", allow(unused))] idx| {
                #[cfg(target_arch = "wasm32")]
                if js_sys::Math::random() < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }

                #[cfg(not(target_arch = "wasm32"))]
                if idx % 13 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Self {
            width,
            height,
            cells,
        }
    }

    pub fn tick(&mut self) {
        let _t = Timer::new("universe::tick");

        let mut next = {
            let _t = Timer::new("universe::clone_cells");

            self.cells.clone()
        };

        for row in 0..self.height {
            for col in 0..self.width {
                let _t = Timer::new("universe::process_single_cell");

                let idx = self.index_of_cell(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                next[idx] = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2 | 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (x, _) => x,
                };
            }
        }

        self.cells = next;
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

impl Universe {
    fn index_of_cell(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let north = if row == 0 { self.height - 1 } else { row - 1 };
        let south = if row == self.height - 1 { 0 } else { row + 1 };
        let west = if col == 0 { self.width - 1 } else { col - 1 };
        let east = if col == self.width - 1 { 0 } else { col + 1 };
        let mut count = 0u8;

        count += self.cells[self.index_of_cell(north, west)] as u8; // NW
        count += self.cells[self.index_of_cell(north, col)] as u8; // N
        count += self.cells[self.index_of_cell(north, east)] as u8; // NE
        count += self.cells[self.index_of_cell(row, west)] as u8; // W
        count += self.cells[self.index_of_cell(row, east)] as u8; // E
        count += self.cells[self.index_of_cell(south, west)] as u8; // SW
        count += self.cells[self.index_of_cell(south, col)] as u8; // S
        count += self.cells[self.index_of_cell(south, east)] as u8; // SE

        count
    }
}
