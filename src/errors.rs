#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorCode {
    HeaderNotExpected,
    UnrecognizedCharacter,
    WrongRleHeader,
    UnrecognizedFileFormat,
    FailedToOpenFile,
    FailedToReadFile,
    FieldTooSmall
}
