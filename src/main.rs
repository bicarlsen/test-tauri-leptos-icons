mod app;

use app::*;
use leptos::prelude::*;

fn main() {
    tracing::enable();
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}



mod tracing {
    use tracing_subscriber::{filter, fmt::time::UtcTime, prelude::*};

    pub fn enable() {
        let target_filter = filter::Targets::new()
            .with_target("syre", tracing::Level::TRACE)
            .with_target("leptos", tracing::Level::TRACE);
        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_ansi(false) // Only partially supported across browsers
            .with_timer(UtcTime::rfc_3339())
            .pretty()
            .with_writer(tracing_web::MakeWebConsoleWriter::new()); // write events to the console

        let perf_layer = tracing_web::performance_layer()
            .with_details_from_fields(tracing_subscriber::fmt::format::Pretty::default());

        let layers = tracing_subscriber::registry().with(fmt_layer);

        let layers = layers.with(perf_layer);

        layers.with(target_filter).init();
    }
}

