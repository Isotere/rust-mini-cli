use std::error::Error;
use std::io::{self, Write}; // Почему без этого не работают write операции?

use logentry::{
    app::{Options, run},
    entry::LogLevel,
};

use clap::Parser;

#[derive(Parser)]
#[command(name = "logsum", about = "Filter and summarize log lines from stdin")]
struct Cli {
    /// Keep entries at this level or higher: DEBUG<INFO<WARNING<ERROR<FATAL
    #[arg(long)]
    min_level: Option<String>,
    /// Keep entries whose message contains this substring
    #[arg(long)]
    contains: Option<String>,
    /// Print each kept line, not just the summary
    #[arg(long)]
    show: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let opts = Options {
        min_level: cli.min_level.as_deref().map(LogLevel::from),
        contains: cli.contains,
        show: cli.show,
    };

    let stdin = io::stdin();
    let stdout = io::stdout();

    let summary = run(stdin.lock(), stdout.lock(), &opts)?;

    // summary — в stderr, чтобы не мешать прошедшим строкам в stdout
    let mut err = io::stderr().lock();
    writeln!(err, "--- summary ---")?;
    writeln!(err, "total: {}", summary.total())?;
    for (level, n) in summary.iter() {
        writeln!(err, "{level}: {n}")?;
    }

    Ok(())
}
