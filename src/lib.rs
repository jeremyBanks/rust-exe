#![doc = include_str!("../README.md")]

#[cfg(doc)]
pub mod reference {
    #![doc = include_str!("../reference.md")]
}

#[doc(hidden)]
#[allow(unused)]
pub(crate) use {
    crate::{arg_stream::*, cli::*, crates::*, git_hashing::*, run::*, toolchain::*, util::*},
    ::{
        cargo_lock::Lockfile,
        eyre::Result,
        heck::*,
        indexmap::{IndexMap, IndexSet},
        quote::quote,
        serde_json::{json, Value as Json},
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
