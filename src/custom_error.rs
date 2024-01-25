//module rules;
#![allow(dead_code)]

#[derive(Debug)]
pub enum CellError {
    None,
    OutOfBounds,
    UndefinedBehavior,
    CouldNotComplete,
    FailedToSwap,
    TargtNotLoaded
}

pub enum ChunkError {
    None,
    TargtNotLoaded,
    TargtAlreadyLoaded,
    OutOfBounds,
    FailedToUnload,
    CouldNotComplete
}

impl From<std::io::Error> for CellError {
    fn from(_: std::io::Error) -> Self {
        CellError::CouldNotComplete
    }
}

impl From<CellError> for ChunkError {
    fn from(value: CellError) -> Self {
        match value {
            _ => ChunkError::None,
        }
    }
}

impl From<ChunkError> for CellError {
    fn from(value: ChunkError) -> Self {
        match value {
            _ => CellError::CouldNotComplete,
        }
    }
}