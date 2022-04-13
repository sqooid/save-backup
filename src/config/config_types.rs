use std::{
    fs::{self, ReadDir},
    path::PathBuf,
};

#[derive(Debug, PartialEq)]
pub struct SharedConfig {
    pub save_root: PathBuf,
    pub zip: bool,
    pub count: i64,
    pub interval: i64,
}

impl SharedConfig {
    pub fn new(
        save_root: Option<&str>,
        zip: Option<bool>,
        count: Option<i64>,
        interval: Option<i64>,
    ) -> SharedConfig {
        SharedConfig {
            save_root: PathBuf::from(save_root.unwrap_or("~/Documents/save-backups")),
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
    pub root: PathBuf,
    pub files: Option<Vec<String>>,
    pub interval: i64,
    pub count: i64,
}

impl GameConfig {
    pub fn with_defaults(
        name: &str,
        save_dir: Option<&str>,
        zip: Option<bool>,
        root: &str,
        files: Option<Vec<String>>,
        interval: Option<i64>,
        count: Option<i64>,
        defaults: &SharedConfig,
    ) -> GameConfig {
        let mut save_dir = PathBuf::from(save_dir.unwrap_or(defaults.save_root.to_str().unwrap()));
        save_dir.push(name);

        GameConfig {
            name: name.to_owned(),
            save_dir,
            zip: zip.unwrap_or(defaults.zip),
            root: PathBuf::from(root),
            files,
            interval: interval.unwrap_or(defaults.interval),
            count: count.unwrap_or(defaults.count),
        }
    }
}

pub struct FileList {
    root: String,
    files: Option<Vec<String>>,
}

impl FileList {
    pub fn new(root: &str, files: Option<Vec<String>>) -> Self {
        Self {
            root: root.to_owned(),
            files,
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
            dir_iterator: None,
        }
    }
}

pub struct FileListIterator<'a> {
    dir_iterator: Option<ReadDir>,
    file_index: usize,
    file_list: &'a FileList,
}

impl<'a> Iterator for FileListIterator<'a> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<Self::Item> {
        if self.file_list.files.is_none() {
            if self.dir_iterator.is_none() {
                self.dir_iterator = fs::read_dir(&self.file_list.root).ok();
            }
            let iterator = self.dir_iterator.as_mut().unwrap();
            let file = iterator.next();
            match file {
                Some(x) => match x {
                    Ok(x) => Some(x.path()),
                    _ => None,
                },
                _ => None,
            }
        } else {
            let files = self.file_list.files.as_ref().unwrap();
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
        let file_list = FileList::new("./test/test_latest/file", None);
        let files: Vec<PathBuf> = file_list.into_iter().collect();
        println!("{:?}", files);
        assert_eq!(files.len(), 3);
    }

    #[test]
    fn test_file_list_only_root_twice() {
        let file_list = FileList::new("./test/test_latest/file", None);
        let files: Vec<PathBuf> = file_list.into_iter().collect();
        let files2: Vec<PathBuf> = file_list.into_iter().collect();
        println!("{:?}", files2);
        assert_eq!(files, files2);
    }

    #[test]
    fn test_file_list_list() {
        let file_list = FileList::new(
            "./test/test_latest/file",
            Some(vec!["file-1.txt".to_string()]),
        );
        let files: Vec<PathBuf> = file_list.into_iter().collect();
        println!("{:?}", files);
        assert_eq!(files.len(), 1);
    }
}
