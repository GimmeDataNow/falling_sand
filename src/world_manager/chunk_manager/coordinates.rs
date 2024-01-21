use crate::config::CHUNK_SIZE_I32;
use std::ops::Add;
use serde::{Serialize, Deserialize};

/// # Functionality:
/// contains and sparates the global coordinates from the other types
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GlobalCoords {
    pub x: i32,
    pub y: i32
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
            x: if coords.0 >= 0 { coords.0 / CHUNK_SIZE_I32 + 1 } else { coords.0 / CHUNK_SIZE_I32 },
            y: if coords.1 >= 0 { coords.1 / CHUNK_SIZE_I32 + 1 } else { coords.1 / CHUNK_SIZE_I32 }
        }
    }
}

impl From<&(i32, i32)> for ChunkCoords {
    fn from(coords: &(i32, i32)) -> Self {
        ChunkCoords { 
            x: if coords.0 >= 0 { coords.0 / CHUNK_SIZE_I32 + 1 } else { coords.0 / CHUNK_SIZE_I32 },
            y: if coords.1 >= 0 { coords.1 / CHUNK_SIZE_I32 + 1 } else { coords.1 / CHUNK_SIZE_I32 }
        }
    }
}

impl From<GlobalCoords> for ChunkCoords {
    fn from(coords: GlobalCoords) -> Self {
        ChunkCoords { 
            x: if coords.x >= 0 { coords.x / CHUNK_SIZE_I32 + 1 } else { coords.x / CHUNK_SIZE_I32 },
            y: if coords.y >= 0 { coords.x / CHUNK_SIZE_I32 + 1 } else { coords.y / CHUNK_SIZE_I32 }
        }
    }
}

impl From<&GlobalCoords> for ChunkCoords {
    fn from(coords: &GlobalCoords) -> Self {
        ChunkCoords { 
            x: if coords.x >= 0 { coords.x / CHUNK_SIZE_I32 + 1 } else { coords.x / CHUNK_SIZE_I32 },
            y: if coords.y >= 0 { coords.x / CHUNK_SIZE_I32 + 1 } else { coords.y / CHUNK_SIZE_I32 }
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
            x: coords.x.rem_euclid(CHUNK_SIZE_I32),
            y: coords.y.rem_euclid(CHUNK_SIZE_I32)
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
        Index { i: (coords.x.rem_euclid(CHUNK_SIZE_I32) + coords.y.rem_euclid(CHUNK_SIZE_I32) * CHUNK_SIZE_I32) as usize }
    }
}

impl From<&GlobalCoords> for Index {
    fn from(coords: &GlobalCoords) -> Self {
        Index { i: (coords.x.rem_euclid(CHUNK_SIZE_I32) + coords.y.rem_euclid(CHUNK_SIZE_I32) * CHUNK_SIZE_I32) as usize }
    }
}

impl From<LocalCoords> for Index {
    fn from(coords: LocalCoords) -> Self {
        Index { i: (coords.x + coords.y * CHUNK_SIZE_I32) as usize }
    }
}

impl From<&LocalCoords> for Index {
    fn from(coords: &LocalCoords) -> Self {
        Index { i: (coords.x + coords.y * CHUNK_SIZE_I32) as usize }
    }
}