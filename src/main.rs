use std::env;
use clap::{ArgMatches, Parser, Subcommand};
use southbound::SouthboundResult;

fn main() -> SouthboundResult<()> {
    let args: Vec<String> = env::args().collect();
    // for m in clap::App::new("southbound").get_matches(). {
    //
    // }

    let opt: Opt = Opt::parse();

    // TODO: add debug
    // if opt.debug {
    //     std::env::set_var("RUST_LOG", "debug");
    //     env_logger::init();
    // }

    match opt.cmd {
        Command::Baseline(_) => {
            Ok(())
        }
        Command::Clean(_) => {
            Ok(())
        }
        Command::Info(_) => {
            Ok(())
        }
        Command::Migrate(_) => {
            Ok(())
        }
        Command::Repair(_) => {
            Ok(())
        }
        Command::Undo(_) => {
            Ok(())
        }
        Command::Validate(_) => {
            Ok(())
        }
    }

}

#[derive(Subcommand, Debug)]
enum Command {
    Baseline(BaselineOpt),
    Clean(CleanOpt),
    Info(InfoOpt),
    Migrate(MigrateOpt),
    Repair(RepairOpt),
    Undo(UndoOpt),
    Validate(ValidateOpt),
}

// TODO: environment variable fallback here or via config?
// should document it here regardless
// #[clap(global_setting(AppSettings::PropagateVersion))]
// #[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
// #[clap(setting(AppSettings::SubcommandRequiredElseHelp))]
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Opt {

    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Clone, Debug, Parser)]
#[clap()]
struct BaselineOpt {

}

#[derive(Clone, Debug, Parser)]
#[clap()]
struct InitOpt {

}

#[derive(Clone, Debug, Parser)]
#[clap()]
struct CleanOpt {

}

#[derive(Clone, Debug, Parser)]
#[clap(help = "Prints the details and status information about all the migrations.")]
struct InfoOpt {

}

#[derive(Clone, Debug, Parser)]
#[clap()]
struct MigrateOpt {

}

#[derive(Clone, Debug, Parser)]
#[clap()]
struct RepairOpt {

}

#[derive(Clone, Debug, Parser)]
#[clap()]
struct UndoOpt {

}

#[derive(Clone, Debug, Parser)]
#[clap()]
struct ValidateOpt {

}