#![doc = include_str!("../README.md")]
#![allow(unused_crate_dependencies)]

#[cfg(doc)]
pub mod reference {
    #![doc = include_str!("../reference.md")]
}

#[cfg(test)]
pub(crate) use expect_test::expect;

#[doc(hidden)]
#[allow(unused)]
pub(crate) use {
    crate::{arg_stream::*, cli::*, crates::*, git_hashing::*, run::*, toolchain::*, util::*},
    ::{
        cargo_lock::Lockfile,
        eyre::Result,
        heck::*,
        indexmap::{IndexMap, IndexSet},
        std::str::FromStr,
        std::{
            self,
            env::current_dir,
            ffi::{OsStr, OsString},
            os::unix::prelude::OsStrExt,
            path::PathBuf,
            time::{SystemTime, UNIX_EPOCH},
        },
        syn,
        toml_edit::easy::{toml, Value as Toml},
        tracing::{debug, error, info, trace, warn},
    },
};

#[doc(hidden)]
pub(crate) mod arg_stream;
#[doc(hidden)]
pub(crate) mod cli;
#[doc(hidden)]
pub(crate) mod crates;
#[doc(hidden)]
pub(crate) mod git_hashing;
#[doc(hidden)]
pub(crate) mod run;
#[doc(hidden)]
pub(crate) mod toolchain;
#[doc(hidden)]
pub(crate) mod util;

#[doc(hidden)]
pub use cli::main;
