use crate::Cell;

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

        let mut cells = vec![false; (width * height) as usize];
        let mut rng = fastrand::Rng::new();
        let mut idx = 0;

        // Out of a random `u128` number, generate 32 `bool`s at a time.
        while idx + 32 < cells.len() {
            let n = rng.u128(..);
            // There's a +0.300ns/iter penalty if you inline them.
            let even_bools = n & 0x01010101010101010101010101010101_u128;
            let odd_bools = (n & 0x10101010101010101010101010101010_u128) >> 4;

            let bools = unsafe { std::mem::transmute::<u128, [bool; 16]>(even_bools) };
            cells[idx..idx + 16].copy_from_slice(&bools);
            idx += 16;

            let bools = unsafe { std::mem::transmute::<u128, [bool; 16]>(odd_bools) };
            cells[idx..idx + 16].copy_from_slice(&bools);
            idx += 16;
        }

        // Generate the rest of them manually.
        for cell in &mut cells[idx..] {
            *cell = rng.bool();
        }

        Self {
            // SAFETY: A cell has the same states as a `bool`.
            cells: unsafe { std::mem::transmute::<Vec<bool>, Vec<Cell>>(cells) },
            height,
            width,
        }
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
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
