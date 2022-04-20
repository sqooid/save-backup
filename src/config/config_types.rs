use core::fmt;
use std::{
    fs::{self, ReadDir},
    path::PathBuf,
};

use crate::utils::log::LogExpectResult;

#[derive(Debug, PartialEq)]
pub struct SharedConfig {
    pub save_root: PathBuf,
    pub zip: bool,
    pub count: u64,
    pub interval: i64,
}

impl fmt::Display for SharedConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
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

impl fmt::Display for GameConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
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
    pub root: PathBuf,
    include: Option<Vec<PathBuf>>,
    exclude: Option<Vec<PathBuf>>,
}

impl FileList {
    pub fn new(root: &str, include: Option<Vec<String>>, exclude: Option<Vec<String>>) -> Self {
        let root_path = PathBuf::from(root);
        let exclude = if let Some(exclude) = exclude {
            Some(exclude.iter().map(|x| root_path.join(x)).collect())
        } else {
            None
        };
        let include = if let Some(include) = include {
            Some(include.iter().map(|x| PathBuf::from(x)).collect())
        } else {
            None
        };
        Self {
            root: PathBuf::from(root),
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
            dir_iterators: if self.include.is_none() {
                vec![fs::read_dir(&self.root).unwrap()]
            } else {
                vec![]
            },
            include_files: vec![],
            initialized: false,
        }
    }
}

pub struct FileListIterator<'a> {
    file_list: &'a FileList,
    dir_iterators: Vec<ReadDir>,
    include_files: Vec<PathBuf>,
    initialized: bool,
}

impl<'a> Iterator for FileListIterator<'a> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<Self::Item> {
        // Add include directory iterators if present
        if !self.initialized {
            if let Some(include_list) = &self.file_list.include {
                for path in include_list {
                    let path = self.file_list.root.join(path);
                    if path.is_dir() {
                        let iterator = path
                            .read_dir()
                            .log_expect(format!("Failed to open directory {:?}", &path));
                        self.dir_iterators.push(iterator);
                    } else {
                        self.include_files.push(path);
                    }
                }
            }
            self.initialized = true;
        }
        loop {
            if let Some(mut read_dir) = self.dir_iterators.pop() {
                let next = read_dir.next();

                if let Some(entry) = next {
                    if let Ok(dir_entry) = entry {
                        let path = dir_entry.path();
                        let exclude = &self.file_list.exclude;

                        // Check if excluded
                        if let Some(exclude) = exclude {
                            if exclude.contains(&path) {
                                self.dir_iterators.push(read_dir);
                                continue;
                            }
                        }

                        // Add to iterator queue if directory
                        if path.is_dir() {
                            let nested_iter = fs::read_dir(&path);

                            if let Ok(nested_iter) = nested_iter {
                                self.dir_iterators.push(read_dir);
                                self.dir_iterators.push(nested_iter);
                            }
                        } else {
                            let path = dir_entry.path();
                            self.dir_iterators.push(read_dir);
                            return Some(path);
                        }
                    }
                } else {
                    continue;
                }
            } else {
                if self.file_list.include.is_none() {
                    return None;
                }
                return self.include_files.pop();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::FileList;

    #[test]
    fn test_file_list_only_root() {
        let file_list = FileList::new("./test/test_list", None, None);
        let mut files: Vec<PathBuf> = file_list.into_iter().collect();
        let mut expected = vec![
            PathBuf::from("./test/test_list/file1"),
            PathBuf::from("./test/test_list/folder3/file4"),
            PathBuf::from("./test/test_list/folder1/file2"),
            PathBuf::from("./test/test_list/folder1/folder2/file3"),
        ];
        files.sort();
        expected.sort();
        assert_eq!(files, expected)
    }

    #[test]
    fn test_file_list_excludes() {
        let exclude = vec!["folder3".to_owned()];
        let file_list = FileList::new("./test/test_list", None, Some(exclude));
        let mut files: Vec<PathBuf> = file_list.into_iter().collect();
        let mut expected = vec![
            PathBuf::from("./test/test_list/file1"),
            PathBuf::from("./test/test_list/folder1/file2"),
            PathBuf::from("./test/test_list/folder1/folder2/file3"),
        ];
        files.sort();
        expected.sort();
        assert_eq!(files, expected)
    }
    #[test]
    fn test_wtf() {
        let path1 = PathBuf::from("/");
        let path2 = PathBuf::from("\\");

        assert_eq!(path1, path2)
    }

    #[test]
    fn test_file_list_only_root_twice() {
        let file_list = FileList::new("./test/test_list", None, None);
        let files: Vec<PathBuf> = file_list.into_iter().collect();
        let files2: Vec<PathBuf> = file_list.into_iter().collect();
        assert_eq!(files, files2);
    }

    #[test]
    fn test_file_list_list() {
        let include = vec!["folder1".to_string(), "file1".to_string()];
        let exclude = vec!["folder1/folder2".to_string()];
        let file_list = FileList::new("./test/test_list", Some(include), Some(exclude));
        let mut files: Vec<PathBuf> = file_list.into_iter().collect();
        let mut expected = vec![
            PathBuf::from("./test/test_list/file1"),
            PathBuf::from("./test/test_list/folder1/file2"),
        ];
        files.sort();
        expected.sort();
        assert_eq!(files, expected)
    }
}
