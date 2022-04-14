use std::path::PathBuf;

pub fn normalize_paths(paths: Vec<String>) -> Vec<PathBuf> {
    paths
        .iter()
        .map(|x| PathBuf::from(str::replace(x, r"\", "/")))
        .collect()
}
