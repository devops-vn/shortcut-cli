use dirs;

use config::Config;
use std::collections::HashMap;
use std::path::PathBuf;

// This macro to allow for concise init of arbitrary values
// Used like `let hm = p_hashmap!['foo' => 1, 'bar' => 2]`
macro_rules! p_hashmap {
    ($( $key: expr => $val: expr ), *) => {{
        let mut p_map = HashMap::new();
        $( p_map.insert($key, $val); )*
        p_map
    }}
}

#[macro_export]
macro_rules! vec_of_url {
    ($($x:expr),*) => (
        vec![$($x.to_string()),*].join("/")
    );
}

pub fn config() -> HashMap<String, String> {
    let settings_file_path = get_config_file_dir(".shortcut-cli/config.yml");
    let mut settings = Config::default();
    let mut settings_dict = HashMap::new();

    // Make sure the config exists.
    // If not then use default configs.
    if std::fs::metadata(settings_file_path).is_ok() {
        let w_name = config::File::with_name(settings_file_path);
        settings.merge(w_name).unwrap();
        settings_dict = settings.try_into::<HashMap<String, String>>().unwrap();
    }

    let mut default_configs = p_hashmap![
        "bash_header".to_owned() => "#!/bin/bash -e".to_owned(),
        "bin_dir".to_owned() => "/usr/local/bin".to_owned(),
        "log_type".to_owned() => "term".to_owned(),
        "root_dir".to_owned() => get_config_file_dir(".shortcut-cli").to_owned(),
        "hub_url".to_owned() => "https://pastebin.com/raw".to_owned()
    ];

    for (key, val) in default_configs.iter_mut() {
        *val = settings_dict.get(key).cloned().unwrap_or(val.to_string());
    }

    default_configs
}

fn get_config_file_dir(config_file: &str) -> &'static str {
    let dir: PathBuf = match dirs::home_dir() {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(""),
    };

    let final_config_file_path: String = dir
        .join(config_file)
        .into_os_string()
        .into_string()
        .unwrap();

    // FIXME: This is bad idea for String to &'static str
    Box::leak(final_config_file_path.into_boxed_str())
}
