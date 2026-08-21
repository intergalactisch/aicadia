use std::io::Write;

use aicadia_host_independent_change_wait_lab::{run_preflight, serve_external};
use anyhow::{Context, Result, bail};

#[tokio::main]
async fn main() -> Result<()> {
    match std::env::args().nth(1).as_deref() {
        Some("preflight") => {
            let report = run_preflight().await?;
            println!(
                "{}",
                serde_json::to_string_pretty(&report).context("encode preflight report")?
            );
            Ok(())
        }
        Some("serve") => {
            let token = std::env::args()
                .nth(2)
                .context("serve requires a controller token")?;
            let ready = serve_external(token).await?;
            println!(
                "{}",
                serde_json::to_string(&ready).context("encode server-ready record")?
            );
            std::io::stdout()
                .flush()
                .context("flush server-ready record")?;
            std::future::pending::<()>().await;
            Ok(())
        }
        Some(other) => bail!("unknown lab command: {other}"),
        None => bail!("expected lab command: preflight | serve"),
    }
}
