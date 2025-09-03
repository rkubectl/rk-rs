use super::*;

#[derive(Debug)]
pub struct Context {
    kubeapi: Kubeapi,
    ui: Ui,
}

impl Context {
    pub fn new(kubeapi: Kubeapi, ui: Ui) -> Self {
        Self { kubeapi, ui }
    }

    pub fn kubeapi(&self) -> &Kubeapi {
        &self.kubeapi
    }

    pub fn ui(&self) -> &Ui {
        &self.ui
    }

    pub fn output_deprecated(&self) -> &OutputFormat {
        self.ui.output_deprecated()
    }
}
