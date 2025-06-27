use crate::analyzer::analyzer_report;
use crate::cli::initialize_cli;
use crate::error::*;

mod analyzer;
mod cli;
mod domain;
mod error;
mod parser;

fn main() -> Result<()> {
    let input_data = "2025-06-27T10:00:01Z [INFO] Application starting up.
2025-06-27T10:00:05Z [INFO] Database connection established.
2025-06-27T10:01:10Z [WARNING] Configuration value 'timeout' is deprecated.
2025-06-27T10:02:00Z [ERROR] Failed to process transaction 1A4B: upstream service unavailable.
2025-06-27T10:02:01Z [ERROR] Failed to process transaction C82F: upstream service unavailable.
2025_06-27T10:02:03Z [INFO] Retrying connection to upstream service.
2025-06-27T10:02:04Z [ERROR] Failed to process transaction 9F0E: upstream service unavailable.
2025-06-27T10:02:05Z [ERROR] Retrying failed, escalating issue.
2025-06-27T10:02:06Z [ERROR] Circuit breaker opened for upstream service.
2025-06-27T10:03:06Z [ERROR] Circuit breaker opened for upstream service.
2025-06-27T10:03:08Z [ERROR] Circuit breaker opened for upstream service.
2025-06-27T10:05:00Z [INFO] System recovered.";

    let analysis = analyzer_report(input_data)?;
    initialize_cli(analysis);

    Ok(())
}
