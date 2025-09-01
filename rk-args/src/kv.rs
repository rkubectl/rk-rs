use k8s::LabelSelectorExt;

use super::*;

pub use envfile::EnvFile;
pub use file::File;

mod envfile;
mod file;

#[derive(Clone, Debug)]
pub struct KeyValue<T> {
    key: String,
    value: T,
}

impl<T> KeyValue<T> {
    const DELIMITER: &str = "=";

    pub fn as_pair(&self) -> (&String, &T) {
        (&self.key, &self.value)
    }

    pub fn into_pair(self) -> (String, T) {
        (self.key, self.value)
    }
}

impl KeyValue<k8s::ByteString> {}

impl KeyValue<String> {
    pub fn value_parser() -> impl TypedValueParser {
        StringValueParser::new().try_map(Self::from_text)
    }

    pub fn base64_encoded(&self) -> KeyValue<k8s::ByteString> {
        let Self { key, value } = self;
        let key = key.clone();
        let value = BASE64_STANDARD.encode(value).into_bytes();
        let value = k8s::ByteString(value);
        KeyValue { key, value }
    }

    pub fn label_selector(&self) -> metav1::LabelSelector {
        metav1::LabelSelector::new().match_labels([(&self.key, &self.value)])
    }

    fn from_text(text: impl AsRef<str>) -> Result<Self, String> {
        let text = text.as_ref();
        text.split_once(Self::DELIMITER)
            .map(Self::from_parts)
            .ok_or_else(|| format!("Invalid key=value: no '=' in {text}"))
    }

    fn from_lines(text: &str) -> Result<Vec<Self>, String> {
        text.lines()
            .map(Self::from_text)
            .collect::<Result<Vec<_>, _>>()
    }

    fn from_parts((key, value): (&str, &str)) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

impl KeyValue<Vec<u8>> {
    pub fn base64_encoded(self) -> KeyValue<k8s::ByteString> {
        let Self { key, value } = self;
        let value = BASE64_STANDARD.encode(&value).into_bytes();
        let value = k8s::ByteString(value);
        KeyValue { key, value }
    }
}

impl KeyValue<PathBuf> {
    fn from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let value = path.as_ref();
        path_to_key(value)
            .map(|key| Self::new(key, value))
            .ok_or_else(|| io::Error::other("No a valid filename"))
    }

    fn from_direntry(entry: fs::DirEntry) -> io::Result<Option<Self>> {
        if entry.file_type()?.is_file() {
            let value = entry.path();
            Self::from_path(value).map(Some)
        } else {
            Ok(None)
        }
    }

    fn new(key: &str, value: &Path) -> Self {
        let key = key.to_string();
        let value = value.to_path_buf();
        Self { key, value }
    }

    fn load(self) -> io::Result<KeyValue<Vec<u8>>> {
        let Self { key, value } = self;
        fs::read(value).map(|value| KeyValue { key, value })
    }
}

impl From<KeyValue<String>> for KeyValue<PathBuf> {
    fn from(kv: KeyValue<String>) -> Self {
        let KeyValue { key, value } = kv;
        let value = PathBuf::from(value);
        Self { key, value }
    }
}

impl str::FromStr for KeyValue<PathBuf> {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        KeyValue::from_text(text).map(Into::into)
    }
}

fn path_to_key(path: &Path) -> Option<&str> {
    path.file_name()?.to_str()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_string() {
        let kv = KeyValue::from_text("aaa=bbb").unwrap();
        assert_eq!(kv.key, "aaa");
        assert_eq!(kv.value, "bbb");
    }

    #[test]
    fn invalid_string() {
        let err = KeyValue::from_text("aaa-bbb").unwrap_err();
        assert_eq!(err, "Invalid key=value: no '=' in aaa-bbb");
    }

    #[test]
    fn from_lines() {
        let kv = KeyValue::from_lines("aaa=bbb\nccc=ddd\n").unwrap();
        assert_eq!(kv.len(), 2);
        assert_eq!(kv[0].key, "aaa");
        assert_eq!(kv[0].value, "bbb");
        assert_eq!(kv[1].key, "ccc");
        assert_eq!(kv[1].value, "ddd");
    }

    #[test]
    fn from_path() {
        let kv = KeyValue::from_path("config.toml").unwrap();
        assert_eq!(kv.key, "config.toml");
        assert_eq!(kv.value, Path::new("config.toml"));
    }

    #[test]
    fn from_invalid_path() {
        let err = KeyValue::from_path("/").unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::Other);
    }
}
