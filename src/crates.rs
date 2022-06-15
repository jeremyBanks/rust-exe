#![allow(unused)]
use crate::*;

/// The unix timestamp of the first crates.io index commit, rounded to noon UTC:
/// 2014-11-04T12:00:00Z.
static CRATES_EPOCH_SECONDS: u64 = 1_415_102_400;

/*

to process a file
    identify file (module) level metadata in the doc strings.

    identify every possible dependency, marking them as "confident" (if
        they're from an `extern crate` or a fully-qualified path) or
        "uncertain".
        for each of these dependency candidate tokens, look for version
        metadata on the enclosing item's doc string.

    identify every external reference it has by `mod` statement

    for each external reference, add it with the list of possible
    filenames (usually 1 if there's a #[path], else 2) to the queue
    for processing.

maybe produce a git hash of the input tree, and the output tree.
*/
