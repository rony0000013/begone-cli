use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use log::{debug, error, info, warn};
use std::{
    fs,
    path::Path,
};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "begone")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Run in dry-run mode (don't delete anything)
    #[arg(short, long, default_value_t = false)]
    dry_run: bool,
    /// Enable verbose output
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Clean Rust project directories (target/)
    Rust,
    /// Clean Python project directories (.venv/)
    Python,
    /// Clean JavaScript/TypeScript project directories (node_modules/)
    Js,
    /// Clean Java project directories (target/)
    Java,
    /// Clean Go project directories (bin/, pkg/)
    Go,
    /// Clean .NET project directories (bin/, obj/)
    Dotnet,
    /// Clean all supported project directories
    All,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logger
    let mut builder = env_logger::Builder::from_default_env();
    if cli.verbose {
        builder.filter_level(log::LevelFilter::Debug);
    } else {
        builder.filter_level(log::LevelFilter::Info);
    }
    builder.init();

    let current_dir = std::env::current_dir()?;
    debug!("Current directory: {}", current_dir.display());

    match &cli.command {
        Commands::Rust => clean_rust(&current_dir, cli.dry_run)?,
        Commands::Python => clean_python(&current_dir, cli.dry_run)?,
        Commands::Js => clean_js(&current_dir, cli.dry_run)?,
        Commands::Java => clean_java(&current_dir, cli.dry_run)?,
        Commands::Go => clean_go(&current_dir, cli.dry_run)?,
        Commands::Dotnet => clean_dotnet(&current_dir, cli.dry_run)?,
        Commands::All => {
            clean_rust(&current_dir, cli.dry_run)?;
            clean_python(&current_dir, cli.dry_run)?;
            clean_js(&current_dir, cli.dry_run)?;
            clean_java(&current_dir, cli.dry_run)?;
            clean_go(&current_dir, cli.dry_run)?;
            clean_dotnet(&current_dir, cli.dry_run)?;
        }
    }

    Ok(())
}

fn clean_rust(dir: &Path, dry_run: bool) -> Result<()> {
    clean_directories(
        dir,
        &["Cargo.toml"],
        &["target"],
        "Rust",
        dry_run,
    )
}

fn clean_python(dir: &Path, dry_run: bool) -> Result<()> {
    clean_directories(
        dir,
        &["requirements.txt", "pyproject.toml", "setup.py", "Pipfile"],
        &[".venv", "venv", "__pycache__", ".pytest_cache", ".mypy_cache"],
        "Python",
        dry_run,
    )
}

fn clean_js(dir: &Path, dry_run: bool) -> Result<()> {
    clean_directories(
        dir,
        &["package.json"],
        &["node_modules", ".next", ".nuxt", ".cache", "dist", "build"],
        "JavaScript/TypeScript",
        dry_run,
    )
}

fn clean_java(dir: &Path, dry_run: bool) -> Result<()> {
    clean_directories(
        dir,
        &["pom.xml", "build.gradle", "build.gradle.kts"],
        &["target", "build", ".gradle", ".classpath"],
        "Java",
        dry_run,
    )
}

fn clean_go(dir: &Path, dry_run: bool) -> Result<()> {
    clean_directories(
        dir,
        &["go.mod", "go.sum"],
        &["bin", "pkg", "__debug_bin"],
        "Go",
        dry_run,
    )
}

fn clean_dotnet(dir: &Path, dry_run: bool) -> Result<()> {
    clean_directories(
        dir,
        &["*.csproj", "*.fsproj", "*.sln"],
        &["bin", "obj"],
        ".NET",
        dry_run,
    )
}

fn clean_directories(
    root_dir: &Path,
    indicator_files: &[&str],
    target_dirs: &[&str],
    language: &str,
    dry_run: bool,
) -> Result<()> {
    info!("Cleaning {} projects in: {}", language, root_dir.display());
    let mut cleaned = 0;
    let mut skipped = 0;

    // Walk through all directories
    for entry in WalkDir::new(root_dir)
        .min_depth(0) 
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir())
    {
        let dir_path = entry.path();
        
        // Check if this directory contains any of the indicator files
        let has_indicator = indicator_files.iter().any(|pattern| {
            if pattern.starts_with('*') {
                // Handle wildcard patterns like "*.csproj"
                let pattern = &pattern[1..]; // Remove the leading '*'
                dir_path
                    .read_dir()
                    .map(|mut entries| {
                        entries.any(|e| {
                            e.ok()
                                .and_then(|e| e.file_name().to_str().map(|s| s.ends_with(pattern)))
                                .unwrap_or(false)
                        })
                    })
                    .unwrap_or(false)
            } else {
                dir_path.join(pattern).exists()
            }
        });

        if has_indicator {
            for target_dir in target_dirs {
                let target_path = dir_path.join(target_dir);
                if target_path.exists() {
                    if dry_run {
                        println!(
                            "{} {} {}",
                            "Would remove:".yellow().bold(),
                            target_path.display(),
                            format!("({} project)", language).dimmed()
                        );
                        cleaned += 1;
                    } else {
                        match fs::remove_dir_all(&target_path) {
                            Ok(_) => {
                                println!(
                                    "{} {}",
                                    "Removed:".green().bold(),
                                    target_path.display()
                                );
                                cleaned += 1;
                            }
                            Err(e) => {
                                error!(
                                    "Failed to remove {}: {}",
                                    target_path.display(),
                                    e
                                );
                                skipped += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    if cleaned > 0 || skipped > 0 {
        let action = if dry_run { "Would remove" } else { "Removed" };
        info!(
            "{} {} {}{}",
            action,
            cleaned,
            language,
            if cleaned != 1 { " projects" } else { " project" },
        );
        if skipped > 0 {
            warn!("Failed to remove {} directories (permission denied or in use)", skipped);
        }
    } else {
        info!("No {} projects found to clean", language);
    }

    Ok(())
}
