#[macro_use]
extern crate tracing;

fn main() -> eyre::Result<()> {
    install_helpers(cfg!(debug_assertions))?;

    info!("A: Hello?");
    panic!("B: Hi!");
}

fn install_helpers(verbose: bool) -> eyre::Result<()> {
    use tracing::level_filters::LevelFilter;
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

    let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default().into_hooks();
    eyre_hook.install()?;

    let stderr_logs = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr)
        .with_filter(
            EnvFilter::builder()
                .with_default_directive(if verbose {
                    LevelFilter::DEBUG.into()
                } else {
                    LevelFilter::INFO.into()
                })
                .from_env_lossy(),
        );

    tracing_subscriber::registry().with(stderr_logs).init();

    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        eprintln!("{}", panic_hook.panic_report(info));
        default_panic(info);
    }));

    Ok(())
}
