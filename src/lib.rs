mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;

extern crate web_sys;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// implementation of Game of life cell
#[wasm_bindgen]
#[repr(u8)] // reprecent the cell as a u8
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    // The cell has two states; dead: 0, alive: 1
    Dead = 0,
    Alive = 1,
}

// Implement a toggle method so js can toggle cells by clicking
impl Cell {
    fn toggle(&mut self) {
        // match the current state and invert
        // and change the self state
        *self = match *self {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
        };
    }
}
// Lets define the universe, the universe has a
// height, width and a vector of cells
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

// impement the fmt::Display trait on universe
// so we can do text rendering using unicode
// chars;  ◼ ("black medium square"). 
// For dead cells, we'll print ◻ (a "white medium square").
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // get the cell vector
        let cells = self.cells.clone();
        // iterate over the cell vector by first converting
        // it from Vec literal to vec slice and chuncking
        // according to the univers width
        for line in cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                // cell refers to cell enum type so we can compare
                let symbol = if cell == Cell::Alive {'◼'} else {'◻'};
                // write the symbol using write macro
                // write and unwrap the results for exceptions ("?")
                write!(f, "{}", symbol)?;
            }
            // write a new line
            // write and unwrap the results for exceptions ("?")
            write!(f, "\n")?;
        }
        // Return an ok result
        // when all goes good
        Ok(())
    }
}

// Now lets implement the functionalities of the
// universe
// since javascript should have the ability to modify
// the tick function (tick is an instance of the discrete universe)
// we use wasm_bindgen attribute
#[wasm_bindgen]
impl Universe {
    // Universe constructor
    pub fn new() -> Universe {
        // Initialize a panick hook in the constructor
        // so we can console log the panicks
        utils::set_panic_hook();

        let height = 64;
        let width = 64;
        // Create an initial cell pattern for the universe
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    // return Cell::Alive so the
                    // collector collects it
                    Cell::Alive
                }
                else {
                    // return Cell::Dead so the
                    // collector collects it
                    Cell::Dead
                }
            }).collect();
        
        // Initalize the new universe
        Universe {
            width,
            height,
            cells,
        }
    }
    // get width
    pub fn width(&self) -> u32 {
        self.width
    }
    // get height
    pub fn height(&self) -> u32 {
        self.height
    }
    
    // A text render for the universe
    pub fn render(&self) -> String {
        self.to_string()
    }
    
    pub fn cells(&self) -> *const Cell {
        // return a pointer to the start of the cell vector
        // js consumes the pointer from the wasm linear memory
        // and render it on the canvas.
        self.cells.as_ptr()
    }

    // lets set some setters and getters to have different size universes
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        // initiate all the cells to dead
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        // initiate all the cells to dead
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }
    // the tick function below modifies a cell for the next tick of the
    // universe; the cell can be die, stay alive or reborn.
    // the cell modification rules are as follows,
    
    // Rule 1: Any live cell with fewer than two live neighbours
    // dies, as if caused by underpopulation.

    // Rule 2: Any live cell with two or three live neighbours
    // lives on to the next generation.

    // Rule 3: Any live cell with more than three live
    // neighbours dies, as if by overpopulation.

    // Rule 4: Any dead cell with exactly three live neighbours
    // becomes a live cell, as if by reproduction.

    // All other cells remain in the same state.
    pub fn tick(&mut self) {
        // get the flat vect of cells in the universe
        let mut next = self.cells.clone();
        // Iterate over the universe grid
        for row in 0..self.height {
            for col in 0..self.width {
                // get the current flat index
                let idx = self.get_index(row, col);
                // get the current cell
                let cell = self.cells[idx];
                // find the living neighbors
                let neighbors_alive = self.live_neighbor_count(row, col);
                // Now do a pattern matching using the current cell and its 
                // living neighbors to find the next cell state according to the rules
                let next_cell = match (cell, neighbors_alive) {
                    // RULE: 1
                    // if the current cell alive and neighbors < 2 current cell in the
                    // next tick dies
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // RULE: 2
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule: 3
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule: 4
                    (Cell::Dead, 3) => Cell::Alive,
                    // the non-exhaustive patter, all the other cells
                    // in the universe remains the same
                    (otherwise, _) => otherwise,
                };
                // update the state of the cell for the next tick
                next[idx] = next_cell;
            }
        }
        // Initialize the Universe structure with the current status
        self.cells = next;
    }

    // given the row and column find the
    // flatten index of the cell
    fn get_index(&self, row: u32, column: u32) -> usize {
        // idx = row * width + col
        (row * self.width + column) as usize
    }
    // count the live neighbors around a given cell
    // located at a given row col index
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        // a mutable to hold the count
        let mut count = 0;
        // iterate using deltas
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue
                }
                // use modulo to handle the univers edges
                // in this case the neighbor of an edge cell will
                // be the edge cell at the other side of the universe
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                // get the vector index of the neighbor row and col
                let idx = self.get_index(neighbor_row, neighbor_col);
                // update the count by getting the alive neighbor cells
                // if alive: +=1 increase count
                // if dead: +=0 do nothing
                count += self.cells[idx] as u8;
            }
        }
        count
    }
    // function that updates the universe with the new toggled state
    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        // get flat idx
        let idx = self.get_index(row, col);
        // toggle the cell
        self.cells[idx].toggle();
    }
}

// Here we implement a part of univers that does not expose to Javascript
// the reason is rust wasm cant return references. So we do rust level 
// testing of the functionality
impl Universe {
    // get cells: returns a reference to a vector slice the cells
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }
    // set cells to be alive by taking a list of row, col tuples
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        // iterate over the cells, get row, cols and set the corresponding
        // cells alive
        for (row, col) in cells.iter().cloned() {
            // get the flat index of the cell
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }

}

// A Println! like macro using tocken trees (tt) to console log in js
macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(format!($(tt)*).into());
    };
}