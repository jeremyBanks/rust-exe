#![allow(unused)]

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
static RUST_EDITION_RELEASES: &[(u64, u64)] = &[(0, 2015), (31, 2018), (56, 2021)];

/// Returns the minor version number of the most recent Rust release at a given
/// timestamp. For example, this would return `59` if given a timestamp shortly
/// after the Rust 1.59.0 release. Rounds up to version 1.0.0 if given a timestamp
/// from before the first Rust release. Pre-1.0 releases aren't supported by
/// rustup, and we don't attempt to support them either.
pub fn last_release_at(timestamp: u64) -> u64 {
    if timestamp < RUST_EPOCH_SECONDS {
        return 0;
    }
    return ((timestamp - RUST_EPOCH_SECONDS) / RUST_RELEASE_INTERVAL_SECONDS) as u64;
}

#[test]
fn test_last_release_at() {
    assert_eq!(last_release_at(0), 0);
    assert_eq!(last_release_at(RUST_EPOCH_SECONDS), 0);
    assert_eq!(
        last_release_at(RUST_EPOCH_SECONDS + RUST_RELEASE_INTERVAL_SECONDS),
        1
    );
    assert_eq!(last_release_at(1646136000), 59);
    assert_eq!(last_release_at(1654979723), 61);
}
