use aicadia_host_independent_change_wait_lab::{
    AdapterScenario, SubjectState, WaitInput, WaitResult,
};
use anyhow::{Context, Result, ensure};

#[tokio::main]
async fn main() -> Result<()> {
    let base_url = std::env::args()
        .nth(1)
        .context("client requires the lab base URL")?;
    let client = reqwest::Client::new();
    let initial: SubjectState = client
        .get(format!("{base_url}/state"))
        .header("x-lab-client", "terminal")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    let wait_result: WaitResult = client
        .post(format!("{base_url}/wait"))
        .header("x-lab-client", "terminal")
        .json(&WaitInput::table(initial.version, 30_000))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    ensure!(
        !wait_result.timed_out && wait_result.changed_subjects.contains(&initial.subject),
        "terminal wait ended without a changed Table"
    );
    let final_state: SubjectState = client
        .get(format!("{base_url}/state"))
        .header("x-lab-client", "terminal")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    let report = AdapterScenario {
        adapter: "terminal_http".to_owned(),
        initial,
        wait_result,
        final_state,
        observations: Vec::new(),
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&report).context("encode terminal report")?
    );
    Ok(())
}
