pub(crate) use crate::error::Error;

pub(crate) type Result<T> = core::result::Result<T, Error>;

pub(crate) use crate::{
    bullet::Bullet,
    color::Color,
    direction::Direction,
    game::Game,
    game_state::GameState,
    tank::Tank,
    viewport::Viewport,
};

pub(crate) type Position = (u16, u16);

pub(crate) type Boundary = (Position, Position);
