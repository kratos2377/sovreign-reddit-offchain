use common_tracing::initialize_logging_and_tracing;
use anyhow::Error;
use crate::conf::configuration::Configuration;

pub fn init(config: &Configuration) -> Result<(), Error> {
    Ok(initialize_logging_and_tracing(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        |mut e| {
            // Configure root level
            if let Some(root_level) = &config.logging.level.root {
                e = e.add_directive(root_level.parse().unwrap_or_default())
            }

            // Configure specific directives
            for directive in &config.logging.level.directives {
                let directive_string = format!("{}={}", directive.namespace, directive.level);
                e = e.add_directive(directive_string.parse().unwrap_or_default());
            }

            e
        },
    )?)
}