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

#[derive(Debug, Clone, Default)]
pub struct TeamPoints {
    points: [usize; Team::COUNT],
}

impl TeamPoints {
    pub fn add_points(&mut self, team: Team, points: usize) {
        let index = team.to_index();
        self.points[index] += points;
    }

    pub fn has_bigger_points(&self, team: Team) -> bool {
        let team_index = team.to_index();
        let other_team_index = (team_index + 1) % Team::COUNT;

        self.points[team_index] > self.points[other_team_index]
    }

    pub fn all_points_to_team(&mut self, team: Team) {
        let team_index = team.to_index();
        let other_team_index = (team_index + 1) % Team::COUNT;
        let points_sum = self.points.iter().fold(0, |acc, curr| acc + *curr);
        self.points[team_index] = points_sum;
        self.points[other_team_index] = 0;
    }

    pub fn get_points(&self, team: Team) -> usize {
        self.points[team.to_index()]
    }
}
