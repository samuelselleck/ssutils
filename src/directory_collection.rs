use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    fmt::Display,
    fs::{File, OpenOptions},
    io,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectoryCollection {
    path: PathBuf,
    entries: HashMap<String, PathBuf>,
}

impl DirectoryCollection {
    pub fn try_load(path: &Path) -> Result<Self> {
        let entries = if !path.exists() {
            Default::default()
        } else {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);
            serde_json::from_reader(reader)?
        };
        Ok(Self {
            path: path.into(),
            entries,
        })
    }

    pub fn try_save(&self) -> Result<()> {
        if !self.path.exists() {
            std::fs::create_dir_all(self.path.parent().ok_or(anyhow!("no parent"))?)?;
        }
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)?;
        let writer = io::BufWriter::new(file);
        Ok(serde_json::to_writer_pretty(writer, &self.entries)?)
    }

    pub fn insert(&mut self, name: String, path: PathBuf) -> Result<()> {
        self.entries.insert(name, std::fs::canonicalize(path)?);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&PathBuf> {
        self.entries.get(name)
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl Display for DirectoryCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_set: HashMap<_, _> = self
            .entries
            .iter()
            .map(|(k, v)| (k.as_str(), v.to_str().unwrap_or("not utf-8")))
            .collect();

        fn col_len<'a>(itr: impl Iterator<Item = &'a &'a str>) -> usize {
            let len = itr.map(|v| v.len()).max().unwrap_or(0);
            std::cmp::max(len, 15)
        }

        let k_len = col_len(display_set.keys());
        let v_len = col_len(display_set.values());
        // Print the HashMap as a table
        writeln!(f, " {:<k$} {:<v$} ", "Name", "Dir", k = k_len, v = v_len)?;
        for (key, value) in display_set.iter() {
            writeln!(f, " {:<k$} {:<v$} ", key, value, k = k_len, v = v_len)?;
        }
        Ok(())
    }
}
