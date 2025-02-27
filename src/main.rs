use octocrab::params::repos::PullRequestSort;
use octocrab::models::pulls::PullRequest;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the GitHub token from the environment
    let token = env::var("GITHUB_TOKEN")?;
    let octocrab = octocrab::Octocrab::builder()
        .personal_token(token)
        .build()?;

    // Get the repository owner and name from the environment
    let repo_owner = env::var("GITHUB_REPOSITORY")?;
    let (owner, repo) = repo_owner.split_once('/').ok_or("Invalid repository name")?;

    // Get the pull request number from the event
    let pr_number = env::var("GITHUB_EVENT_PATH")?;
    let event_data = std::fs::read_to_string(pr_number)?;
    let event: serde_json::Value = serde_json::from_str(&event_data)?;

    let pull_request_number = event["pull_request"]["number"]
        .as_i64()
        .ok_or("Failed to extract pull request number")?;

    println!("Pull Request Number: {}", pull_request_number);

    Ok(())
}