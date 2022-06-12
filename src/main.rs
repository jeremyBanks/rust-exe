#![doc = include_str!("../README.md")]

#[doc(hidden)]
macro_rules! docs {
        ($( $ident:ident ),* $(,)?) => {
            $(
                #[cfg(doc)]
                pub mod $ident {
                    #![doc = concat!("[🔗 _", stringify!($ident), "_][self#!]")]
                    //!
                    //! <details>
                    //! <summary>&nbsp;</summary>
                    //! <br />
                    //! <div id="!"></div>
                    //!
                    #![doc = include_str!(concat!("../doc/", stringify!($ident), ".md"))]
                    //!
                    //! <br /><br /><br /><br /><br /><br /><br /><br />
                    //! <br /><br /><br /><br /><br /><br /><br /><br />
                    //! <br /><br /><br /><br /><br /><br /><br /><br />
                    //! <br /><br /><br /><br /><br /><br /><br /><br />
                    //! </details>
                    use super::*;
                }
            )*
        }
    }

docs! {
    reference,
}
#[doc(hidden)]
#[allow(unused)]
pub use {
    crate::{crates::*, hashing::*, run::*, toolchain::*, util::*},
    ::{
        cargo_lock::Lockfile,
        eyre::Result,
        heck::*,
        quote::quote,
        serde_json::{json, Value as Json},
        std::{
            self,
            env::current_dir,
            ffi::OsString,
            os::unix::prelude::OsStrExt,
            path::PathBuf,
            time::{SystemTime, UNIX_EPOCH},
        },
        syn,
        toml_edit::easy::{toml, Value as Toml},
    },
};

#[doc(hidden)]
mod crates;
#[doc(hidden)]
mod hashing;
#[doc(hidden)]
mod run;
#[doc(hidden)]
mod toolchain;
#[doc(hidden)]
mod util;

#[doc(hidden)]
fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    env_logger::try_init()?;

    let mut args = Vec::from_iter(std::env::args_os().skip(1));

    if args.is_empty() {
        args.splice(..0, ["help".into()]);
    } else if is_path_like(args[0].as_bytes()) {
        args.splice(..0, ["run".into()]);
    }

    match args[0].as_bytes() {
        b"help" => help()?,
        b"run" => run(PathBuf::from(&args[1]), &args[2..])?,
        b"eval" => eval(
            args[1..]
                .iter()
                .map(|s| s.to_str().unwrap())
                .collect::<Vec<_>>()
                .join(" "),
            &[],
        )?,
        _ => {
            eprintln!("no such command: {:?}", &args[0]);
            help()?;
            std::process::exit(1);
        }
    }

    Ok(())
}
