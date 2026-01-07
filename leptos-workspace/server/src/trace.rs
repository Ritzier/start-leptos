use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer, fmt};

pub struct Trace;

impl Trace {
    pub fn setup() {
        let cargo_crate_name = env!("CARGO_CRATE_NAME");
        let base_filter = match cfg!(debug_assertions) {
            true => format!("{cargo_crate_name}=debug,app=debug"),
            false => format!("{cargo_crate_name}=info,app=info"),
        };

        tracing_subscriber::registry()
            .with(fmt::layer().with_writer(std::io::stdout).with_filter(
                EnvFilter::try_from_default_env().unwrap_or_else(|_| base_filter.into()),
            ))
            .init();
    }
}
