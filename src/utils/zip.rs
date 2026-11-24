//! Reads the files out of a zip archive (tldr-pages ships its pages as
//! one zip per language). Only what those use: files stored as they are or
//! deflated, no encryption, no zip64. Anything else is skipped, never an
//! error: a broken archive gives fewer files, or none.

/// Every file in `archive` up to `max_each` bytes unpacked, as (path,
/// bytes), in the archive's order. Folders and bigger files are left out.
pub fn files(archive: &[u8], max_each: usize) -> Vec<(String, Vec<u8>)> {
    let Some(end) = find_end(archive) else {
        return Vec::new();
    };
    let count = usize::from(u16_at(archive, end + 10).unwrap_or(0));
    let mut at = u32_at(archive, end + 16).map_or(usize::MAX, |o| o as usize);
    let mut found = Vec::new();
    for _ in 0..count {
        let Some(entry) = entry_at(archive, at) else {
            break;
        };
        at = entry.next;
        if entry.name.ends_with('/') || entry.size > max_each {
            continue;
        }
        if let Some(bytes) = unpack(archive, &entry, max_each) {
            found.push((entry.name, bytes));
        }
    }
    found
}

/// One file in the archive's table.
struct Entry {
    name: String,
    method: u16,
    packed: usize,
    size: usize,
    /// Where its local header is.
    local: usize,
    /// Where the next table entry starts.
    next: usize,
}

/// The end record (22 bytes, signature `PK\x05\x06`), where the table of
/// files is: at the very end, unless a comment (at most 64 KB) follows it.
fn find_end(archive: &[u8]) -> Option<usize> {
    let last = archive.len().checked_sub(22)?;
    let first = last.saturating_sub(0xffff);
    (first..=last)
        .rev()
        .find(|&at| u32_at(archive, at) == Some(0x0605_4b50))
}

/// The table entry at `at` (signature `PK\x01\x02`).
fn entry_at(archive: &[u8], at: usize) -> Option<Entry> {
    if u32_at(archive, at)? != 0x0201_4b50 {
        return None;
    }
    let name_len = usize::from(u16_at(archive, at + 28)?);
    let extra_len = usize::from(u16_at(archive, at + 30)?);
    let comment_len = usize::from(u16_at(archive, at + 32)?);
    let name = archive.get(at + 46..at + 46 + name_len)?;
    Some(Entry {
        name: String::from_utf8_lossy(name).into_owned(),
        method: u16_at(archive, at + 10)?,
        packed: u32_at(archive, at + 20)? as usize,
        size: u32_at(archive, at + 24)? as usize,
        local: u32_at(archive, at + 42)? as usize,
        next: at + 46 + name_len + extra_len + comment_len,
    })
}

/// A file's bytes, unpacked; None if they cannot be.
fn unpack(archive: &[u8], entry: &Entry, max: usize) -> Option<Vec<u8>> {
    let at = entry.local;
    if u32_at(archive, at)? != 0x0403_4b50 {
        // the local header's signature, `PK\x03\x04`
        return None;
    }
    // the local header repeats the name, with an extra field of its own size
    let start =
        at + 30 + usize::from(u16_at(archive, at + 26)?) + usize::from(u16_at(archive, at + 28)?);
    let packed = archive.get(start..start.checked_add(entry.packed)?)?;
    match entry.method {
        0 => Some(packed.to_vec()), // stored as it is
        8 => miniz_oxide::inflate::decompress_to_vec_with_limit(packed, max).ok(),
        _ => None,
    }
}

fn u16_at(bytes: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(
        bytes.get(at..at.checked_add(2)?)?.try_into().ok()?,
    ))
}

fn u32_at(bytes: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(
        bytes.get(at..at.checked_add(4)?)?.try_into().ok()?,
    ))
}
