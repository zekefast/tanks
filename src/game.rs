use crate::GameState;
use crate::prelude::Tank;

pub(crate) struct Game {
    pub(crate) state: GameState,
}

impl Game {
    pub(crate) fn new(state: GameState) -> Game {
        Game {
            state,
        }
    }

    pub(crate) fn add_tank(&mut self, tank: Tank) {
        self.state.tanks.push(tank);
    }
}
