//module rules;
#![allow(dead_code)]

#[derive(Debug)]
pub enum CellError {
    None,
    OutOfBounds,
    UndefinedBehavior,
    CouldNotComplete,
    FailedToSave,
    FailedToUnload,
    TargtNotLoaded
}

pub enum ChunkError {
    None,
    TargtNotLoaded,
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