use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ErrorCodes {
    NotImplemented,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub code: ErrorCodes,
    pub subcode: u64,
    pub file: String,
    pub line: u32,
    pub column: u32,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Code: {:?}, Subcode: {:x} ({}: line {}, column {})",
            self.code, self.subcode, self.file, self.line, self.column
        )
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn not_implemented() -> Error {
        Error {
            code: ErrorCodes::NotImplemented,
            subcode: 0,
            file: String::from(file!()),
            line: line!(),
            column: column!(),
        }
    }
}
