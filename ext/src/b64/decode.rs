use super::*;

pub trait Base64Decode<Output = String> {
    fn decode(self) -> Result<Output, base64::DecodeError>;
}

impl Base64Decode for ByteString {
    fn decode(self) -> Result<String, base64::DecodeError> {
        let decoded = BASE64_STANDARD.decode(&self.0)?;
        Ok(String::from_utf8_lossy(&decoded).to_string())
    }
}

impl Base64Decode<Vec<u8>> for ByteString {
    fn decode(self) -> Result<Vec<u8>, base64::DecodeError> {
        BASE64_STANDARD.decode(&self.0)
    }
}
