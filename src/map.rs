use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    // single dimensional row-first (why isn't this a 2d vector?)
    // [0][1][2][3][4]
    // [5][6][7][8][9]
    // in this example, (y * 5) + x will transform coords to the index
    pub tiles: Vec<TileType>,
}

// retrieve index of map (aka striding, according to the book)
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        // iterating over y first is faster
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK,
                            to_cp437('.')
                        );
                    }
                    TileType::Wall => {
                        ctx.set(x, y, GREEN, BLACK,
                            to_cp437('#')
                        );
                    }
                }
            }
        }
    }

    // true if the point is within the bounds of the map
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x > 0 && point.x < SCREEN_WIDTH
            && point.y > 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point)
            && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    // returns the tile index for a point
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if self.in_bounds(point) {
            Some(map_idx(point.x, point.y))
        } else {
            None
        }
    }
}
