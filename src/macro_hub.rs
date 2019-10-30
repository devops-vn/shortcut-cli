use slog::Logger;
use std::collections::HashMap;

use yaml_rust::YamlLoader;

use crate::error::EasyErrorHandling;
use crate::kvs::KVS;
use regex::Regex;

use colored::*;

#[derive(Debug)]
pub struct MacroHub<'a> {
    logger: &'a Logger,
    config: &'a HashMap<String, String>,
    hub_list_uri: &'a str,
    macros_hub: HashMap<String, String>,
}

impl<'a> MacroHub<'a> {
    pub fn new(logger: &'a Logger, config: &'a HashMap<String, String>) -> Self {
        let hub_list_uri = "Eqi7uExx";
        let macros_hub: HashMap<String, String> = HashMap::new();

        Self {
            logger,
            config,
            hub_list_uri,
            macros_hub,
        }
    }

    pub fn execute(&mut self, cmd: &str, mut macro_keys: Vec<&str>, kvs: &KVS) {
        // Ensure macro list always up-to-date before search
        self.update();

        if cmd == "search" {
            self.maybe_wildcards_search(&mut macro_keys);

            if macro_keys.is_empty() {
                info!(self.logger, "nothing to do!");
            }
        }

        for m_key in macro_keys {
            match self.macros_hub.get(m_key) {
                Some(val) => {
                    info!(self.logger, "found macro key={}", m_key);
                    match cmd {
                        "search" => println!("{0: <20}\n{1: <80}", "CONTENT".bold(), val),
                        "add" => {
                            info!(self.logger, "add macro key={} to local machine", m_key);
                            kvs.save_or_update(&m_key.to_string(), val);
                        }
                        _ => (),
                    }
                }
                None => warn!(self.logger, "macro key={} not found", m_key),
            }
        }
    }

    fn maybe_wildcards_search<'b>(&'b self, macro_keys: &'b mut Vec<&str>) {
        let old_macro_keys = macro_keys.clone();

        macro_keys.clear();

        for m_key in old_macro_keys {
            if m_key.contains("*") {
                let s_key = m_key.replace("*", "(.*)");
                let r_key = Regex::new(&s_key).unwrap();
                for (org_key, _) in self.macros_hub.iter() {
                    match r_key.captures(org_key) {
                        Some(_) => {
                            macro_keys.push(self.string_to_static_str(org_key.to_string()));
                        },
                        None => ()
                    }
                }
            } else {
                macro_keys.push(self.string_to_static_str(m_key.to_string()));
            }
        }
    }

    fn string_to_static_str(&self, s: String) -> &'static str {
        Box::leak(s.into_boxed_str())
    }

    fn update(&mut self) {
        let endpoint_url = vec_of_url![
            self.config.get("hub_url").unwrap().to_string(),
            self.hub_list_uri.to_string()
        ];

        info!(
            self.logger,
            "updating macro list from hub url: `{}`", endpoint_url
        );

        let mut response = reqwest::get(&endpoint_url).unwrap_or_panic("Cannot fetch macro list.");

        match response.status() {
            reqwest::StatusCode::OK => {
                info!(self.logger, "updated success");
                let body = response.text().unwrap();
                let docs = YamlLoader::load_from_str(&body).unwrap();
                let doc = docs[0].as_hash().unwrap();
                for item in doc.iter() {
                    let (macro_key, macro_detail) = item;
                    let macro_content = macro_detail["content"].as_str().unwrap();
                    self.macros_hub
                        .entry(macro_key.as_str().unwrap().to_string())
                        .or_insert(String::from(macro_content));
                }
            }
            others => error!(self.logger, "{}", others),
        };
    }
}
