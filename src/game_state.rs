use termion::raw::RawTerminal;
use std::io::StdoutLock;
use crate::prelude::{
    Direction,
    Position,
    Tank,
    Viewport,
};

pub(crate) struct GameState {
    pub(crate) viewport: Viewport,
    pub(crate) tanks: Vec<Tank>,
}

impl GameState {
    pub(crate) fn new(viewport: Viewport) -> GameState {
        GameState {
            viewport,
            tanks: Vec::new(),
        }
    }

    fn can_move<'a>(&self, tanks: impl Iterator<Item = &'a Tank>, new_position: Position) -> bool {
        let new_boundary = Tank::boundary(new_position);

        let mut result = true;
        for tank in tanks {
            let tank_boundary = Tank::boundary(tank.position);

            result = result && (
                new_boundary.0.0 < tank_boundary.0.0 || new_boundary.0.0 > tank_boundary.1.0 ||
                new_boundary.0.1 < tank_boundary.0.1 || new_boundary.0.1 > tank_boundary.1.1
            ) && (
                new_boundary.1.0 < tank_boundary.0.0 || new_boundary.1.0 > tank_boundary.1.0 ||
                new_boundary.0.1 < tank_boundary.0.1 || new_boundary.0.1 > tank_boundary.1.1
            ) && (
                new_boundary.0.0 < tank_boundary.0.0 || new_boundary.0.0 > tank_boundary.1.0 ||
                new_boundary.1.1 < tank_boundary.0.1 || new_boundary.1.1 > tank_boundary.1.1
            ) && (
                new_boundary.1.0 < tank_boundary.0.0 || new_boundary.1.0 > tank_boundary.1.0 ||
                new_boundary.1.1 < tank_boundary.0.1 || new_boundary.1.1 > tank_boundary.1.1
            );
        }

        result
    }

    pub(crate) fn update_tank(&mut self, tank_index: usize, direction: Direction, stdout: &mut RawTerminal<StdoutLock>) {
        let tanks = self.tanks.iter()
            .filter(|&tank| *tank != self.tanks[tank_index]);

        if self.tanks[tank_index].is_direction(direction) {
            let new_position = self.tanks[tank_index].go(&self.viewport);

            if self.can_move(tanks, new_position) {
                self.tanks[tank_index].r#move(new_position, stdout);
            }
        } else {
            self.tanks[tank_index].turn(direction, stdout);
        }
    }
}
