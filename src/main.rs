mod error;
mod prelude;
mod tank;
mod bullet;

use termion::{
    event::{
        Event, Key, parse_event
    },
    raw::{
        IntoRawMode, RawTerminal
    }
};
use std::{
    io::{
        Read,
        stdout,
        StdoutLock,
        Write
    },
    thread,
    time
};
use crate::prelude::*;

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