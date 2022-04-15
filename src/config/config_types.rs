use std::{
    fs::{self, ReadDir},
    path::PathBuf,
};

use crate::utils::path::normalize_paths;

#[derive(Debug, PartialEq)]
pub struct SharedConfig {
    pub save_root: PathBuf,
    pub zip: bool,
    pub count: u64,
    pub interval: i64,
}

impl SharedConfig {
    pub fn new(
        save_root: Option<&str>,
        zip: Option<bool>,
        count: Option<u64>,
        interval: Option<i64>,
    ) -> SharedConfig {
        SharedConfig {
            save_root: PathBuf::from(save_root.unwrap_or("./save-backups")),
            zip: zip.unwrap_or(true),
            count: count.unwrap_or(5),
            interval: interval.unwrap_or(30),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct GameConfig {
    pub name: String,
    pub save_dir: PathBuf,
    pub zip: bool,
    pub file_list: FileList,
    pub interval: i64,
    pub count: u64,
}

impl GameConfig {
    pub fn with_defaults(
        name: &str,
        save_dir: Option<&str>,
        zip: Option<bool>,
        root: &str,
        include: Option<Vec<String>>,
        exclude: Option<Vec<String>>,
        interval: Option<i64>,
        count: Option<u64>,
        defaults: &SharedConfig,
    ) -> GameConfig {
        let mut save_dir = PathBuf::from(save_dir.unwrap_or(defaults.save_root.to_str().unwrap()));
        save_dir.push(name);

        GameConfig {
            name: name.to_owned(),
            save_dir,
            zip: zip.unwrap_or(defaults.zip),
            file_list: FileList::new(root, include, exclude),
            interval: interval.unwrap_or(defaults.interval),
            count: count.unwrap_or(defaults.count),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FileList {
    pub root: String,
    include: Option<Vec<PathBuf>>,
    exclude: Option<Vec<PathBuf>>,
}

impl FileList {
    pub fn new(root: &str, include: Option<Vec<String>>, exclude: Option<Vec<String>>) -> Self {
        let exclude = if let Some(exclude) = exclude {
            Some(normalize_paths(exclude))
        } else {
            None
        };
        let include = if let Some(include) = include {
            Some(normalize_paths(include))
        } else {
            None
        };
        Self {
            root: root.to_owned(),
            include,
            exclude,
        }
    }
}

impl<'a> IntoIterator for &'a FileList {
    type Item = PathBuf;
    type IntoIter = FileListIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        FileListIterator {
            file_list: &self,
            file_index: 0,
            dir_iterators: vec![fs::read_dir(&self.root).unwrap()],
        }
    }
}

pub struct FileListIterator<'a> {
    file_list: &'a FileList,
    dir_iterators: Vec<ReadDir>,
    file_index: usize,
}

impl<'a> Iterator for FileListIterator<'a> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<Self::Item> {
        if self.file_list.include.is_none() {
            loop {
                if let Some(read_dir) = self.dir_iterators.last_mut() {
                    let next = read_dir.next();

                    if let Some(entry) = next {
                        if let Ok(dir_entry) = entry {
                            let path = dir_entry.path();
                            let exclude = &self.file_list.exclude;

                            // Check if excluded
                            if let Some(exclude) = exclude {
                                if exclude.contains(&path) {
                                    continue;
                                }
                            }
                            if path.is_dir() {
                                let nested_iter = fs::read_dir(path);

                                if let Ok(nested_iter) = nested_iter {
                                    self.dir_iterators.push(nested_iter);
                                }
                            } else {
                                let path = dir_entry.path();
                                return Some(path);
                            }
                        }
                    } else {
                        self.dir_iterators.pop();
                    }
                } else {
                    return None;
                }
            }
        } else {
            let files = self.file_list.include.as_ref().unwrap();
            if self.file_index >= files.len() {
                return None;
            }
            let root_path = PathBuf::from(&self.file_list.root);
            let file_path = root_path.as_path().join(&files[self.file_index]);
            self.file_index += 1;
            Some(file_path)
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::FileList;

    #[test]
    fn test_file_list_only_root() {
        let file_list = FileList::new("./test/test_latest/file", None, None);
        let files: Vec<PathBuf> = file_list.into_iter().collect();
        println!("{:?}", files);
        assert_eq!(files.len(), 6);
    }

    #[test]
    fn test_file_list_excludes() {
        let exclude = vec!["./test/test_latest/file/nested/more_nested".to_owned()];
        let file_list = FileList::new("./test/test_latest/file", None, Some(exclude));
        let files: Vec<PathBuf> = file_list.into_iter().collect();
        println!("{:?}", files);
        assert_eq!(files.len(), 5);
    }

    #[test]
    fn test_file_list_only_root_twice() {
        let file_list = FileList::new("./test/test_latest/file", None, None);
        let files: Vec<PathBuf> = file_list.into_iter().collect();
        let files2: Vec<PathBuf> = file_list.into_iter().collect();
        assert_eq!(files, files2);
    }

    #[test]
    fn test_file_list_list() {
        let file_list = FileList::new(
            "./test/test_latest/file",
            Some(vec!["file-1.txt".to_string()]),
            None,
        );
        let files: Vec<PathBuf> = file_list.into_iter().collect();
        assert_eq!(files.len(), 1);
    }
}
