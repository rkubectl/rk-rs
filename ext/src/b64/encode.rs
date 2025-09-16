use super::*;

pub trait Base64Encode<Output = ByteString> {
    fn encode(self) -> Output;
}

impl Base64Encode for &[u8] {
    fn encode(self) -> ByteString {
        let encoded = BASE64_STANDARD.encode(self);
        ByteString(encoded.into_bytes())
    }
}

impl Base64Encode for &str {
    fn encode(self) -> ByteString {
        self.as_bytes().encode()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn _encoded<T>(value: T) -> ByteString
    where
        T: Base64Encode,
    {
        value.encode()
    }

    #[expect(clippy::useless_vec)]
    #[test]
    fn vec() {
        let vec = vec![1, 2, 3];
        let encoded = vec.encode();
        assert_eq!(encoded.0, b"AQID");
    }
}
