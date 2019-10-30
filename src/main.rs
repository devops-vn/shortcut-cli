use std::io::Result;
use structopt::StructOpt;

use shortcut_cli::kvs::KVS;
use shortcut_cli::log;
use shortcut_cli::macro_commands::*;
use shortcut_cli::utils;

#[warn(unused_imports)]

/// A simple shortcut cli
#[derive(StructOpt)]
#[structopt(name = "shortcut-cli", author = "Cuong Le <metacuong@gmail.com>")]
#[allow(dead_code)]
struct DevOpsCLI {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
#[allow(dead_code)]
enum Command {
    #[structopt(name = "macro")]
    /// Macro scripts management.
    Macro(Macro),
}

#[derive(StructOpt)]
#[allow(dead_code)]
struct Macro {
    #[structopt(subcommand)]
    macro_commands: MacroCommands,
}

#[derive(StructOpt)]
#[allow(dead_code)]
enum MacroCommands {
    #[structopt(name = "run")]
    /// Run macro(s).
    Run {
        #[structopt(long = "async", short = "a")]
        /// Run async macro(s).
        /// If this flag is not specified then `sync` method will be used.
        r#async: bool,
        #[structopt(long = "key", short = "k", required = true)]
        /// Macro key(s). (required) e.g.: foo bar
        macro_keys: Vec<String>,
    },

    #[structopt(name = "create")]
    /// Create new macro script.
    Create {
        #[structopt(long = "key", short = "k")]
        /// Macro key. (required)
        macro_key: String,
        #[structopt(long = "macro-file-type", short = "f")]
        /// The macro type default is `TEXT`.
        /// If this flag is specified then the macro type will use `FILE` instead.
        is_macro_file_type: bool,
        #[structopt(long = "content", short = "c")]
        /// Macro content. (required)
        /// If macro type = `f` -> content = `/path/to/script/file` else content = `Foo Bar`
        macro_content: String,
    },

    #[structopt(name = "delete")]
    /// Delete macro(s) script.
    Delete {
        #[structopt(long = "key", short = "k", required = true)]
        /// Macro key(s). (required) e.g.: foo bar
        macro_keys: Vec<String>,
    },

    #[structopt(name = "describe")]
    /// View content of macro(s) script.
    Describe {
        #[structopt(long = "key", short = "k", required = true)]
        /// Macro key(s). (required) e.g.: foo bar
        macro_keys: Vec<String>,
    },

    #[structopt(name = "ls")]
    /// Listing all macro scripts in this marchine.
    Listing {
        #[structopt(long = "filter", short = "f", required = false)]
        /// Filter with a regular expression.
        regex: String,
    },

    #[structopt(name = "make-bash")]
    /// Make bash script for macro(s) in this marchine (Require sudo privileges)
    /// e.g.: /usr/bin/foo ; /usr/bin/bar
    MakeBash {
        #[structopt(long = "key", short = "k", required = true)]
        /// Macro key(s). (required) e.g.: foo bar
        macro_keys: Vec<String>,
    },

    #[structopt(name = "hub")]
    /// Management existing macros from a macro hub. e.g.: pastebin.com
    Hub(HubMacroSubCommands),
}

#[derive(StructOpt)]
#[allow(dead_code)]
enum HubMacroSubCommands {
    #[structopt(name = "search")]
    /// Search macro(s) on a macro hub. e.g.: pastebin.com
    Search {
        #[structopt(long = "key", short = "k", required = true)]
        /// Macro key(s). (required) e.g.: foo bar
        macro_keys: Vec<String>,
    },
    #[structopt(name = "add")]
    /// Add existing macro to local machine.
    Add {
        #[structopt(long = "key", short = "k", required = true)]
        /// Macro key(s). (required) e.g.: foo bar
        macro_keys: Vec<String>,
    },
}

fn main() -> Result<()> {
    let matches = DevOpsCLI::clap().get_matches();
    let config = utils::config();
    let mut logger = log::init_term();

    if let Some(log_type) = config.get("log_type") {
        if log_type == "file" {
            logger = log::init_file();
        }
    }

    let kvs = KVS::new(&logger, &config);

    match matches.subcommand() {
        ("macro", Some(macro_cmd)) => match macro_cmd.subcommand() {
            (macro_action, Some(macro_action_args)) => {
                macro_generic(&macro_action, &logger, &macro_action_args, &config, &kvs)
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }

    Ok(())
}
