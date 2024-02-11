use crate::config::{CHUNK_WIDTH_I32, CHUNK_SIZE_I32};
use std::{fmt, ops::{Add, AddAssign}};
use serde::{Serialize, Deserialize};

/// # Functionality:
/// contains and sparates the global coordinates from the other types
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GlobalCoords {
    pub x: i32,
    pub y: i32
}

impl From<(f32, f32)> for GlobalCoords {
    fn from(value: (f32, f32)) -> Self {
        GlobalCoords { 
            x: value.0 as i32,
            y: value.1 as i32
        }
    }
}

impl From<&(f32, f32)> for GlobalCoords {
    fn from(value: &(f32, f32)) -> Self {
        GlobalCoords { 
            x: value.0 as i32,
            y: value.1 as i32
        }
    }
}

impl From<(i32, i32)> for GlobalCoords {
    fn from(value: (i32, i32)) -> Self {
        GlobalCoords { 
            x: value.0,
            y: value.1
        }
    }
}

impl From<&(i32, i32)> for GlobalCoords {
    fn from(value: &(i32, i32)) -> Self {
        GlobalCoords { 
            x: value.0,
            y: value.1
        }
    }
}

impl From<GlobalFloatingCoordinates> for GlobalCoords {
    fn from(value: GlobalFloatingCoordinates) -> Self {
        GlobalCoords { 
            x: value.x.trunc() as i32, 
            y: value.y.trunc() as i32 
        }
    }
}

impl From<&GlobalFloatingCoordinates> for GlobalCoords {
    fn from(value: &GlobalFloatingCoordinates) -> Self {
        GlobalCoords { 
            x: value.x.trunc() as i32, 
            y: value.y.trunc() as i32 
        }
    }
}

impl Add<(i32, i32)> for GlobalCoords {
    type Output = GlobalCoords;
    fn add(self, rhs: (i32, i32)) -> Self::Output {
        GlobalCoords { x: self.x + rhs.0, y: self.y + rhs.1 }
    }
}

/// # Functionality:
/// contains and sparates the chunk coordinates from the other types
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub struct ChunkCoords {
    pub x: i32,
    pub y: i32
}

impl From<(i32, i32)> for ChunkCoords {
    fn from(coords: (i32, i32)) -> Self {
        ChunkCoords { 
            x: if coords.0 >= 0 { coords.0 / CHUNK_WIDTH_I32 + 1 } else { coords.0 / CHUNK_WIDTH_I32 },
            y: if coords.1 >= 0 { coords.1 / CHUNK_WIDTH_I32 + 1 } else { coords.1 / CHUNK_SIZE_I32 }
        }
    }
}

impl From<&(i32, i32)> for ChunkCoords {
    fn from(coords: &(i32, i32)) -> Self {
        ChunkCoords { 
            x: if coords.0 >= 0 { coords.0 / CHUNK_WIDTH_I32 + 1 } else { coords.0 / CHUNK_WIDTH_I32 },
            y: if coords.1 >= 0 { coords.1 / CHUNK_WIDTH_I32 + 1 } else { coords.1 / CHUNK_WIDTH_I32 }
        }
    }
}

impl From<GlobalCoords> for ChunkCoords {
    fn from(coords: GlobalCoords) -> Self {
        ChunkCoords { 
            x: if coords.x >= 0 { coords.x / CHUNK_WIDTH_I32 + 1 } else { coords.x / CHUNK_WIDTH_I32 },
            y: if coords.y >= 0 { coords.x / CHUNK_WIDTH_I32 + 1 } else { coords.y / CHUNK_WIDTH_I32 }
        }
    }
}

impl From<&GlobalCoords> for ChunkCoords {
    fn from(coords: &GlobalCoords) -> Self {
        ChunkCoords { 
            x: if coords.x >= 0 { coords.x / CHUNK_WIDTH_I32 + 1 } else { coords.x / CHUNK_WIDTH_I32 },
            y: if coords.y >= 0 { coords.x / CHUNK_WIDTH_I32 + 1 } else { coords.y / CHUNK_WIDTH_I32 }
        }
    }
}

impl Add<(i32, i32)> for ChunkCoords {
    type Output = ChunkCoords;
    fn add(self, rhs: (i32, i32)) -> Self::Output {
        ChunkCoords {
            x: self.x + rhs.0,
            y: self.y + rhs.1
        }
    }
}

/// # Functionality:
/// contains and sparates the local coordinates from the other types
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LocalCoords {
    pub x: i32,
    pub y: i32
}

impl From<GlobalCoords> for LocalCoords {
    fn from(coords: GlobalCoords) -> Self {
        LocalCoords { 
            x: coords.x.rem_euclid(CHUNK_WIDTH_I32),
            y: coords.y.rem_euclid(CHUNK_WIDTH_I32)
        }
    }
}

/// # Functionality:
/// contains and sparates the index from the other types
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Index {
    pub i: usize
}

impl From<GlobalCoords> for Index {
    fn from(coords: GlobalCoords) -> Self {
        Index { i: (coords.x.rem_euclid(CHUNK_WIDTH_I32) + coords.y.rem_euclid(CHUNK_WIDTH_I32) * CHUNK_WIDTH_I32) as usize }
    }
}

impl From<&GlobalCoords> for Index {
    fn from(coords: &GlobalCoords) -> Self {
        Index { i: (coords.x.rem_euclid(CHUNK_WIDTH_I32) + coords.y.rem_euclid(CHUNK_WIDTH_I32) * CHUNK_WIDTH_I32) as usize }
    }
}

impl From<LocalCoords> for Index {
    fn from(coords: LocalCoords) -> Self {
        Index { i: (coords.x + coords.y * CHUNK_WIDTH_I32) as usize }
    }
}

impl From<&LocalCoords> for Index {
    fn from(coords: &LocalCoords) -> Self {
        Index { i: (coords.x + coords.y * CHUNK_WIDTH_I32) as usize }
    }
}

/// # Functionality:
/// creates a floating point coordinate system, due to it being useful when it comes to shaders and the player camera/position
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GlobalFloatingCoordinates {
    pub x: f32,
    pub y: f32,
}

impl From<(f32, f32)> for GlobalFloatingCoordinates {
    fn from(value: (f32, f32)) -> Self {
        GlobalFloatingCoordinates { x: value.0, y: value.1 }
    }
}

impl From<GlobalCoords> for GlobalFloatingCoordinates {
    fn from(value: GlobalCoords) -> Self {
        GlobalFloatingCoordinates { x: value.x as f32, y: value.y as f32 }
    }
}

impl Add<(i32, i32)> for GlobalFloatingCoordinates {
    type Output = GlobalFloatingCoordinates;
    fn add(self, rhs: (i32, i32)) -> Self::Output {
        GlobalFloatingCoordinates {
            x: self.x + rhs.0 as f32,
            y: self.y + rhs.1 as f32,
        }
    }
}

impl Add<(f32, f32)> for GlobalFloatingCoordinates {
    type Output = GlobalFloatingCoordinates;
    fn add(self, rhs: (f32, f32)) -> Self::Output {
        GlobalFloatingCoordinates {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl AddAssign<(f32, f32)> for GlobalFloatingCoordinates {
    fn add_assign(&mut self, rhs: (f32, f32)) {
        *self = Self  {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl fmt::Display for GlobalFloatingCoordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {} y: {}", self.x, self.y)
    }
}