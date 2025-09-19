use std::borrow::Cow;

use super::*;

mod accessreview;
mod componentstatus;
mod configmap;
mod created;
mod feature;
mod namespace;
mod node;
mod objectlist;
mod pod;
mod service;
mod subjectreview;

trait StripManagedFields<'a>: Clone + 'a {
    fn maybe_strip_managed_fields(&'a self, params: &ShowParams) -> Cow<'a, Self>;
}

impl<'a, K> StripManagedFields<'a> for K
where
    K: Clone + kube::ResourceExt + 'a,
{
    fn maybe_strip_managed_fields(&'a self, params: &ShowParams) -> Cow<'a, K> {
        if params.show_managed_fields {
            Cow::Borrowed(self)
        } else {
            let mut object = self.clone();
            object.meta_mut().managed_fields = None;
            Cow::Owned(object)
        }
    }
}
