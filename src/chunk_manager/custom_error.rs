#[derive(Debug)]
pub enum CustomErrors {
    None,
    OutOfBounds,
    UndefinedBehavior,
    CouldNotComplete,
    FailedToSave,
    FailedToUnload
}