use crate::*;

#[allow(unused)]
pub fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let argv: &Vec<OsString> = &std::env::args_os().collect();
    let arg = &argv[1..];
    let exe = &std::env::current_exe()?;
    let cwd = &std::env::current_dir()?;
    let env: &IndexMap<OsString, OsString> = &std::env::vars_os().collect();

    let mut flags_quiet = 0;
    let mut flags_verbose = 0;

    // let arg: Vec<_> = arg.iter().filter(|arg| {
    //     match arg.as_bytes() {
    //         b"-q" | b"--quiet" => {
    //             flags_quiet += 1;
    //             false
    //         }
    //         b"-v" | b"--verbose" => {
    //             flags_verbose += 1;
    //             false
    //         }
    //         _ => true,
    //     }
    // }).collect();

    let default_verbosity = 3;

    let log_env = std::env::var("RUST_LOG").unwrap_or_default();

    let log_level = if flags_verbose == 0 && flags_quiet == 0 && !log_env.is_empty() {
        log_env
    } else {
        match default_verbosity + flags_verbose - flags_quiet {
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
            .with_env_filter(::tracing_subscriber::EnvFilter::new(log_level))
            .with_span_events(
                tracing_subscriber::fmt::format::FmtSpan::ENTER
                    | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
            )
            .compact()
            .finish(),
    ));

    let mut args: Vec<OsString> = arg.iter().map(|x| OsString::clone(x)).collect();

    trace!("args = {args:?}");
    trace!("exe = {exe:?}");
    trace!("cwd = {cwd:?}");

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
