use std::error::Error;
use std::fmt::Display;

pub trait GuiCoreError: Error { }

#[derive(Debug)]
pub enum GeneralError {

    StructInit,


}

impl GuiCoreError for GeneralError { }
impl Error for GeneralError { }

impl Display for GeneralError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {

        match self {
            GeneralError::StructInit => write!(f, "[GeneralError]: Struct could not be initialized: {}", self.description())
        }
        

    }
}