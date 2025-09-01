use super::*;

pub trait NamespaceGetExt2 {
    fn get_status(&self) -> &str;
}

impl NamespaceGetExt2 for corev1::Namespace {
    fn get_status(&self) -> &str {
        self.phase().unwrap_or("Unknown")
    }
}
