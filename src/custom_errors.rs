#[derive(Debug)]
pub enum CellError {
    None,
    OutOfBounds,
    UndefinedBehavior,
    CouldNotComplete,
    FailedToSave,
    FailedToUnload
}