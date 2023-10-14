use termion::raw::RawTerminal;
use std::io::{StdoutLock, Write};
use termion::color::{Fg, Reset};
use termion::color;
use unicode_segmentation::UnicodeSegmentation;
use crate::{Boundary, Position};
use crate::prelude::{
    Bullet,
    Color,
    Direction,
    Viewport,
};


pub(crate) type TankPicture = [&'static str; 3];

const TANK_UP: TankPicture = [
    "╔═║═╗",
    "┋ ╩ ┋",
    "╚═══╝",
    // "╔═╕ ╿ ╒═╗",
    // "║ ╔═╧═╗ ║",
    // "║ ║ - ║ ║",
    // "║ ╚═══╝ ║",
    // "╚═╛   ╘═╝",
];
const TANK_DOWN: TankPicture = [
    "╔═══╗",
    "┋ ╦ ┋",
    "╚═║═╝",
    // "╔═╕   ╒═╗",
    // "║ ╔═══╗ ║",
    // "║ ║ - ║ ║",
    // "║ ╚═╤═╝ ║",
    // "╚═╛ ╽ ╘═╝",
];
const TANK_RIGHT: TankPicture = [
    "╔╍╍╍╗",
    "║ ╠══",
    "╚╍╍╍╝",
    // "╔═════╗  ",
    // "╙╔═══╗╜  ",
    // " ║ | ╟──╼",
    // "╓╚═══╝╖  ",
    // "╚═════╝  ",
];
const TANK_LEFT: TankPicture = [
    "╔╍╍╍╗",
    "══╣ ║",
    "╚╍╍╍╝",
    // "  ╔═════╗",
    // "  ╙╔═══╗╜",
    // "╾──╢ | ║ ",
    // "  ╓╚═══╝╖",
    // "  ╚═════╝",
];


#[derive(PartialEq)]
pub(crate) struct Tank {
    pub(crate) position: Position,
    pub(crate) direction: Direction,
    pub(crate) color: Color,
}

impl Tank {
    pub(crate) fn new(position: Position, direction: Direction, color: Color) -> Tank {
        Tank {
            position,
            direction,
            color,
        }
    }

    pub(crate) fn width() -> u16 {
        TANK_UP.first().unwrap().graphemes(true).count() as u16
    }

    pub(crate) fn height() -> u16 {
        TANK_UP.len() as u16
    }

    pub(crate) fn boundary(position: Position) -> Boundary {
        (
            (position.0 - Self::width()/2, position.1 - Self::height()/2),
            (position.0 + Self::width()/2, position.1 + Self::height()/2)
        )
    }

    pub(crate) fn print(&self, stdout: &mut RawTerminal<StdoutLock>) {
        for (index, &s) in self.get_tank_picture().iter().enumerate() {
            let position = termion::cursor::Goto(
                self.position.0 - Self::width()/2,
                self.position.1 - Self::height()/2 + index as u16
            );

            match self.color {
                Color::LightBlue => write!(stdout, "{position}{color}{}", s,
                    color = Fg(color::LightBlue),
                    position = position
                ),
                Color::LightRed => write!(stdout, "{position}{color}{}", s,
                    color = Fg(color::LightRed),
                    position = position
                ),
            }.unwrap();
        }

        write!(stdout, "{}", Fg(Reset)).unwrap();
    }

    pub(crate) fn erase(&self, stdout: &mut RawTerminal<StdoutLock>) {
        for index in 0..Self::height() {
            write!(stdout, "{position}{:width$}",
                " ",
                width = Self::width() as usize,
                position = termion::cursor::Goto(
                    self.position.0  - Self::width()/2,
                    self.position.1  - Self::height()/2 + index as u16
                )
            ).unwrap();
        }
    }

    fn get_tank_picture(&self) -> TankPicture {
        match self.direction {
            Direction::Up => TANK_UP,
            Direction::Down => TANK_DOWN,
            Direction::Left => TANK_LEFT,
            Direction::Right => TANK_RIGHT,
        }
    }

    pub(crate) fn r#move(&mut self, position: Position, stdout: &mut RawTerminal<StdoutLock>) {
        self.erase(stdout);

        self.position = position;

        self.print(stdout);
    }

    pub(crate) fn go(&self, viewport: &Viewport) -> Position {
        match self.direction {
            Direction::Up if self.position.1 > 2 =>
                (self.position.0, self.position.1 - 1),
            Direction::Down if self.position.1 < viewport.height - Self::height()/2 =>
                (self.position.0, self.position.1 + 1),
            Direction::Left if self.position.0 > 2 =>
                (self.position.0 - 1, self.position.1),
            Direction::Right if self.position.0 < viewport.width - Self::width()/2 =>
                (self.position.0 + 1, self.position.1),

            _ => self.position
        }
    }

    pub(crate) fn turn(&mut self, direction: Direction, stdout: &mut RawTerminal<StdoutLock>) {
        self.direction = direction;

        self.print(stdout);
    }

    pub(crate) fn is_direction(&self, direction: Direction) -> bool {
        self.direction == direction
    }

    pub(crate) fn shoot(&self, bullets: &mut Vec<Bullet>) {
        bullets.push(Bullet::new(
            match self.direction {
                Direction::Up => (self.position.0, self.position.1 - Self::height()/2 - 1),
                Direction::Down => (self.position.0, self.position.1 + Self::height()/2 + 1),
                Direction::Left => (self.position.0 - Self::width()/2 - 1, self.position.1),
                Direction::Right => (self.position.0 + Self::width()/2 + 1, self.position.1),
            },
            self.direction,
        ));
    }
}