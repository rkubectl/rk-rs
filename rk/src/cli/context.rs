use super::*;

#[derive(Debug)]
pub struct Context {
    kubectl: Kubectl,
    ui: Ui,
}

impl Context {
    pub fn new(kubectl: Kubectl, ui: Ui) -> Self {
        Self { kubectl, ui }
    }

    pub fn kubectl(&self) -> &Kubectl {
        &self.kubectl
    }

    pub fn ui(&self) -> &Ui {
        &self.ui
    }

    pub fn output_deprecated(&self) -> &OutputFormat {
        self.ui.output_deprecated()
    }
}
