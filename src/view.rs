use std::{ env, io::Cursor, process::Command };
use skim::prelude::{ Skim, SkimItemReader, SkimOptionsBuilder };

use crate::model::SharedData;

pub fn list_contexts() {
    if env::var("VAULTCTX_IGNORE_FZF").unwrap_or_default() == "1" {
        choose_context();
    } else {
        choose_context_interactive();
    }
}

fn choose_context() {
    let data = SharedData::new();

    for config in &data.configs {
        println!("{}", config.name);
    }
}

fn choose_context_interactive() {
    let data = SharedData::new();

    let names: Vec<String> = data.configs
        .into_iter()
        .map(|config| config.name)
        .collect();
    let names_str = names.join("\n");

    let options = SkimOptionsBuilder::default().height(Some("50%")).multi(false).build().unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(names_str));

    if let Some(output) = Skim::run_with(&options, Some(items)) {
        if !output.is_abort {
            let selected_items = output.selected_items;
            let selected_entry = selected_items.get(0).unwrap().output();

            let current_exe = env::current_exe().expect("Failed to get current executable");

            let status = Command::new(current_exe)
                .arg(selected_entry.as_ref())
                .status()
                .expect("Failed to execute command");

            if status.success() {
                println!("Command executed successfully.");
            } else {
                eprintln!("Command failed to execute.");
            }
        } else {
            println!("No entry selected");
        }
    } else {
        println!("Skim failed to run");
    }
}
