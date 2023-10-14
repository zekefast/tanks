use termion::raw::RawTerminal;
use std::io::{StdoutLock, Write};
use crate::prelude::{
    Direction,
    Position,
    Viewport,
};


const BULLET: char = '*';


pub(crate) struct Bullet {
    pub(crate) position: Position,
    direction: Direction,
}

impl Bullet {
    pub(crate) fn new(position: Position, direction: Direction) -> Bullet {
        Bullet {
            position,
            direction,
        }
    }

    fn print(&self, stdout: &mut RawTerminal<StdoutLock>) {
        write!(stdout, "{position}{}", BULLET,
            position = termion::cursor::Goto(self.position.0, self.position.1)
        ).unwrap();
    }

    fn erase(&self, stdout: &mut RawTerminal<StdoutLock>) {
        write!(stdout, "{position} ",
            position = termion::cursor::Goto(self.position.0, self.position.1)
        ).unwrap();
    }

    fn r#move(&mut self, position: Position, stdout: &mut RawTerminal<StdoutLock>) {
        self.erase(stdout);

        self.position = position;

        self.print(stdout);
    }

    pub(crate) fn go(&mut self, viewport: &Viewport, stdout: &mut RawTerminal<StdoutLock>) {
        let new_position = match self.direction {
            Direction::Up if self.position.1 > 0 =>
                (self.position.0, self.position.1 - 1),
            Direction::Down if self.position.1 < viewport.height =>
                (self.position.0, self.position.1 + 1),
            Direction::Left if self.position.0 > 0 =>
                (self.position.0 - 1, self.position.1),
            Direction::Right if self.position.0 < viewport.width =>
                (self.position.0 + 1, self.position.1),

            _ => self.position
        };

        self.r#move(new_position, stdout);
    }

    pub(crate) fn destroy(self, stdout: &mut RawTerminal<StdoutLock>) {
        self.erase(stdout);
    }
}