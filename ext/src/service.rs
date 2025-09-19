use super::*;

pub trait ServiceGetExt2 {
    fn spec(&self) -> Option<&corev1::ServiceSpec>;
    fn status(&self) -> Option<&corev1::ServiceStatus>;
    fn ingress(&self) -> Option<&corev1::LoadBalancerIngress> {
        self.status()?
            .load_balancer
            .as_ref()?
            .ingress
            .as_ref()?
            .first()
    }
}

impl ServiceGetExt2 for corev1::Service {
    fn spec(&self) -> Option<&corev1::ServiceSpec> {
        self.spec.as_ref()
    }

    fn status(&self) -> Option<&corev1::ServiceStatus> {
        self.status.as_ref()
    }
}
