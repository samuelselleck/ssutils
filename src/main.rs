use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use directory_collection::DirectoryCollection;
use std::{env, path::PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Ds { name: String, path: Option<PathBuf> },
    Dr { name: String },
    Dl,
    Dclear,
}

mod directory_collection;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let dirs = get_project_dirs()?;
    let data_dir = &dirs.data_local_dir();
    let dir_lib_file = data_dir.join("directory_library.json");

    let mut dir_collection = DirectoryCollection::try_load(&dir_lib_file)?;
    match cli.command {
        Commands::Ds { name, path } => {
            let path = path.ok_or(()).or_else(|_| env::current_dir())?;
            println!("saved {} as path: {:?}", name, path);
            dir_collection.insert(name, path)?;
        }
        Commands::Dr { name } => {
            let path = dir_collection
                .get(&name)
                .ok_or(anyhow!("no path with name \"{}\" saved", name))?;
            print!("{}", path.to_string_lossy());
        }
        Commands::Dl => {
            println!("{}", dir_collection);
        }
        Commands::Dclear => {
            dir_collection.clear();
        }
    }

    dir_collection.try_save()?;

    Ok(())
}

fn get_project_dirs() -> Result<ProjectDirs> {
    ProjectDirs::from("com.samuelselleck", "Samuel Selleck", "ssutils")
        .ok_or_else(|| anyhow!("couldn't find project directories"))
}

// TODO:
// - add ability to add a saved name (or a path directly) to PATH
