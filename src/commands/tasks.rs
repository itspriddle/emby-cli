use crate::cli::{TasksArgs, TasksCommand};
use crate::client::Client;
use crate::config::Config;
use crate::emby::ticks;
use crate::emby::types::TaskInfo;
use crate::error::Result;
use crate::format::table;

pub fn run(args: &TasksArgs) -> Result<()> {
    match &args.command {
        Some(TasksCommand::Run(run_args)) => run_task(&run_args.id),
        None => list_tasks(args.all),
    }
}

fn list_tasks(show_all: bool) -> Result<()> {
    let config = Config::load()?;
    let client = Client::new(&config);
    let tasks: Vec<TaskInfo> = client.get("/ScheduledTasks")?;

    let tasks: Vec<&TaskInfo> = tasks
        .iter()
        .filter(|t| show_all || !t.is_hidden.unwrap_or(false))
        .collect();

    if tasks.is_empty() {
        println!("No scheduled tasks found");
        return Ok(());
    }

    let rows: Vec<Vec<String>> = tasks
        .iter()
        .map(|t| {
            let category = t.category.as_deref().unwrap_or("").to_string();
            let name = t.name.as_deref().unwrap_or("").to_string();
            let state = t.state.as_deref().unwrap_or("").to_string();
            let (last_run, last_status) = t
                .last_execution_result
                .as_ref()
                .map(|r| {
                    let run = r
                        .end_time_utc
                        .as_deref()
                        .map_or_else(String::new, ticks::format_premiere_date);
                    let status = r.status.as_deref().unwrap_or("").to_string();
                    (run, status)
                })
                .unwrap_or_default();
            let id = t.id.as_deref().unwrap_or("").to_string();

            vec![category, name, state, last_run, last_status, id]
        })
        .collect();

    println!(
        "{}",
        table::build_table(
            &["Category", "Name", "State", "Last Run", "Last Status", "ID"],
            rows
        )
    );

    Ok(())
}

fn run_task(id: &str) -> Result<()> {
    let config = Config::load()?;
    let client = Client::new(&config);
    client.post(&format!("/ScheduledTasks/Running/{id}"), None)?;
    println!("Task {id} started");
    Ok(())
}
