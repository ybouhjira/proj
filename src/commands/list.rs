use anyhow::Result;

use crate::config::Config;
use crate::discovery;
use crate::ui;

pub async fn execute(remote: bool, local: bool, all: bool) -> Result<()> {
    let config = Config::load()?;
    let projects_dir = config.projects_dir_expanded();

    let local_projects = if !remote {
        discovery::discover_local(&projects_dir).await?
    } else {
        Vec::new()
    };

    let remote_projects = if !local {
        discovery::discover_remote(&config).await?
    } else {
        Vec::new()
    };

    let projects = discovery::merge_projects(local_projects, remote_projects).await;

    ui::print_project_table(&projects, all || remote);

    Ok(())
}
