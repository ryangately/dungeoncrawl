use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {

    // fill all tiles with the given type
    pub fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    // carve out rooms into the map
    // (this is assuming that the map is filled with non-walkable tiles)
    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {

        // TODO: refactor this book code
        // we could get stuck in an infinite loop here if the randomly
        // generated room doesn't fit inside the arbitrary map size
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            if !self.rooms_overlap(&room) {
                // turn each point inside the room into a walkable tile
                room.for_each(|p| {
                    if self.map.in_bounds(p) {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
                self.rooms.push(room);
            }
        }
    }

    // test for intersection of a room with other existing rooms
    // (extracted function compared to the book)
    fn rooms_overlap(&self, room: &Rect) -> bool {
        for r in self.rooms.iter() {
            if r.intersect(room) {
                return true;
            }
        }
        false
    }
}