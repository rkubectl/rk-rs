use base64::prelude::*;
use k8s::openapi::ByteString;

pub use decode::Base64Decode;
pub use encode::Base64Encode;

use super::*;

mod decode;
mod encode;

// pub fn base64_encoded(&self) -> KeyValue<k8s::openapi::ByteString> {
//     let Self { key, value } = self;
//     let key = key.clone();
//     let value = BASE64_STANDARD.encode(value).into_bytes();
//     let value = k8s::ByteString(value);
//     KeyValue { key, value }
// }
