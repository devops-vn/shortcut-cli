#[macro_use]
extern crate lazy_static;

use shortcut_cli::kvs::KVS;
use shortcut_cli::log;
use shortcut_cli::macro_commands::macro_generic;
use shortcut_cli::utils;
use slog::Logger;
use std::collections::HashMap;
use structopt::clap::{App, Arg};

#[macro_use]
lazy_static! {
    static ref CONFIG: HashMap<String, String> = utils::config();
    static ref LOGGER: Logger = log::init_term();
}

#[test]
fn test_macro_commands() {
    run_test(|| {
        macro_command_create();
        macro_command_delete();
    })
}

fn run_test<T>(test: T) -> ()
where
    T: FnOnce() -> () + std::panic::UnwindSafe,
{
    let result = std::panic::catch_unwind(|| test());

    assert!(result.is_ok())
}

fn macro_command_create() {
    let kvs = KVS::new(&LOGGER, &CONFIG);

    let args = App::new("create")
        .arg(Arg::from_usage("-k, --macro_key=[macro_key] 'macro_key'").default_value("test"))
        .arg(
            Arg::from_usage("-c, --macro_content=[macro_content] 'macro_content'")
                .default_value("TEST"),
        )
        .get_matches_from_safe(vec!["create"]);

    assert!(args.is_ok());

    macro_generic("create", &LOGGER, &args.unwrap(), &CONFIG, &kvs);

    let val = kvs.get(&"test".to_owned()).unwrap();

    assert_eq!("TEST", val);
}

fn macro_command_delete() {
    let kvs = KVS::new(&LOGGER, &CONFIG);

    let args = App::new("delete")
        .arg(Arg::from_usage("-k, --macro_keys=[macro_keys] 'macro_keys'").default_value("test"))
        .get_matches_from_safe(vec!["delete"]);

    assert!(args.is_ok());

    macro_generic("delete", &LOGGER, &args.unwrap(), &CONFIG, &kvs);

    assert_eq!(false, kvs.get(&"test".to_owned()).is_ok());
}
