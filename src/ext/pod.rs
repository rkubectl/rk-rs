use super::*;

pub trait PodGetExt2 {
    fn pod_scheduled_reason(&self) -> Option<&str>;
    fn ready_containers(&self) -> usize;
    fn total_containers(&self) -> usize;
    fn restarts(&self) -> i32;
    fn kubectl_status(&self) -> String;
    fn init_container_kubectl_status(&self) -> Option<String>;
}

impl PodGetExt2 for corev1::Pod {
    fn pod_scheduled_reason(&self) -> Option<&str> {
        self.condition(POD_SCHEDULED_CONDITION)?.reason()
    }

    fn ready_containers(&self) -> usize {
        self.container_statuses()
            .unwrap_or_default()
            .iter()
            .filter(|status| status.ready)
            .count()
    }

    fn total_containers(&self) -> usize {
        let containers = self.containers().unwrap_or_default().len();
        let init_containers = self
            .init_containers()
            .unwrap_or_default()
            .iter()
            .filter(|container| container.is_restartable())
            .count();
        containers + init_containers
    }
    fn restarts(&self) -> i32 {
        self.container_statuses()
            .unwrap_or_default()
            .iter()
            .map(|status| status.restart_count)
            .sum()
    }

    // mimics `kubectl get pod` `STATUS`
    fn kubectl_status(&self) -> String {
        let reason = self.phase().unwrap_or_default();
        let reason = self.reason().unwrap_or(reason);
        let reason = self
            .pod_scheduled_reason()
            .filter(|reason| *reason == POD_REASON_SCHEDULING_GATED)
            .unwrap_or(reason);
        self.init_container_kubectl_status()
            .map_or_else(|| reason.to_string(), |reason| format!("Init:{reason}"))
    }

    fn init_container_kubectl_status(&self) -> Option<String> {
        let init_containers = self.init_containers().unwrap_or_default();
        let total_init_containers = init_containers.len();
        self.init_container_statuses()
            .unwrap_or_default()
            .iter()
            .enumerate()
            // Check only abnormally terminated init containers
            .filter(|(_, status)| {
                status
                    .terminated()
                    .is_none_or(|terminated| terminated.exit_code != 0)
            })
            // Ignore restartable init containers (sidecars)
            .filter(|(_, status)| {
                init_containers
                    .iter()
                    .any(|container| container.name == status.name && container.is_restartable())
            })
            .map(|(idx, status)| {
                status
                    // Take the termination reason if any
                    .terminated_reason()
                    // Or wating reason
                    .or_else(|| status.waiting_reason())
                    // Or just its position in the list
                    .unwrap_or_else(|| format!("{idx}/{total_init_containers}"))
            })
            // Show the last one
            .next_back()
    }
}
