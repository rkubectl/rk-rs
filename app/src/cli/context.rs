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

    pub fn show<T>(&self, item: T)
    where
        T: Show,
    {
        self.ui.show(item);
    }

    pub fn print_deprecated(&self, text: impl fmt::Display) {
        self.ui.print(text);
    }

    pub fn not_implemented(&self, item: impl fmt::Debug) {
        self.ui.not_implemented(item);
    }

    fn _ui(&self) -> &Ui {
        &self.ui
    }

    pub fn output_deprecated(&self) -> OutputFormat {
        self.ui.output_deprecated()
    }
}
