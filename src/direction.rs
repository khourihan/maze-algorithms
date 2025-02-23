use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Neg, Not};

use glam::UVec2;
use rand::seq::IndexedRandom;

#[derive(Clone, Copy, Debug)]
pub struct Directions {
    pub east: bool,
    pub north: bool,
    pub west: bool,
    pub south: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    East = 0,
    North = 1,
    West = 2,
    South = 3,
}

impl Directions {
    pub const NONE: Directions = Directions::new(false, false, false, false);
    pub const ALL: Directions = Directions::new(true, true, true, true);
    pub const EAST: Directions = Directions::new(true, false, false, false);
    pub const NORTH: Directions = Directions::new(false, true, false, false);
    pub const WEST: Directions = Directions::new(false, false, true, false);
    pub const SOUTH: Directions = Directions::new(false, false, false, true);

    #[inline(always)]
    pub const fn new(east: bool, north: bool, west: bool, south: bool) -> Directions {
        Directions {
            east,
            north,
            west,
            south,
        }
    }

    #[inline]
    pub fn from_fn<F>(f: F) -> Directions
    where
        F: Fn(Direction) -> bool,
    {
        Directions {
            east: f(Direction::East),
            north: f(Direction::North),
            west: f(Direction::West),
            south: f(Direction::South),
        }
    }

    #[inline]
    pub fn contains(self, direction: Direction) -> bool {
        match direction {
            Direction::East => self.east,
            Direction::North => self.north,
            Direction::West => self.west,
            Direction::South => self.south,
        }
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        !(self.east || self.north || self.west || self.south)
    }

    pub fn choose<R: rand::Rng + ?Sized>(self, rng: &mut R) -> Option<Direction> {
        let mut dirs = Vec::with_capacity(4);

        if self.east {
            dirs.push(Direction::East);
        }

        if self.north {
            dirs.push(Direction::North);
        }

        if self.west {
            dirs.push(Direction::West);
        }

        if self.south {
            dirs.push(Direction::South);
        }

        dirs.choose(rng).copied()
    }
}

impl Direction {
    #[inline]
    pub fn offset(self, p: UVec2) -> UVec2 {
        match self {
            Direction::East => p + UVec2::X,
            Direction::North => p + UVec2::Y,
            Direction::West => p - UVec2::X,
            Direction::South => p - UVec2::Y,
        }
    }

    #[inline]
    pub fn checked_offset(self, p: UVec2) -> Option<UVec2> {
        match self {
            Direction::East => Some(p + UVec2::X),
            Direction::North => Some(p + UVec2::Y),
            Direction::West => p.checked_sub(UVec2::X),
            Direction::South => p.checked_sub(UVec2::Y),
        }
    }
}

impl From<Direction> for Directions {
    fn from(value: Direction) -> Self {
        match value {
            Direction::East => Directions::EAST,
            Direction::North => Directions::NORTH,
            Direction::West => Directions::WEST,
            Direction::South => Directions::SOUTH,
        }
    }
}

impl rand::distr::Distribution<Directions> for rand::distr::StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Directions {
        Directions::new(
            rng.random_bool(0.5),
            rng.random_bool(0.5),
            rng.random_bool(0.5),
            rng.random_bool(0.5),
        )
    }
}

impl rand::distr::Distribution<Direction> for rand::distr::StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        if rng.random_bool(0.5) {
            if rng.random_bool(0.5) {
                Direction::East
            } else {
                Direction::West
            }
        } else if rng.random_bool(0.5) {
            Direction::North
        } else {
            Direction::South
        }
    }
}

impl Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Self::Output {
        match self {
            Direction::East => Direction::West,
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
        }
    }
}

impl Not for Directions {
    type Output = Directions;

    fn not(self) -> Self::Output {
        Directions {
            east: !self.east,
            north: !self.north,
            west: !self.west,
            south: !self.south,
        }
    }
}

impl BitOr<Directions> for Directions {
    type Output = Directions;

    fn bitor(self, rhs: Directions) -> Self::Output {
        Directions {
            east: self.east | rhs.east,
            north: self.north | rhs.north,
            west: self.west | rhs.west,
            south: self.south | rhs.south,
        }
    }
}

impl BitAnd<Directions> for Directions {
    type Output = Directions;

    fn bitand(self, rhs: Directions) -> Self::Output {
        Directions {
            east: self.east & rhs.east,
            north: self.north & rhs.north,
            west: self.west & rhs.west,
            south: self.south & rhs.south,
        }
    }
}

impl BitXor<Directions> for Directions {
    type Output = Directions;

    fn bitxor(self, rhs: Directions) -> Self::Output {
        Directions {
            east: self.east ^ rhs.east,
            north: self.north ^ rhs.north,
            west: self.west ^ rhs.west,
            south: self.south ^ rhs.south,
        }
    }
}

impl BitOrAssign<Directions> for Directions {
    fn bitor_assign(&mut self, rhs: Directions) {
        self.east |= rhs.east;
        self.north |= rhs.north;
        self.west |= rhs.west;
        self.south |= rhs.south;
    }
}

impl BitAndAssign<Directions> for Directions {
    fn bitand_assign(&mut self, rhs: Directions) {
        self.east &= rhs.east;
        self.north &= rhs.north;
        self.west &= rhs.west;
        self.south &= rhs.south;
    }
}

impl BitXorAssign<Directions> for Directions {
    fn bitxor_assign(&mut self, rhs: Directions) {
        self.east ^= rhs.east;
        self.north ^= rhs.north;
        self.west ^= rhs.west;
        self.south ^= rhs.south;
    }
}
