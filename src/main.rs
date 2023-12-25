use clap::Parser;
use clap::ArgGroup;
use clap::Arg;
use clap::Subcommand;


use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    database: String,
}

#[derive(Parser)]
#[clap(version = "1.0")]
struct Opts {
    #[clap(subcommand)]
    subcmd: KeedotsCommand,
}

#[derive(Parser)]
enum KeedotsCommand {
    Init(Init),
    Add(Add),
    Commit(Commit),
    Update(Update),
    Describe(Describe),
}

#[derive(Parser)]
struct Init {
    #[clap(name = "file", short, long)]
    database_path: Option<String>,
}
#[derive(Parser)]
struct Add {
    #[clap(name = "file", short, long)]
    file: String,
}

#[derive(Parser)]
struct Commit {
}

#[derive(Parser)]
struct Update {
}

#[derive(Parser)]
struct Describe {
}



fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        KeedotsCommand::Init(t) => {
            init_command(t.database_path)
        }
        KeedotsCommand::Add(t) => {
            println!("Adding file: {}", t.file);
            // Kod do wykonania, gdy wywołano komendę "add"
        },
        KeedotsCommand::Commit(t) => {
            // Kod do wykonania, gdy wywołano komendę "commit"
        },
        KeedotsCommand::Update(t) => {
            // Kod do wykonania, gdy wywołano komendę "update"
        },
        KeedotsCommand::Describe(t) => {
            // Kod do wykonania, gdy wywołano komendę "describe"
        },
    }
}
fn init_command(database_path: Option<String>) {
    let config_dir =
        directories_next::ProjectDirs::from("com", "keedots", "keedots")
            .expect("Unable to get config directory")
            .config_dir()
            .join("keedots.yaml");
    let database_path = database_path.unwrap_or_else(|| {
        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir.parent().unwrap())
                .expect("Could not create config directory");
        }
        config_dir
            .parent()
            .unwrap()
            .join("keedots.kdbx")
            .to_str()
            .unwrap()
            .to_string()
    });
    let config = Config { database: database_path.clone() };
    let config_string = serde_yaml::to_string(&config).unwrap();
    std::fs::write(config_dir, config_string).expect("Unable to write config");
    if !std::path::Path::new(&database_path).exists() {
        // Tutaj tworzymy bazę danych KeePass, wykorzystując bibliotekę `keepass`
        // ...
    }
    println!("Initialization finished successfully");
}
