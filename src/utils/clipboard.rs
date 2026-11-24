//! Copying to the clipboard through the terminal: the OSC 52 escape
//! (`ESC ] 52 ; c ; <base64> BEL`) asks the terminal itself to put the text
//! on the clipboard, so it works over ssh and inside tmux (with
//! `set-clipboard on`), with no clipboard tool installed. Terminals that do
//! not know it ignore it.

use std::io::{self, Write};

/// The escape that copies `text`.
pub fn osc52(text: &str) -> String {
    format!("\x1b]52;c;{}\x07", base64(text.as_bytes()))
}

/// Sends the copy escape to the terminal.
pub fn copy(text: &str) -> io::Result<()> {
    let mut out = io::stdout().lock();
    out.write_all(osc52(text).as_bytes())?;
    out.flush()
}

/// Standard base64 with `=` padding, which OSC 52 wants; short texts only,
/// so no crate for it.
pub fn base64(bytes: &[u8]) -> String {
    let alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b = [
            chunk[0],
            *chunk.get(1).unwrap_or(&0),
            *chunk.get(2).unwrap_or(&0),
        ];
        let n = (u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2]);
        for i in 0..4 {
            if i <= chunk.len() {
                out.push(char::from(alphabet[((n >> (18 - 6 * i)) & 63) as usize]));
            } else {
                out.push('=');
            }
        }
    }
    out
}
