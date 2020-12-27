use std::fmt;
use subprocess::PopenError;

#[derive(Debug)]
pub enum AppError {
    PartOfSpeechError(&'static str),
    GrammemError(&'static str),
    PopenError(PopenError),
    MystemError(&'static str),
}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred.")
    }
}
impl From<PopenError> for AppError {
    fn from(e: PopenError) -> AppError {
        return AppError::PopenError(e);
    }
}
