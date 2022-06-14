use crate::*;

#[allow(unused)]
pub fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let argv: &Vec<OsString> = &std::env::args_os().collect();
    let arg = &argv[1..];
    let exe = &std::env::current_exe()?;
    let cwd = &std::env::current_dir()?;
    let env: &IndexMap<OsString, OsString> = &std::env::vars_os().collect();

    let entry = CliEntry::try_new(arg.iter().cloned().collect())?;

    let default_verbosity = 3;

    let log_env = std::env::var("RUST_LOG").unwrap_or_default();

    let log_level = if entry.verbosity.is_none() && !log_env.is_empty() {
        log_env
    } else {
        // TODO: default to `warn` for all crates and `info` for our own(?)
        match default_verbosity + entry.verbosity.unwrap_or(0) {
            i32::MIN..=0 => "off".into(),
            1 => "error".into(),
            2 => "warn".into(),
            3 => "info".into(),
            4 => "debug".into(),
            5..=i32::MAX => "trace".into(),
        }
    };

    tracing_subscriber::util::SubscriberInitExt::init(tracing_subscriber::Layer::with_subscriber(
        tracing_error::ErrorLayer::default(),
        tracing_subscriber::fmt()
            .without_time()
            .with_env_filter(::tracing_subscriber::EnvFilter::new(log_level))
            .with_span_events(
                tracing_subscriber::fmt::format::FmtSpan::ENTER
                    | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
            )
            .compact()
            .finish(),
    ));

    trace!("{entry:#?}");

    match entry.subcommand {
        Subcommand::Help(_args) => help()?,
        Subcommand::Run(mut args) => run(args.next_path().unwrap(), args.all())?,
        Subcommand::Eval(args) => eval(
            args.into_iter()
                .map(|s| s.into_string().unwrap())
                .collect::<Vec<_>>()
                .join(" "),
            &[],
        )?,
    }

    Ok(())
}

#[test]
fn test_cli_entry() {
    expect![[r#"
        CliEntry {
            verbosity: None,
            subcommand: Run(
                ArgStream {
                    args: [
                        "",
                        "run",
                        "./target/debug/hello",
                        "--to",
                        "world",
                    ],
                    offset: 2,
                },
            ),
        }
    "#]]
    .assert_debug_eq(
        &CliEntry::try_new(
            ["./target/debug/hello", "--to", "world"]
                .iter()
                .map(Into::into)
                .collect(),
        )
        .unwrap(),
    );

    expect![[r#"
        CliEntry {
            verbosity: Some(
                -1,
            ),
            subcommand: Run(
                ArgStream {
                    args: [
                        "--verbose",
                        "run",
                        "-qq",
                        "./hello.rs",
                        "--to",
                        "world",
                    ],
                    offset: 3,
                },
            ),
        }
    "#]]
    .assert_debug_eq(
        &CliEntry::try_new(
            ["--verbose", "run", "-qq", "./hello.rs", "--to", "world"]
                .iter()
                .map(Into::into)
                .collect(),
        )
        .unwrap(),
    );
}

#[derive(Debug, Clone)]
pub struct CliEntry {
    pub verbosity: Option<i32>,
    pub subcommand: Subcommand,
}

#[derive(Debug, Clone)]
pub enum Subcommand {
    Run(ArgStream),
    Eval(ArgStream),
    Help(ArgStream),
}

impl CliEntry {
    pub fn try_new(args: Vec<OsString>) -> eyre::Result<Self> {
        let mut args = ArgStream::new(args);

        dbg!(&args);

        if args.peek_path().is_some() {
            args.push_front("run".into());
        }

        let mut verbosity: Option<i32> = None;

        let options_before_subcommand = args.next_options();

        let subcommand = args.next_subcommand().unwrap_or_else(|| "help".into());

        let options_after_subcommand = args.next_options();

        let options = options_before_subcommand
            .into_iter()
            .chain(options_after_subcommand);

        for option in options {
            dbg!(&option);
            if let Some(option_bytes) = option.as_bytes().strip_prefix(b"--") {
                match option_bytes {
                    b"verbose" => {
                        verbosity = Some(verbosity.unwrap_or(0) + 1);
                    }
                    b"quiet" => {
                        verbosity = Some(verbosity.unwrap_or(0) - 1);
                    }
                    _ => {
                        eyre::bail!("unrecognized long argument: {:?}", option.to_string_lossy());
                    }
                }
            } else if let Some(option_bytes) = option.as_bytes().strip_prefix(b"-") {
                for arg_byte in option_bytes {
                    match arg_byte {
                        b'v' => {
                            verbosity = Some(verbosity.unwrap_or(0) + 1);
                        }
                        b'q' => {
                            verbosity = Some(verbosity.unwrap_or(0) - 1);
                        }
                        _ => {
                            eyre::bail!(
                                "unrecognized short argument: {:?}",
                                option.to_string_lossy()
                            );
                        }
                    }
                }
            } else {
                unreachable!()
            }
        }

        Ok(CliEntry {
            verbosity,
            subcommand: match subcommand.as_bytes() {
                b"run" => Subcommand::Run(args),
                b"eval" => Subcommand::Eval(args),
                b"help" => Subcommand::Help(args),
                _ => eyre::bail!(
                    "unrecognized subcommand: {:?}",
                    subcommand.to_string_lossy()
                ),
            },
        })
    }
}
