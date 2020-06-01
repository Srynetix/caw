//! Game of Life logic
#![allow(dead_code)]

use once_cell::sync::Lazy;
use rand::Rng;

const MAX_LIFE: u8 = 200;

static OFFSETS: Lazy<Vec<(isize, isize)>> = Lazy::new(|| {
    vec![
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ]
});

#[derive(Clone)]
pub struct GameState {
    pub width: usize,
    pub height: usize,
    pub data: Vec<bool>,
    pub life: Vec<u8>,
    pub current_tick: usize,
    pub ticks_per_cycle: usize,
    pub running: bool,
    pub stats: GameStats,
}

#[derive(Clone)]
pub struct GameStats {
    pub moving: usize,
    pub stopped: usize,
}

impl GameStats {
    pub fn new() -> Self {
        Self {
            moving: 0,
            stopped: 0,
        }
    }

    pub fn from_state(game_state: &GameState) -> Self {
        let mut stopped = 0;
        let mut moving = 0;

        for i in 0..game_state.data.len() {
            let alive = game_state.data[i];
            if alive {
                if game_state.life[i] == MAX_LIFE {
                    stopped += 1;
                } else {
                    moving += 1;
                }
            }
        }

        Self { moving, stopped }
    }
}

impl GameState {
    pub fn new((width, height): (usize, usize)) -> Self {
        Self {
            width,
            height,
            data: vec![false; width * height],
            life: vec![0; width * height],
            current_tick: 0,
            ticks_per_cycle: 1,
            running: true,
            stats: GameStats::new(),
        }
    }

    pub fn set_ticks_per_cycle(&mut self, value: usize) {
        self.ticks_per_cycle = value;
    }

    pub fn size(&self) -> usize {
        self.width * self.height
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        self.data = (0..self.size()).map(|_| rng.gen_range(0, 2) == 0).collect();
        self.life = self.data.iter().map(|_| 0).collect();
    }

    pub fn clear(&mut self) {
        self.data = vec![false; self.width * self.height];
        self.life = vec![0; self.width * self.height];
    }

    pub fn set_value_at_pos(&mut self, pos: (usize, usize), value: bool) {
        let pos = self.pos_to_index(pos);
        self.data[pos] = value;
        self.life[pos] = 0;
    }

    pub fn set_value_at_pos_with_radius(
        &mut self,
        pos: (usize, usize),
        radius: usize,
        value: bool,
    ) {
        let radius = radius as isize / 2;
        if radius == 0 {
            return self.set_value_at_pos(pos, value);
        }

        for ry in -radius..radius {
            for rx in -radius..radius {
                if rx * rx + ry * ry <= radius * radius {
                    let wrapped = self.wrap_positions((pos.0 as isize + rx, pos.1 as isize + ry));
                    self.set_value_at_pos(wrapped, value);
                }
            }
        }
    }

    pub fn index_to_pos(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }

    pub fn pos_to_index(&self, (x, y): (usize, usize)) -> usize {
        x + y * self.width
    }

    fn wrap_positions(&self, (x, y): (isize, isize)) -> (usize, usize) {
        (
            x.rem_euclid(self.width as isize) as usize,
            y.rem_euclid(self.height as isize) as usize,
        )
    }

    fn alive_neighbors_count_for_index(&self, idx: usize) -> usize {
        let (x, y) = self.index_to_pos(idx);
        OFFSETS
            .iter()
            .map(|(ox, oy)| self.wrap_positions((*ox + x as isize, oy + y as isize)))
            .map(|pos| self.data[self.pos_to_index(pos)])
            .filter(|x| *x)
            .count()
    }

    pub fn tick(&mut self) {
        let mut new_data = vec![false; self.width * self.height];

        for (idx, alive) in self.data.iter().enumerate() {
            let life = self.life[idx];
            let state = {
                let count = self.alive_neighbors_count_for_index(idx);
                count == 3 || (*alive && count == 2)
            };

            new_data[idx] = state;
            if *alive && state == *alive {
                // More life
                let new_life = if life + 1 >= MAX_LIFE {
                    MAX_LIFE
                } else {
                    life + 1
                };
                self.life[idx] = new_life;
            } else {
                self.life[idx] = 0;
            }
        }

        self.data = new_data;
        self.current_tick += 1;
    }

    pub fn cycle(&mut self) {
        if !self.running {
            return;
        }

        for _ in 0..self.ticks_per_cycle {
            self.tick();
        }

        self.stats = GameStats::from_state(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pos_to_index() {
        let state = GameState::new((10, 10));
        assert_eq!(state.pos_to_index((0, 0)), 0);
        assert_eq!(state.pos_to_index((9, 0)), 9);
        assert_eq!(state.pos_to_index((0, 9)), 90);
        assert_eq!(state.pos_to_index((9, 9)), 99);
    }

    #[test]
    fn test_index_to_pos() {
        let state = GameState::new((10, 10));
        assert_eq!(state.index_to_pos(0), (0, 0));
        assert_eq!(state.index_to_pos(9), (9, 0));
        assert_eq!(state.index_to_pos(90), (0, 9));
        assert_eq!(state.index_to_pos(99), (9, 9));
    }

    #[test]
    fn test_alive_neighbors_count_for_index() {
        let mut state = GameState::new((3, 3));

        // $+-
        // ++-
        // --+
        // => 4
        {
            state.clear();
            state.set_value_at_pos((1, 0), true);
            state.set_value_at_pos((0, 1), true);
            state.set_value_at_pos((1, 1), true);
            state.set_value_at_pos((2, 2), true);
            assert_eq!(state.alive_neighbors_count_for_index(0), 4);
        }

        // -+-
        // +$-
        // --+
        // => 3
        {
            state.clear();
            state.set_value_at_pos((1, 0), true);
            state.set_value_at_pos((0, 1), true);
            state.set_value_at_pos((2, 2), true);
            assert_eq!(
                state.alive_neighbors_count_for_index(state.pos_to_index((1, 1))),
                3
            );
        }
    }
}
