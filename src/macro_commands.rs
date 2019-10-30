use slog::Logger;
use std::path::Path;
use structopt::clap::ArgMatches;

use crate::kvs::KVS;
use crate::macro_hub::MacroHub;
use crate::error::EasyErrorHandling;

use std::collections::HashMap;

use std::fs;
use std::io::Write;
use std::process;

use colored::*;

fn macro_create(logger: &Logger, args: &ArgMatches, kvs: &KVS) {
    info!(logger, "macro create");
    let macro_key = args.value_of("macro_key").unwrap();
    let macro_content = args.value_of("macro_content").unwrap();
    let is_macro_file_type = args.occurrences_of("is_macro_file_type") != 0;

    if is_macro_file_type {
        info!(
            logger,
            "verify macro content file: {} is existing", macro_content
        );
        let is_file_path_exists = Path::new(macro_content).is_file();

        if !!!is_file_path_exists {
            error!(logger, "macro content file: {} is not exists or not a regular file", macro_content; "type" => "file");
            process::exit(1);
        }

        info!(logger, "reading macro content file: {}", macro_content);
        let macro_text_content = fs::read_to_string(macro_content);

        match macro_text_content {
            Ok(_content_result) => (),
            Err(error) => {
                error!(logger, "{}", error; "type" => "read_file");
                process::exit(1);
            }
        }
    }

    kvs.save_or_update(&macro_key.to_owned(), &macro_content.to_owned());
}

fn macro_delete(logger: &Logger, args: &ArgMatches, kvs: &KVS, configs: &HashMap<String, String>) {
    info!(logger, "macro delete");
    let macro_keys: Vec<&str> = args.values_of("macro_keys").unwrap().collect();

    let bin_dir = configs.get("bin_dir").unwrap();

    for m_key in macro_keys {
        info!(logger, "delete macro key={}", m_key);
        if kvs.delete(&m_key.to_owned()) {
            let external_script_file = Path::new(bin_dir).join(m_key);
            if external_script_file.is_file() {
                process::Command::new("rm")
                    .arg("-rf")
                    .arg(external_script_file)
                    .spawn()
                    .expect("failed to delete script");
            }
            info!(logger, "success");
        } else {
            warn!(logger, "macro key={} not exist or error", m_key);
        }
    }
}

fn macro_describe(
    logger: &Logger,
    args: &ArgMatches,
    kvs: &KVS,
    configs: &HashMap<String, String>,
) {
    info!(logger, "macro describe");
    let macro_keys: Vec<&str> = args.values_of("macro_keys").unwrap().collect();

    let bin_dir = configs.get("bin_dir").unwrap();

    for m_key in macro_keys {
        if kvs.contain_key(m_key) {
            println!("{}", "-".repeat(80));
            println!("[{}]: {}", "KEY".bold(), m_key);
            println!("[{}]:", "CONTENT".bold());

            let val = kvs.get(&m_key.to_owned()).unwrap();
            println!("{}", val);

            let external_script_file = Path::new(bin_dir).join(m_key);
            let mut macro_type = "Internal";
            if external_script_file.is_file() {
                macro_type = "External";
                println!("[{}]: {:?}", "SCRIPT".bold(), external_script_file);
            }
            println!("[{}]: {} macro", "TYPE".bold(), macro_type);
        } else {
            warn!(logger, "macro key={} not exist or error", m_key);
        }
    }
}

fn macro_ls(logger: &Logger, kvs: &KVS) {
    info!(logger, "macro listing");
    kvs.show_list();
}

fn macro_run(logger: &Logger, args: &ArgMatches, kvs: &KVS) {
    info!(logger, "macro run");
    let macro_keys: Vec<&str> = args.values_of("macro_keys").unwrap().collect();

    for m_key in macro_keys {
        if kvs.contain_key(m_key) {
            info!(logger, "running macro key={}", m_key);
            let val = kvs.get(&m_key.to_owned()).unwrap();
            process::Command::new("bash")
                .arg("-c")
                .arg(val)
                .spawn()
                .expect("macro failed to start");
        } else {
            error!(logger, "macro key={} not exist", m_key);
        }
    }
}

fn macro_make_bash(
    logger: &Logger,
    args: &ArgMatches,
    kvs: &KVS,
    configs: &HashMap<String, String>,
) {
    info!(logger, "macro make bash");

    if !!!match std::env::var("USER") {
        Ok(val) => val == "root",
        Err(_error) => false,
    } {
        error!(logger, "make-bash command is required sudo privileges");
        std::process::exit(1)
    }

    let macro_keys: Vec<&str> = args.values_of("macro_keys").unwrap().collect();
    let bash_header = configs.get("bash_header").unwrap();
    let bin_dir = configs.get("bin_dir").unwrap();

    for m_key in macro_keys {
        if kvs.contain_key(m_key) {
            info!(logger, "make bash script for macro key={}", m_key);
            let val = kvs.get(&m_key.to_owned()).unwrap();
            let val_fn = format!(
                r###"{}
{}"###,
                bash_header, val
            );
            let bash_file = &format!("{}/{}", bin_dir, m_key);

            let mut file = fs::File::create(bash_file).unwrap_or_panic("Unable to create file.");
            file.write_all(String::from(val_fn).as_bytes())
                .unwrap_or_panic("Unable to write file.");

            process::Command::new("chmod")
                .arg("+x")
                .arg(bash_file)
                .spawn()
                .expect("Cannot set execute for bash script.");
        }
    }
}

fn macro_hub(logger: &Logger, args: &ArgMatches, kvs: &KVS, configs: &HashMap<String, String>) {
    let mut mh = MacroHub::new(logger, configs);

    match args.subcommand() {
        (hub_command, Some(hub_command_args)) => {
            let macro_keys = hub_command_args.values_of("macro_keys").unwrap().collect();
            mh.execute(hub_command, macro_keys, kvs);
        }
        _ => unreachable!(),
    }
}

pub fn macro_generic(
    action: &str,
    logger: &Logger,
    args: &ArgMatches,
    configs: &HashMap<String, String>,
    kvs: &KVS,
) {
    match action {
        "create" => macro_create(&logger, &args, &kvs),
        "delete" => macro_delete(&logger, &args, &kvs, &configs),
        "describe" => macro_describe(&logger, &args, &kvs, &configs),
        "ls" => macro_ls(&logger, &kvs),
        "run" => macro_run(&logger, &args, &kvs),
        "make-bash" => macro_make_bash(&logger, &args, &kvs, &configs),
        "hub" => macro_hub(&logger, &args, &kvs, &configs),
        _ => unreachable!(),
    }
}
