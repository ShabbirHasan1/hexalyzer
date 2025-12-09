use intelhex::IntelHex;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) enum FileKind {
    Hex,
    Bin,
    Elf,
    Unknown,
}

fn detect_file_kind(path: &PathBuf) -> std::io::Result<FileKind> {
    let mut f = File::open(path)?;

    // Read the first 32 bytes (OK if file has less)
    let mut buf = [0u8; 32];
    let n = f.read(&mut buf)?;
    let buf = &buf[..n];

    // If read 0 bytes -> Unknown
    if n == 0 {
        return Ok(FileKind::Unknown);
    }

    // ELF magic check
    if buf.len() >= 4 && &buf[..4] == b"\x7FELF" {
        return Ok(FileKind::Elf);
    }

    // Intel HEX record start check
    if buf[0] == b':' {
        return Ok(FileKind::Hex);
    }

    // Otherwise consider the file as raw binary
    Ok(FileKind::Bin)
}

pub(crate) fn load_file(path: &PathBuf, ih: &mut IntelHex) -> Result<(), Box<dyn Error>> {
    let file_type = match detect_file_kind(path) {
        Ok(kind) => kind,
        Err(err) => {
            return Err(Box::new(err));
        }
    };

    match file_type {
        FileKind::Hex => ih.load_hex(path),
        FileKind::Bin => {
            // Set base addr to 0 to avoid complex logic around waiting
            // to fill the pop-up. Can re-addr later.
            ih.load_bin(path, 0)
        }
        FileKind::Elf => Err("ELF files are not yet supported".into()),
        FileKind::Unknown => Err("Could not determine the file type".into()),
    }
}
