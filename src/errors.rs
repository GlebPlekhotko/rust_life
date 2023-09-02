#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorCode {
    HeaderNotExpected = -1,
    UnrecognizedCharacter = -2,
    WrongRleHeader = -3,
    UnrecognizedFileFormat = -4,
    FailedToOpenFile = -5,
    FailedToReadFile = -6,
    FieldTooSmall = -7,
    RleVolationXSize = -8,
    RleVolationYSize = -9,
    FailedToCreateFile = -10,
    FailedToWriteFile = -11,
    NewlineFailed = -12,
    NoFileExtension = -13,
    EmptyPath = -14,
    FailedToCreateDirectory = -15,
    CellsNotExpected = -16
}
