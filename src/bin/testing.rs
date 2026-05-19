use std::str::FromStr;

use autosaver::fs::abs::AbsPathStr;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn main() -> anyhow::Result<()> {
    init_logs();

    let abs = AbsPathStr::from_str("/home/daniele/.config/nvim")?;
    abs.find(
        |e| {
            println!("{e:?}");
            Ok(())
        },
        &mut Default::default(),
    )?;

    Ok(())
}

fn init_logs() {
    tracing_subscriber::registry()
        .with(fmt::layer().with_line_number(true))
        .with(EnvFilter::new("trace"))
        .init();
}
