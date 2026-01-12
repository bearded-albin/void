#![forbid(unsafe_code)]

/*
Purpose: 3D grid management, coordinate/index helpers, neighbor queries.

Uses CellState, LatticeCoord, Direction from types.
Called by: init, transport, evolution, visualization, conservation, oscillation.
*/

use crate::types::{CellState, FORCES, LatticeCoord, VARS};

/**/
#[derive(Default)]
pub struct Lattice {
    size: (usize, usize, usize),
    cells: Vec<CellState>,
}

/**/
impl Lattice {
    /*
    Create empty lattice.
    All energy is 0.0
    */
    pub fn new(size: (usize, usize, usize)) -> Option<Lattice> {
        let cell_count = Self::cell_count_size(size);
        let mut cells: Vec<CellState> = vec![];
        for _ in 0..cell_count? {
            // Double check 0 or 1 start
            let cell_state = CellState {
                e: [[0.0; FORCES]; VARS],
            };
            cells.push(cell_state);
        }
        Some(Lattice { size, cells })
    }

    /*
    Return lattice dimensions.
    */
    pub fn size(&self) -> (usize, usize, usize) {
        self.size
    }

    /**/
    pub fn cell_count_size(size: (usize, usize, usize)) -> Option<u128> {
        let cell_count = (size.0 as u128)
            .checked_mul(size.1 as u128)?
            .checked_mul(size.2 as u128)?;
        Some(cell_count)
    }

    /*
    Convert coordinates → index.
    */
    pub fn index(&self, coord: LatticeCoord) -> Option<u128> {
        let cell_count = Self::cell_count_size(self.size);
        let layer = cell_count?
            .checked_mul(cell_count?)?
            .checked_mul(coord.z as u128)?;
        let row = cell_count?.checked_mul(coord.y as u128)?;
        (coord.x as u128).checked_add(row)?.checked_add(layer)
    }

    /*
    Convert index → coordinates.
    */
    pub fn coord(&self, index: u128) -> Option<LatticeCoord> {
        let n = Self::cell_count_size(self.size)?;
        let n2 = n.checked_mul(n)?;
        let x = index.checked_div(n2)?;
        let r = index.checked_rem(n2)?;
        let y = r.checked_div(n)?;
        let z = r.checked_rem(n)?;
        Some(LatticeCoord {
            x: x as usize,
            y: y as usize,
            z: z as usize,
        })
    }

    /*
    TODO
    */
    pub fn at(&self, coord: LatticeCoord) -> Option<&CellState> {
        todo!();
    }

    /*
    TODO
    */
    pub fn at_mut(&mut self, coord: LatticeCoord) -> Option<&mut CellState> {
        todo!();
    }

    /**/
    pub fn in_bounds(&self, coord: &LatticeCoord) -> bool {
        coord.x < self.size.0 && coord.y < self.size.1 && coord.z < self.size.2
    }

    /*
    TODO
    Apply periodic boundary conditions.
    */
    pub fn periodic_coord(&self, coord: LatticeCoord) -> LatticeCoord {
        todo!();
    }

    /*
    Up to 6 neighbors.
    */
    pub fn neighbors_6(&self, coord: LatticeCoord) -> Option<Vec<LatticeCoord>> {
        let mut neighbors: Vec<LatticeCoord> = vec![];
        if self.in_bounds(&coord) {
            neighbors.push(LatticeCoord { x: coord.x.checked_add(1)?, ..coord });
            neighbors.push(LatticeCoord { x: coord.x.checked_sub(1)?, ..coord });
            neighbors.push(LatticeCoord { y: coord.y.checked_add(1)?, ..coord });
            neighbors.push(LatticeCoord { y: coord.y.checked_sub(1)?, ..coord });
            neighbors.push(LatticeCoord { z: coord.z.checked_add(1)?, ..coord });
            neighbors.push(LatticeCoord { z: coord.z.checked_sub(1)?, ..coord });
            Some(neighbors)
        }
        else { None }
    }

    /*
    TODO
    */
    pub fn neighbors_26(&self, coord: LatticeCoord) -> Vec<LatticeCoord> {
        todo!();
    }

    /*
    TODO
    */
    pub fn iter_cells(&self) -> impl Iterator<Item = (LatticeCoord, &CellState)> {
        todo!();
    }

    /*
    TODO
    */
    pub fn iter_cells_mut(&mut self) -> impl Iterator<Item = (LatticeCoord, &mut CellState)> {
        todo!();
    }
}
