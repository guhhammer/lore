//! The web links of a page: its own, then the searches of `SEARCH_LINKS`.

use crate::configs::SEARCH_LINKS;

/// The searches for `name`, as (label, URL).
pub fn searches(name: &str) -> Vec<(String, String)> {
    let query = encode(name);
    SEARCH_LINKS
        .iter()
        .map(|(label, url)| ((*label).to_owned(), url.replace("{}", &query)))
        .collect()
}

/// `text` made safe for a URL's query: letters, digits and `-_.~` stay, every
/// other byte becomes `%XX` (so `[` searches for `[`, and `&` cannot add a
/// parameter of its own).
pub fn encode(text: &str) -> String {
    let mut encoded = String::new();
    for byte in text.bytes() {
        if byte.is_ascii_alphanumeric() || b"-_.~".contains(&byte) {
            encoded.push(char::from(byte));
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }
    encoded
}
