use super::*;

pub trait ComponentStatusGetExt2 {
    fn conditions(&self) -> Option<&[corev1::ComponentCondition]>;

    fn condition(&self, r#type: &str) -> Option<&corev1::ComponentCondition> {
        self.conditions()?
            .iter()
            .find(|condition| condition.type_ == r#type)
    }

    fn healthy(&self) -> Option<&corev1::ComponentCondition> {
        self.condition(COMPONENT_CONDITION_HEALTHY)
    }
}

impl ComponentStatusGetExt2 for corev1::ComponentStatus {
    fn conditions(&self) -> Option<&[corev1::ComponentCondition]> {
        self.conditions.as_deref()
    }
}

pub trait ComponentConditionGetExt2 {
    fn status(&self) -> &str;
    fn r#type(&self) -> &str;
    fn message(&self) -> Option<&str>;
    fn error(&self) -> Option<&str>;
    fn is_true(&self) -> bool;
}

impl ComponentConditionGetExt2 for corev1::ComponentCondition {
    fn status(&self) -> &str {
        &self.status
    }

    fn r#type(&self) -> &str {
        &self.type_
    }

    fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    fn is_true(&self) -> bool {
        self.status == "True"
    }
}
