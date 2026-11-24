//! The internet, for the INTERNET section of a page: whether this machine
//! can reach it (the dot on every screen), and a command's community cheat
//! sheet from cheat.sh, fetched in the background when its page opens.
//!
//! What comes back is anyone's text, unchecked: it is kept as raw lines,
//! with control and invisible characters taken out so the terminal cannot
//! obey it, never parsed for links, never offered to copy, never run.
//! `internet = off` in the settings file (or Internet off on the about
//! screen): no request is ever made.

use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::net::{TcpStream, ToSocketAddrs};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

use crate::configs::{
    CACHE_HOME_FALLBACK, CACHE_HOME_VAR, INTERNET_CACHE_DIR, INTERNET_CACHE_HOURS,
    INTERNET_CHECK_EVERY, INTERNET_HOST, INTERNET_MAX_BYTES, INTERNET_NOT_FOUND, INTERNET_TIMEOUT,
    INTERNET_URL, INTERNET_USER_AGENT, WAKE_EVERY,
};
use crate::info::content::{page_key, shown};
use crate::interface::paths;

/// Whether the internet can be reached.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// Not known yet: the first check is under way.
    Checking,
    Online,
    Offline,
    /// `internet = off`: lore makes no request at all.
    Off,
}

/// What there is for one command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Answer {
    Loading,
    /// The cheat sheet's lines, cleaned.
    Lines(Vec<String>),
    /// The site has nothing for this command.
    NotFound,
    /// The request failed (offline, slow, refused).
    Failed,
}

/// Shared between the screen and the background threads.
#[derive(Debug)]
pub struct Internet {
    /// Off: no thread checks, no request is made (`internet = off`).
    enabled: AtomicBool,
    /// The checking thread is alive; at most one at a time.
    running: AtomicBool,
    status: AtomicU8,
    answers: Mutex<HashMap<String, Answer>>,
    /// Something changed since the screen last looked: draw again.
    changed: AtomicBool,
}

impl Internet {
    fn new(enabled: bool) -> Self {
        Self {
            enabled: AtomicBool::new(enabled),
            running: AtomicBool::new(false),
            status: AtomicU8::new(if enabled {
                Status::Checking
            } else {
                Status::Off
            } as u8),
            answers: Mutex::new(HashMap::new()),
            changed: AtomicBool::new(false),
        }
    }

    /// No internet: no thread, no request (tests, `internet = off`). It can
    /// still be turned on later, from the about screen.
    pub fn off() -> Arc<Self> {
        Arc::new(Self::new(false))
    }

    /// Starts checking the connection in the background, now and every
    /// `INTERNET_CHECK_EVERY`.
    pub fn start() -> Arc<Self> {
        let internet = Arc::new(Self::new(true));
        internet.spawn_checker();
        internet
    }

    /// Whether lore may use the internet (the setting, not the connection).
    pub fn enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    /// Turns the internet on or off while lore runs (the about screen).
    /// Off: the dot goes red and no request is made from now on.
    pub fn set_enabled(self: &Arc<Self>, on: bool) {
        self.enabled.store(on, Ordering::Relaxed);
        let status = if on { Status::Checking } else { Status::Off };
        self.status.store(status as u8, Ordering::Relaxed);
        if on {
            // what failed or never started while off is asked again
            if let Ok(mut answers) = self.answers.lock() {
                answers.retain(|_, answer| matches!(answer, Answer::Lines(_) | Answer::NotFound));
            }
            self.spawn_checker();
        }
        self.changed.store(true, Ordering::Relaxed);
    }

    /// The thread that checks the connection, unless one is alive; it ends
    /// once the internet is turned off.
    fn spawn_checker(self: &Arc<Self>) {
        if self.running.swap(true, Ordering::Relaxed) {
            return;
        }
        let shared = Arc::clone(self);
        thread::spawn(move || {
            while shared.enabled() {
                let status = if reachable() {
                    Status::Online
                } else {
                    Status::Offline
                };
                // turned off while checking: Off stays
                if shared.enabled()
                    && shared.status.swap(status as u8, Ordering::Relaxed) != status as u8
                {
                    shared.changed.store(true, Ordering::Relaxed);
                }
                // asleep in short steps, so turning it off ends the thread soon
                let mut slept = Duration::ZERO;
                while slept < INTERNET_CHECK_EVERY && shared.enabled() {
                    thread::sleep(WAKE_EVERY);
                    slept += WAKE_EVERY;
                }
            }
            shared.running.store(false, Ordering::Relaxed);
            // turned on again just as this one ended: a new one checks
            if shared.enabled() {
                shared.spawn_checker();
            }
        });
    }

    pub fn status(&self) -> Status {
        match self.status.load(Ordering::Relaxed) {
            s if s == Status::Online as u8 => Status::Online,
            s if s == Status::Offline as u8 => Status::Offline,
            s if s == Status::Checking as u8 => Status::Checking,
            _ => Status::Off,
        }
    }

    /// True once after anything changed: the loop draws again.
    pub fn take_changed(&self) -> bool {
        self.changed.swap(false, Ordering::Relaxed)
    }

    /// What there is for `name` so far; None when nothing was asked.
    pub fn answer(&self, name: &str) -> Option<Answer> {
        self.answers.lock().ok()?.get(name).cloned()
    }

    /// Fetches `name`'s cheat sheet in the background, once a run (or from
    /// the cache, when it is less than a day old).
    pub fn request(self: &Arc<Self>, name: &str) {
        if !self.enabled() {
            return;
        }
        let Ok(mut answers) = self.answers.lock() else {
            return;
        };
        if answers.contains_key(name) {
            return;
        }
        answers.insert(name.to_owned(), Answer::Loading);
        drop(answers);
        let shared = Arc::clone(self);
        let name = name.to_owned();
        thread::spawn(move || {
            let answer = cached(&name).unwrap_or_else(|| fetch(&name));
            if let Ok(mut answers) = shared.answers.lock() {
                answers.insert(name, answer);
            }
            shared.changed.store(true, Ordering::Relaxed);
        });
    }
}

/// Whether the site answers on its https port, within the timeout.
fn reachable() -> bool {
    let Ok(addresses) = (INTERNET_HOST, 443).to_socket_addrs() else {
        return false; // no DNS: no internet, as far as lore can tell
    };
    addresses
        .into_iter()
        .any(|address| TcpStream::connect_timeout(&address, INTERNET_TIMEOUT).is_ok())
}

/// The cheat sheet's address: the page's name (`git-commit`), with every
/// character but letters, digits, `.`, `_` and `-` percent-encoded.
pub fn url(name: &str) -> String {
    let key: String = page_key(name)
        .bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'.' | b'_' | b'-' => {
                char::from(b).to_string()
            }
            _ => format!("%{b:02X}"),
        })
        .collect();
    INTERNET_URL.replace("{name}", &key)
}

/// `url`'s body, at most `max` bytes of it (the rest is cut), within
/// `timeout`; the error says why not. Only https addresses are asked.
pub fn download(url: &str, max: usize, timeout: Duration) -> Result<Vec<u8>, String> {
    if !url.starts_with("https://") {
        return Err(format!("not https: {url}"));
    }
    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(timeout))
        .user_agent(INTERNET_USER_AGENT.replace("{version}", env!("CARGO_PKG_VERSION")))
        .build()
        .new_agent();
    let response = agent.get(url).call().map_err(|e| e.to_string())?;
    let mut bytes = Vec::new();
    response
        .into_body()
        .into_reader()
        .take(max as u64)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    Ok(bytes)
}

/// Asks the site, at most `INTERNET_MAX_BYTES` of answer, within the
/// timeout; keeps a good answer in the cache.
fn fetch(name: &str) -> Answer {
    let Ok(bytes) = download(&url(name), INTERNET_MAX_BYTES, INTERNET_TIMEOUT * 4) else {
        return Answer::Failed;
    };
    let text = String::from_utf8_lossy(&bytes);
    let answer = answer_of(&text);
    if let (Answer::Lines(_), Some(dir)) = (&answer, cache_dir()) {
        let _ = fs::create_dir_all(&dir);
        let _ = fs::write(dir.join(format!("{}.txt", page_key(name))), text.as_bytes());
    }
    answer
}

/// A cached answer under a day old.
fn cached(name: &str) -> Option<Answer> {
    let path = cache_dir()?.join(format!("{}.txt", page_key(name)));
    if age(&path)? > Duration::from_secs(INTERNET_CACHE_HOURS * 3600) {
        return None;
    }
    let bytes = fs::read(&path).ok()?;
    Some(answer_of(&String::from_utf8_lossy(&bytes)))
}

/// `$XDG_CACHE_HOME/lore/internet`, or `~/.cache/lore/internet`.
fn cache_dir() -> Option<PathBuf> {
    Some(cache_home()?.join(INTERNET_CACHE_DIR))
}

/// `$XDG_CACHE_HOME`, or `~/.cache`.
pub fn cache_home() -> Option<PathBuf> {
    std::env::var_os(CACHE_HOME_VAR)
        .map(PathBuf::from)
        .filter(|dir| dir.is_absolute())
        .or_else(|| paths::home().map(|home| home.join(CACHE_HOME_FALLBACK)))
}

/// How long ago `path` was written; None if it is not there.
pub fn age(path: &std::path::Path) -> Option<Duration> {
    SystemTime::now()
        .duration_since(fs::metadata(path).ok()?.modified().ok()?)
        .ok()
}

/// The site's text as lines to show: every character a terminal could obey
/// or that hides text taken out, an escape sequence's leftovers too.
pub fn answer_of(text: &str) -> Answer {
    let start = text.trim_start();
    if start.starts_with(INTERNET_NOT_FOUND) || start.is_empty() {
        return Answer::NotFound;
    }
    // a web page instead of text: not something to show as a cheat sheet
    if start.starts_with('<') {
        return Answer::Failed;
    }
    let lines = text.lines().map(clean).collect();
    Answer::Lines(lines)
}

/// One line, safe to draw: escape sequences (`ESC [ … m`) dropped whole,
/// other control and invisible characters dropped, tabs as spaces.
pub fn clean(line: &str) -> String {
    let mut out = String::new();
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\x1b' => {
                // a CSI sequence: ESC [ parameters… final letter
                if chars.peek() == Some(&'[') {
                    chars.next();
                    for next in chars.by_ref() {
                        if next.is_ascii_alphabetic() {
                            break;
                        }
                    }
                }
            }
            '\t' => out.push_str("    "),
            c if shown(c) => out.push(c),
            _ => {}
        }
    }
    out.trim_end().to_owned()
}
