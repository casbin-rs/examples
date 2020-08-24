use csv::{Error, ReaderBuilder, StringRecord, Trim};
use walkdir::{DirEntry, WalkDir};

use std::{
    convert::AsRef,
    fs,
    path::{Path, PathBuf},
};

pub fn walk_csv<P: AsRef<Path>>(dir: P) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .follow_links(true)
        .min_depth(1)
        .max_depth(3)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok())
        .filter(|e| is_file(e) && is_csv(e))
        .filter_map(|e| fs::canonicalize(e.path()).ok())
        .collect::<Vec<PathBuf>>()
}

pub fn load_csv<P: AsRef<Path>>(paths: Vec<P>) -> Vec<Vec<String>> {
    paths
        .into_iter()
        .map(load_records)
        .flatten()
        .filter_map(|r| r.deserialize::<Vec<String>>(None).ok())
        .collect::<Vec<Vec<String>>>()
}

fn load_records<P: AsRef<Path>>(p: P) -> Vec<StringRecord> {
    if let Ok(mut rdr) = ReaderBuilder::new()
        .has_headers(false)
        .double_quote(false)
        .comment(Some(b'#'))
        .delimiter(b',')
        .trim(Trim::All)
        .from_path(p)
    {
        if let Ok(records) = rdr.records().collect::<Result<Vec<StringRecord>, Error>>()
        {
            return records
                .into_iter()
                .filter(|r| is_valid_policy(r) || is_valid_grouping_policy(r))
                .collect::<Vec<StringRecord>>();
        }
    }

    vec![]
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn is_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file()
}

fn is_csv(entry: &DirEntry) -> bool {
    entry.path().to_string_lossy().ends_with(".csv")
}

fn is_valid_grouping_policy(record: &StringRecord) -> bool {
    if let Some(ptype) = record.get(0) {
        return ptype.starts_with('g') && record.len() >= 3;
    }

    false
}

fn is_valid_policy(record: &StringRecord) -> bool {
    if let Some(ptype) = record.get(0) {
        return ptype.starts_with('p') && record.len() >= 4;
    }

    false
}
