use std::{
    collections::HashMap,
    fs::File,
    io::{Read, self},
    process::{Command, ExitStatus},
};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

use skim::{
    prelude::{SkimItemReader, SkimOptionsBuilder},
    Skim,
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Snippet {
    /// the task tag of the command.
    tag: Vec<String>,

    /// the description of the command, which is used to lookup what you want to search.
    desc: String,

    /// the actual command.
    pub cmd: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Snippets {
    snippets: Vec<Snippet>,
}

pub fn load_snippets(path: String) -> Result<Snippets> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let snippets = serde_yaml::from_str(&buf)?;
    Ok(Snippets { snippets })
}

pub struct SnippetsSearcher {
    snippets: HashMap<String, Snippet>,
    input_sequence: String,
}

impl SnippetsSearcher {
    pub fn new(snippets: Snippets) -> Self {
        let mut map_from_desc = HashMap::new();
        let mut input_sequence = String::new();

        for snippet in snippets.snippets.into_iter() {
            let desc = snippet.desc.clone();
            map_from_desc.insert(desc.clone(), snippet);
            input_sequence += format!("{}\n", desc.clone()).as_str();
        }

        let input_sequence = input_sequence.strip_suffix("\n").unwrap().to_string();

        Self {
            snippets: map_from_desc,
            input_sequence,
        }
    }

    pub fn search_blocking(&self) -> Vec<&Snippet> {
        let options = SkimOptionsBuilder::default()
            .height(Some("50%"))
            .multi(true)
            .build()
            .unwrap();
        let item_reader = SkimItemReader::default();
        let items = item_reader.of_bufread(Cursor::new(self.input_sequence.clone()));

        let selected_items = Skim::run_with(&options, Some(items))
            .map(|out| out.selected_items)
            .unwrap_or_else(|| Vec::new());
        let mut result_snippets = Vec::new();

        for selected_item in selected_items.iter() {
            let selected_key =
                String::from_utf8(selected_item.output().as_bytes().to_vec()).unwrap();

            if !self.snippets.contains_key(&selected_key) {
                continue;
            }

            result_snippets.push(self.snippets.get(&selected_key).unwrap());
        }

        return result_snippets;
    }
}

pub fn edit_snippets(path: String) -> io::Result<ExitStatus> {
    Command::new("vim")
        .arg(path).spawn()
        .expect("failed to open editor").wait()
}
