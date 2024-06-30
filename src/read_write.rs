use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::path::{Path, PathBuf};
use std::fs;
use itertools::Itertools;
use zip::{ZipArchive, ZipWriter, write::SimpleFileOptions};

fn format_path(path: &Path) -> String {
    path.iter().map(|x| x.to_str().unwrap()).join("/")
}

pub trait PathStuff: Sized {
    fn path(&self) -> &Path;
    fn with_path<P: AsRef<Path>>(&self, path: P) -> Self; 

    fn name(&self) -> &str {
        self.path().file_stem().unwrap().to_str().unwrap()
    }

    fn with_file_name<S: AsRef<std::ffi::OsStr>>(&self, file_name: S) -> Self {
        self.with_path(self.path().with_file_name(file_name))
    }

    fn join<P: AsRef<Path>>(&self, path: P) -> Self {
        self.with_path(self.path().join(path))
    }

    fn with_extension<S: AsRef<std::ffi::OsStr>>(&self, ext: S) -> Self {
        self.with_path(self.path().with_extension(ext))
    }
}

pub enum Reader {
    File(PathBuf),
    Zip(Arc<Mutex<ZipArchive<fs::File>>>, PathBuf)
}

impl std::fmt::Debug for Reader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(val) => val.fmt(f),
            Self::Zip(_, val) => val.fmt(f)
        }
    }
}

impl PathStuff for Reader {
    fn path(&self) -> &Path {
        match self {
            Self::File(path) => path.as_path(),
            Self::Zip(_, path) => path.as_path(),
        }
    }
    fn with_path<P: AsRef<Path>>(&self, path: P) -> Self {
        match self {
            Self::File(_) => Self::File(path.as_ref().into()),
            Self::Zip(zip, _) => Self::Zip(zip.clone(), path.as_ref().into()),
        }
    }
}

impl Reader {
    pub fn new_zip<P: AsRef<Path>>(path: P) -> Self {
        Self::Zip(
            Arc::new(Mutex::new(ZipArchive::new(fs::File::open(path).unwrap()).unwrap())),
            PathBuf::new()
        )
    }

    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self::File(
            path.as_ref().into()
        )
    }

    pub fn is_file(&self) -> bool {
        match self {
            Self::File(path) => path.is_file(),
            Self::Zip(zip, path) => zip.lock().unwrap().index_for_name(&format_path(path)).is_some()
        }
    }

    pub fn read(&self) -> Vec<u8> {
        match self {
            Self::File(path) => fs::read(path).unwrap(),
            Self::Zip(zip, path) => {
                let mut zip = zip.lock().unwrap();
                let mut file = zip.by_name(&format_path(path)).unwrap();
                let mut out = Vec::with_capacity(file.size() as usize);
                file.read_to_end(&mut out).unwrap();
                out
            }
        }
    }
}

impl IntoIterator for Reader {
    type Item = Self;
    type IntoIter = std::vec::IntoIter<Self>;
    fn into_iter(self) -> Self::IntoIter {
        let out: Vec<_> = match self {
            Self::File(path) => if path.is_dir() {
                fs::read_dir(path)
                    .unwrap()
                    .filter_map(|x| x.ok().map(|x| Self::File(x.path())))
                    .collect()
            } else {
                vec![]
            },
            Self::Zip(zip, path) => zip.lock().unwrap().file_names().filter_map(|x| {
                let child = PathBuf::from(x);
                child.starts_with(&path).then_some(Self::Zip(zip.clone(), child))
            }).collect()
        };
        out.into_iter()
    }
}

pub enum Writer {
    File(PathBuf),
    Zip(Arc<Mutex<ZipWriter<fs::File>>>, PathBuf)
}

impl PathStuff for Writer {
    fn path(&self) -> &Path {
        match self {
            Self::File(path) => path.as_path(),
            Self::Zip(_, path) => path.as_path(),
        }
    }
    fn with_path<P: AsRef<Path>>(&self, path: P) -> Self {
        match self {
            Self::File(_) => Self::File(path.as_ref().into()),
            Self::Zip(zip, _) => Self::Zip(zip.clone(), path.as_ref().into()),
        }
    }
}

impl Writer {
    pub fn new<P: AsRef<Path>>(path: P, zip: bool) -> Self {
        if zip {
            fs::create_dir_all(path.as_ref().parent().unwrap()).unwrap();
            Self::Zip(
                Arc::new(Mutex::new(ZipWriter::new(fs::File::create(path.as_ref().with_extension("zip")).unwrap()))),
                PathBuf::new()
            )
        } else {
            Self::File(
                path.as_ref().into()
            )
        }
    }

    pub fn write(&self, contents: &[u8]) {
        match self {
            Self::File(path) => {
                fs::create_dir_all(path.parent().unwrap()).unwrap();
                fs::write(path, contents).unwrap();
            },
            Self::Zip(zip, path) => {
                let mut zip = zip.lock().unwrap();
                zip.start_file(format_path(path), SimpleFileOptions::default()).unwrap();
                zip.write_all(contents).unwrap();
                zip.flush().unwrap();
            }
        }
    }
}

