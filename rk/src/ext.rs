use super::*;

// use k8s::ContainerGetExt;
use k8s::ContainerStatusGetExt;
use k8s::NamespaceGetExt;
use k8s::PodConditionGetExt;
use k8s::PodGetExt;

pub use component::ComponentConditionGetExt2;
pub use component::ComponentStatusGetExt2;
pub use container::ContainerGetExt2;
pub use container::ContainerStatusGetExt2;
pub use namespace::NamespaceGetExt2;
pub use pod::PodGetExt2;

mod component;
mod container;
mod namespace;
mod pod;

const POD_SCHEDULED_CONDITION: &str = "PodScheduled";
const POD_INITIALIZING: &str = "PodInitializing";
const POD_REASON_SCHEDULING_GATED: &str = "SchedulingGated";

const COMPONENT_CONDITION_HEALTHY: &str = "Healthy";
