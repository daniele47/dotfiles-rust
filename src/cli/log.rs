use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub struct LogOptions<'a> {
    level: &'a str,
}

impl<'a> LogOptions<'a> {
    pub fn new(level: &'a str) -> Self {
        Self { level }
    }

    pub fn init(&self) {
        tracing_subscriber::registry()
            .with(fmt::layer())
            .with(EnvFilter::new(self.level))
            .init();
    }
}
