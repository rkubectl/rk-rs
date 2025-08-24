use super::*;

// Specify the path to a file to read lines of key=val pairs to create a secret.
#[derive(Clone, Debug)]
pub struct EnvFile(PathBuf);

impl EnvFile {
    pub fn value_parser() -> impl TypedValueParser {
        PathBufValueParser::new().map(Self)
    }

    pub fn load(&self) -> io::Result<Vec<KeyValue<String>>> {
        let text = fs::read_to_string(&self.0)?;
        KeyValue::from_lines(&text).map_err(io::Error::other)
    }
}

impl From<PathBuf> for EnvFile {
    fn from(value: PathBuf) -> Self {
        Self(value)
    }
}
