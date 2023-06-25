pub enum FileFormats {
    PlainText,
    Rle,
    Unknown,
}

/// Deduce the type of the file by its extension, ".cells" and .rle" for the plaintext and run length encoded

pub fn deduce(path : String) -> FileFormats {
    let mut format = FileFormats::Unknown;

    for extension in path.split('.').rev() {
        format = match extension {
            "cells" => FileFormats::PlainText,
            "rle" => FileFormats::Rle,
            _ => FileFormats::Unknown
        };

        break;
    }

    format
}



