#![allow(unused)]
static RELEASE_LIST_URL: &str = "https://static.rust-lang.org/manifests.txt";
static RELEASE_LIST_FALLBACK: &str = include_str!("./manifests.txt");

/// The unix timestamp of the Rust 1.0.0 release, rounded to noon UTC:
/// 2015-05-15T12:00:00Z.
const RUST_EPOCH_SECONDS: u64 = 1431691200;

/// The number of seconds between each Rust subsequent release:
/// 42 days (6 weeks).
const RUST_RELEASE_INTERVAL_SECONDS: u64 = 6 * 7 * 24 * 60 * 60;

#[test]
fn test() {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let seconds_since_epoch = now - RUST_EPOCH_SECONDS;
    let version = seconds_since_epoch / RUST_RELEASE_INTERVAL_SECONDS;
    panic!("You are living in the age of Rust 1.{version}. Wonders never cease!");
}

#[derive(Debug, Clone)]
pub struct ReleaseIndex {
    entries: Vec<Release>,
}

impl ReleaseIndex {
    pub fn offline() -> Self {
        let mut entries = Vec::new();

        for line in RELEASE_LIST_FALLBACK.lines() {
            let line = if let Some(line) = line.strip_prefix("static.rust-lang.org/dist/") {
                line
            } else {
                continue;
            };

            let line = if let Some(line) = line.strip_suffix(".toml") {
                line
            } else {
                panic!("non .toml file in manifest: {:?}", line)
            };

            let (date, name) = if let Some(split) = line.split_once('/') {
                split
            } else {
                panic!("invalid manifest line: {:?}", line)
            };

            let name = if let Some(name) = name.strip_prefix("channel-rust-") {
                name
            } else {
                panic!("invalid manifest line: {:?}", line)
            };

            let identifier = format!("{name}-{date}");

            println!("{identifier}");
        }

        ReleaseIndex { entries }
    }
}

#[test]
fn test_release_index() {
    let index = ReleaseIndex::offline();
    assert!(index.entries.len() > 0);
}

#[derive(Debug, Clone)]
pub struct Release {
    date: Date,
    channel: Channel,
}

#[derive(Debug, Clone)]
pub enum Channel {
    Stable(Version),
    Beta,
    Nightly,
}

#[derive(Debug, Clone)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

#[derive(Debug, Clone)]
pub struct Date {
    pub year: u8,
    pub month: u8,
    pub day: u8,
}
