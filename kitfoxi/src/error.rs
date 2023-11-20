#[derive(Debug)]
pub enum ErrorCodes {
    General,
    IoctlError,
    IoctlErrorInvalidPath,
    IoctlErrorInvalidUtf8,
    IoctlErrorUnsupportedOperation,
    ParsingError,
}

#[derive(Debug)]
pub struct Error {
    pub code: ErrorCodes,
    pub subcode: i32,
    pub message: String,
    pub file: String,
    pub line: u32,
    pub column: u32,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} [Code: {:?}, Subcode: {:x} ({}: line {}, column {})]",
            self.message, self.code, self.subcode, self.file, self.line, self.column
        )
    }
}

impl std::error::Error for Error {}
