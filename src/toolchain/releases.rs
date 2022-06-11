#![allow(unused)]
static RELEASE_LIST_URL: &str = "https://static.rust-lang.org/manifests.txt";
static RELEASE_LIST_FALLBACK: &str = include_str!("./manifests.txt");

/// The unix timestamp of the first crates.io index commit, rounded to noon UTC:
/// 2014-11-04T12:00:00Z.
static CRATES_EPOCH_SECONDS: u64 = 1_415_102_400;

/// The unix timestamp of the Rust 1.0.0 release, rounded to noon UTC:
/// 2015-05-15T12:00:00Z.
static RUST_EPOCH_SECONDS: u64 = 1_431_691_200;

/// The number of seconds between each Rust subsequent release.
/// This is equal to 42 days, or 6 weeks.
static RUST_RELEASE_INTERVAL_SECONDS: u64 = 3_628_800;

/// Ordered list of pairs of rust versions that introduced a new edition.
static RUST_EDITION_RELEASES: &[(u64, u64)] = &[
    (0, 2015),
    (31, 2018),
    (56, 2021),
];

/// Returns the minor version number of the most recent Rust release at a given
/// timestamp. For example, this would return `59` if given a timestamp shortly
/// after the Rust 1.59.0 release. Rounds up to version 1.0.0 if given a timestamp
/// from before the first Rust release. Pre-1.0 releases aren't supported by
/// rustup, and we don't attempt to support them either.
pub fn last_release_at(timestamp: u64) -> u64 {
    if timestamp < RUST_EPOCH_SECONDS {
        return 0;
    }
    return ((timestamp - RUST_EPOCH_SECONDS) / RUST_RELEASE_INTERVAL_SECONDS) as u64
}

#[test]
fn test_last_release_at() {
    assert_eq!(last_release_at(0), 0);
    assert_eq!(last_release_at(RUST_EPOCH_SECONDS), 0);
    assert_eq!(last_release_at(RUST_EPOCH_SECONDS + RUST_RELEASE_INTERVAL_SECONDS), 1);
    assert_eq!(last_release_at(1646136000), 59);
    assert_eq!(last_release_at(1654979723), 61);
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
