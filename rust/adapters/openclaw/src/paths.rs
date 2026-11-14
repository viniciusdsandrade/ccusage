use std::{
    collections::HashSet,
    env,
    path::{Path, PathBuf},
};

use ccusage_adapter_common::collect_files_matching;

const OPENCLAW_DIR_ENV: &str = "OPENCLAW_DIR";

pub(super) fn paths(custom_path: Option<&str>) -> Vec<PathBuf> {
    if let Some(custom_path) = custom_path.filter(|path| !path.trim().is_empty()) {
        return existing_path_list(custom_path);
    }
    if let Ok(env_paths) = env::var(OPENCLAW_DIR_ENV)
        && !env_paths.trim().is_empty()
    {
        return existing_path_list(&env_paths);
    }
    let Some(home) = crate::home::home_dir() else {
        return Vec::new();
    };
    [
        home.join(".openclaw"),
        home.join(".clawdbot"),
        home.join(".moltbot"),
        home.join(".moldbot"),
    ]
    .into_iter()
    .filter(|path| path.is_dir())
    .collect()
}

fn existing_path_list(raw: &str) -> Vec<PathBuf> {
    let mut seen = HashSet::new();
    raw.split(',')
        .map(str::trim)
        .filter(|path| !path.is_empty())
        .map(PathBuf::from)
        .filter(|path| path.is_dir() && seen.insert(path.clone()))
        .collect()
}

/// Collects the session files under `root`. A directory that cannot be read is
/// skipped, as it is for every other source, so it cannot fail the report.
pub(super) fn collect_session_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_files_matching(
        root,
        &|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(is_openclaw_session_file)
        },
        &mut files,
    );
    files.sort();
    files
}

fn is_openclaw_session_file(name: &str) -> bool {
    let Some(index) = name.find(".jsonl") else {
        return false;
    };
    let suffix = &name[index..];
    suffix == ".jsonl"
        || suffix.starts_with(".jsonl.deleted.")
        || suffix.starts_with(".jsonl.reset.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_archived_session_files_as_openclaw_sessions() {
        assert!(is_openclaw_session_file("a.jsonl.deleted.1700000000000"));
        assert!(is_openclaw_session_file(
            "a.jsonl.reset.2026-03-20T06-34-44.520Z"
        ));
        assert!(!is_openclaw_session_file("a.json"));
    }
}
