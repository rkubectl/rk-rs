use super::*;

mod configmap;
mod node;
mod objectlist;
mod pod;

trait StripManagedFields {
    fn strip_managed_fields(&mut self, params: &ShowParams);
}

impl<K> StripManagedFields for K
where
    K: Clone + kube::ResourceExt,
{
    fn strip_managed_fields(&mut self, params: &ShowParams) {
        if !params.show_managed_fields {
            self.meta_mut().managed_fields = None;
        }
    }
}
