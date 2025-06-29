use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, IntoStaticStr};

#[derive(
    Clone, IntoStaticStr, Debug, Hash, PartialEq, PartialOrd, Eq, EnumCount, EnumIter, Copy,
)]
pub enum Team {
    A,
    B,
}

impl Team {}

impl Team {
    pub fn to_index(&self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
        }
    }

    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Self::A,
            1 => Self::B,
            _ => panic!("Invalid index for converting to Team"),
        }
    }

    pub fn get_enemy_team(&self) -> Self {
        match self {
            Self::A => Self::B,
            Self::B => Self::A,
        }
    }

    pub fn from_player_index(player_index: usize) -> Self {
        let index = player_index % Team::COUNT;
        Self::from_index(index)
    }
}
