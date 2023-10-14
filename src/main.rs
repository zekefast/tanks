mod error;
mod prelude;

use termion::{
    color::{
        self, Fg, Reset
    }, 
    event::{
        Event, Key, parse_event
    }, 
    raw::{
        IntoRawMode, RawTerminal
    }
};
use std::{
    io::{
        StdoutLock, 
        Write, 
        Read, 
        stdout
    },
    thread, 
    time
};
use unicode_segmentation::UnicodeSegmentation;

type TankPicture = [&'static str; 3];

const BANNER: &str = r#"
                                                     _..----.._
                                                    ]_.--._____[
                                                  ___|'--'__..|--._
                              __               """    ;            :
                            ()_ """"---...__.'""!":  /    ___       :
                               """---...__\]..__] | /    [ 0 ]      :
                                          """!--./ /      """        :
                                   __  ...._____;""'.__________..--..:_
                                  /  !"''''''!''''''''''|''''/' ' ' ' \"--..__  __..
                                 /  /.--.    |          |  .'          \' ' '.""--.{'.
             _...__            >=7 //.-.:    |          |.'             \ ._.__  ' '""'.
          .-' /    """"----..../ "">==7-.....:______    |                \| |  "";.;-"> \
          """";           __.."   .--"/"""""----...."""""----.....H_______\_!....'----""""]
        _..---|._ __..--""       _!.-=_.            """""""""""""""                   ;"""
       /   .-";-.'--...___     ." .-""; ';""-""-...^..__...-v.^___,  ,__v.__..--^"--""-v.^v,
      ;   ;   |'.         """-/ ./;  ;   ;\P.        ;   ;        """"____;  ;.--""""// '""<,
      ;   ;   | 1            ;  ;  '.: .'  ;<   ___.-'._.'------""""""____'..'.--""";;'  o ';
      '.   \__:/__           ;  ;--""()_   ;'  /___ .-" ____---""""""" __.._ __._   '>.,  ,/;
        \   \    /"""<--...__;  '_.-'/; ""; ;.'.'  "-..'    "-.      /"/    `__. '.   "---";
         '.  'v ; ;     ;;    \  \ .'  \ ; ////    _.-" "-._   ;    : ;   .-'__ '. ;   .^".'
           '.  '; '.   .'/     '. `-.__.' /;;;   .o__.---.__o. ;    : ;   '"";;""' ;v^" .^
             '-. '-.___.'<__v.^,v'.  '-.-' ;|:   '    :      ` ;v^v^'.'.    .;'.__/_..-'
                '-...__.___...---""'-.   '-'.;\     'WW\     .'_____..>."^"-""""""""    fsc
                                      '--..__ '"._..'  '"-;;"""
                                             """---'""""""

"#;

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

const BULLET: char = '*';

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Position = (u16, u16);
type Boundary = (Position, Position);

#[derive(PartialEq)]
enum Color {
    LightBlue,
    LightRed,
}

#[derive(PartialEq)]
struct Tank {
    position: Position,
    direction: Direction,
    color: Color,
}

impl Tank {
    fn new(position: Position, direction: Direction, color: Color) -> Tank {
        Tank {
            position,
            direction,
            color,
        }
    }

    fn width() -> u16 {
        TANK_UP.first().unwrap().graphemes(true).count() as u16
    }

    fn height() -> u16 {
        TANK_UP.len() as u16
    }

    fn boundary(position: Position) -> Boundary {
        (
            (position.0 - Self::width()/2, position.1 - Self::height()/2),
            (position.0 + Self::width()/2, position.1 + Self::height()/2)
        )
    }

    fn print(&self, stdout: &mut RawTerminal<StdoutLock>) {
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

    fn erase(&self, stdout: &mut RawTerminal<StdoutLock>) {
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

    fn r#move(&mut self, position: Position, stdout: &mut RawTerminal<StdoutLock>) {
        self.erase(stdout);

        self.position = position;

        self.print(stdout);
    }

    fn go(&self, viewport: &Viewport) -> Position {
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

    fn turn(&mut self, direction: Direction, stdout: &mut RawTerminal<StdoutLock>) {
        self.direction = direction;

        self.print(stdout);
    }

    fn is_direction(&self, direction: Direction) -> bool {
        self.direction == direction
    }

    fn shoot(&self, bullets: &mut Vec<Bullet>) {
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
struct Viewport {
    width: u16,
    height: u16,
}

impl Viewport {
    fn new(width: u16, height: u16) -> Viewport {
        Viewport {
            width,
            height,
        }
    }

    fn from_terminal() -> Viewport {
        let (width, height) = termion::terminal_size()
            .expect("Can't read terminal size.");

        Viewport::new(width, height)
    }
}

struct Bullet {
    position: Position,
    direction: Direction,
}

impl Bullet {
    fn new(position: Position, direction: Direction) -> Bullet {
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

    fn go(&mut self, viewport: &Viewport, stdout: &mut RawTerminal<StdoutLock>) {
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

    fn destroy(self, stdout: &mut RawTerminal<StdoutLock>) {
        self.erase(stdout);
    }
}

struct GameState {
    viewport: Viewport,
    tanks: Vec<Tank>,
}

impl GameState {
    fn new(viewport: Viewport) -> GameState {
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

    fn update_tank(&mut self, tank_index: usize, direction: Direction, stdout: &mut RawTerminal<StdoutLock>) {
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

struct Game {
    state: GameState,
}

impl Game {
    fn new(state: GameState) -> Game {
        Game {
            state,
        }
    }

    fn add_tank(&mut self, tank: Tank) {
        self.state.tanks.push(tank);
    }
}

fn main() {
    println!("{}", BANNER);
    thread::sleep(time::Duration::from_millis(5000));
    let mut stdin = termion::async_stdin().bytes();
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    write!(stdout, "{}{}",
           termion::clear::All,
           termion::cursor::Hide).unwrap();
    stdout.flush().unwrap();
    
    let mut game = Game::new(
        GameState::new(
            Viewport::from_terminal()
        )
    );

    game.add_tank(Tank::new(
        (Tank::width()/2, Tank::height()/2 + 1), 
        Direction::Right,
        Color::LightBlue
    ));
    game.add_tank(Tank::new(
        (50, 16), 
        Direction::Left,
        Color::LightRed
    ));

    let mut bullets = Vec::new();

    game.state.tanks[0].print(&mut stdout);
    game.state.tanks[1].print(&mut stdout);

    stdout.flush().unwrap();

    loop {
        if let Some(Ok(buf)) = stdin.next() {
            match parse_event(buf, &mut stdin) {
                Ok(Event::Key(Key::Char('q'))) => break,
                
                Ok(Event::Key(Key::Left)) => {
                    game.state.update_tank(0, Direction::Left, &mut stdout); 
                }
                Ok(Event::Key(Key::Right)) => {
                    game.state.update_tank(0, Direction::Right, &mut stdout);
                }
                Ok(Event::Key(Key::Up)) => {
                    game.state.update_tank(0, Direction::Up, &mut stdout);
                }
                Ok(Event::Key(Key::Down)) => {
                    game.state.update_tank(0, Direction::Down, &mut stdout);
                }
                Ok(Event::Key(Key::Char(' '))) => {
                    game.state.tanks[0].shoot(&mut bullets);
                }
    
                Ok(Event::Key(Key::Char('d'))) => {
                    game.state.update_tank(1, Direction::Right,  &mut stdout);
                }
                Ok(Event::Key(Key::Char('a'))) => {
                    game.state.update_tank(1, Direction::Left,  &mut stdout);
                }
                Ok(Event::Key(Key::Char('w'))) => {
                    game.state.update_tank(1, Direction::Up,  &mut stdout);
                }
                Ok(Event::Key(Key::Char('s'))) => {
                    game.state.update_tank(1, Direction::Down,  &mut stdout);
                }
                Ok(Event::Key(Key::Char('`'))) => {
                    game.state.tanks[1].shoot(&mut bullets);
                }
    
                _ => {},
            }
        }

        let mut bullet_indexes_on_edge = Vec::new();
        for (index, bullet) in bullets.iter_mut().enumerate() {
            bullet.go(&game.state.viewport, &mut stdout);     
            
            if bullet.position.0 <= 1 || 
                    bullet.position.0 >= game.state.viewport.width ||
                        bullet.position.1 <= 1 ||
                            bullet.position.1 >= game.state.viewport.height
            {
                bullet_indexes_on_edge.push(index);
            }
        }

        for bullet_index in bullet_indexes_on_edge {
            bullets.remove(bullet_index).destroy(&mut stdout);
        }

        stdout.flush().unwrap();

        thread::sleep(time::Duration::from_millis(50));
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}