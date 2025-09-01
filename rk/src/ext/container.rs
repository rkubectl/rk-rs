use super::*;

pub trait ContainerGetExt2 {
    fn is_restartable(&self) -> bool;
}

impl ContainerGetExt2 for corev1::Container {
    fn is_restartable(&self) -> bool {
        self.restart_policy.as_deref() == Some("Always")
    }
}

pub trait ContainerStatusGetExt2 {
    fn terminated(&self) -> Option<&corev1::ContainerStateTerminated>;
    fn waiting(&self) -> Option<&corev1::ContainerStateWaiting>;

    fn terminated_reason(&self) -> Option<String> {
        self.terminated().map(|terminated| terminated.reason())
    }

    fn waiting_reason(&self) -> Option<String> {
        self.waiting()?
            .reason
            .as_ref()
            .filter(|reason| *reason != POD_INITIALIZING)
            .cloned()
    }
}

impl ContainerStatusGetExt2 for corev1::ContainerStatus {
    fn terminated(&self) -> Option<&corev1::ContainerStateTerminated> {
        self.state()?.terminated.as_ref()
    }

    fn waiting(&self) -> Option<&corev1::ContainerStateWaiting> {
        self.state()?.waiting.as_ref()
    }
}

trait TerminatedExt {
    fn reason(&self) -> String;
}

impl TerminatedExt for corev1::ContainerStateTerminated {
    fn reason(&self) -> String {
        self.reason.clone().unwrap_or_else(|| {
            self.signal.map_or_else(
                || format!("ExitCode:{}", self.exit_code),
                |signal| format!("Signal:{signal}"),
            )
        })
    }
}
