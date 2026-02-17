use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "emby", about = "CLI for some random stuff in Emby")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Trigger library scans
    Scan(ScanArgs),

    /// Show what's currently playing
    Playing(PlayingArgs),

    /// Restart Emby
    Restart,

    /// Show system information
    System,

    /// List users
    Users,

    /// List devices
    Devices,

    /// List libraries
    Libraries,

    /// Find Emby servers on the local network
    FindServer(FindServerArgs),

    /// Show recent activity log
    Activity(ActivityArgs),

    /// Show recently added media
    Latest(LatestArgs),

    /// Search the library
    Search(SearchArgs),

    /// Show next episodes to watch
    NextUp(NextUpArgs),

    /// Show upcoming TV episodes
    Upcoming(UpcomingArgs),

    /// List and run scheduled tasks
    Tasks(TasksArgs),
}

#[derive(clap::Args)]
pub struct FindServerArgs {
    /// Discovery timeout in seconds
    #[arg(short, long, default_value_t = 3)]
    pub timeout: u64,
}

#[derive(clap::Args)]
#[allow(clippy::struct_excessive_bools)]
pub struct ScanArgs {
    /// Library types to scan (shows, movies, music, all)
    #[arg(default_value = "all")]
    pub libraries: Vec<String>,

    /// Recursively scan directories
    #[arg(short = 'r', long, default_value_t = true)]
    pub recursive: bool,

    /// Disable recursive scanning
    #[arg(short = 'R', long = "no-recursive", conflicts_with = "recursive")]
    pub no_recursive: bool,

    /// Metadata refresh mode
    #[arg(long, default_value = "Default")]
    pub metadata_refresh_mode: String,

    /// Image refresh mode
    #[arg(long, default_value = "Default")]
    pub image_refresh_mode: String,

    /// Replace all metadata
    #[arg(long)]
    pub replace_all_metadata: bool,

    /// Replace all images
    #[arg(long)]
    pub replace_all_images: bool,
}

#[derive(clap::Args)]
pub struct PlayingArgs {
    /// Don't colorize output
    #[arg(short, long, group = "format")]
    pub plain: bool,

    /// Show results in JSON format
    #[arg(short, long, group = "format")]
    pub json: bool,

    /// Show raw JSON payload from the Emby API
    #[arg(short, long, group = "format")]
    pub raw: bool,

    /// Filter by user names
    pub users: Vec<String>,
}

#[derive(clap::Args)]
pub struct ActivityArgs {
    /// Maximum number of entries to show
    #[arg(short, long, default_value_t = 25)]
    pub limit: u32,
}

#[derive(clap::Args)]
pub struct LatestArgs {
    /// Maximum number of items to show
    #[arg(short, long, default_value_t = 20)]
    pub limit: u32,

    /// Filter by type (movies, shows, music)
    #[arg(short, long)]
    pub r#type: Option<String>,

    /// User name (defaults to first admin user)
    #[arg(short, long)]
    pub user: Option<String>,
}

#[derive(clap::Args)]
pub struct SearchArgs {
    /// Search query
    pub query: String,

    /// Maximum number of results
    #[arg(short, long, default_value_t = 25)]
    pub limit: u32,
}

#[derive(clap::Args)]
pub struct NextUpArgs {
    /// Maximum number of items to show
    #[arg(short, long, default_value_t = 20)]
    pub limit: u32,

    /// User name (defaults to first admin user)
    #[arg(short, long)]
    pub user: Option<String>,
}

#[derive(clap::Args)]
pub struct UpcomingArgs {
    /// Maximum number of items to show
    #[arg(short, long, default_value_t = 20)]
    pub limit: u32,

    /// User name (defaults to first admin user)
    #[arg(short, long)]
    pub user: Option<String>,
}

#[derive(clap::Args)]
pub struct TasksArgs {
    /// Show hidden tasks
    #[arg(short, long)]
    pub all: bool,

    #[command(subcommand)]
    pub command: Option<TasksCommand>,
}

#[derive(Subcommand)]
pub enum TasksCommand {
    /// Run a scheduled task
    Run(TasksRunArgs),
}

#[derive(clap::Args)]
pub struct TasksRunArgs {
    /// Task ID to run
    pub id: String,
}
