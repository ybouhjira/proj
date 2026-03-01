use anyhow::Result;

use crate::config::Config;
use crate::discovery;
use crate::ui;

pub async fn execute() -> Result<()> {
    let config = Config::load()?;
    let projects_dir = config.projects_dir_expanded();

    let local_projects = discovery::discover_local(&projects_dir).await?;
    let remote_projects = discovery::discover_remote(&config).await?;

    let projects = discovery::merge_projects(local_projects, remote_projects).await;

    ui::print_sync_dashboard(&projects);

    Ok(())
}
