use super::*;

// Key files can be specified using their file path,
// in which case a default name will be given to them,
// or optionally with a name and file path,
// in which case the given name will be used.
// Specifying a directory will iterate each named file in the directory that is a valid secret key.

#[derive(Clone, Debug)]
pub enum File {
    Directory(PathBuf),
    File(PathBuf),
    KeyedFile(String, PathBuf),
}

impl File {
    pub fn value_parser() -> impl TypedValueParser {
        StringValueParser::new().try_map(Self::from_text)
    }

    pub fn validating_value_parser<F>(validate: F) -> impl TypedValueParser
    where
        F: Fn(Self) -> Result<Self, String> + Clone + Send + Sync + 'static,
    {
        StringValueParser::new()
            .try_map(Self::from_text)
            .try_map(validate)
    }

    pub fn load(&self) -> io::Result<Vec<KeyValue<Vec<u8>>>> {
        match self {
            Self::Directory(path) => load_directory(path),
            Self::File(path) => load_file(path).map(|kv| vec![kv]),
            Self::KeyedFile(key, path) => load_keyed_file(key, path).map(|kv| vec![kv]),
        }
    }

    pub fn key(&self) -> Option<&str> {
        match self {
            Self::KeyedFile(key, _) => Some(key.as_str()),
            _ => None,
        }
    }

    fn from_text(text: String) -> Result<Self, String> {
        match KeyValue::from_text(&text) {
            Ok(kv) => Self::keyed_file(kv),
            Err(_) => Self::file_or_dir(&text),
        }
    }

    fn keyed_file(kv: KeyValue<String>) -> Result<Self, String> {
        let KeyValue::<PathBuf> { key, value } = kv.into();
        if value.is_dir() {
            Err("cannot give a key name for a directory path".to_string())
        } else {
            Ok(Self::KeyedFile(key, value))
        }
    }

    fn file_or_dir(value: &str) -> Result<Self, String> {
        let path = PathBuf::from(value);
        if path.is_dir() {
            Ok(Self::Directory(path))
        } else if path.is_file() {
            Ok(Self::File(path))
        } else {
            Err(format!("{value} is not a valid file or directory"))
        }
    }
}

fn load_file(path: &Path) -> io::Result<KeyValue<Vec<u8>>> {
    KeyValue::from_path(path)?.load()
}

fn load_directory(path: &Path) -> io::Result<Vec<KeyValue<Vec<u8>>>> {
    let mut kv = vec![];
    for entry in fs::read_dir(path)? {
        if let Some(item) = entry.and_then(KeyValue::from_direntry)? {
            let item = item.load()?;
            kv.push(item);
        }
    }
    Ok(kv)
}

fn load_keyed_file(key: &str, path: &Path) -> io::Result<KeyValue<Vec<u8>>> {
    KeyValue::<PathBuf>::new(key, path).load()
}
