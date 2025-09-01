use prometheus_parse::Sample;
use prometheus_parse::Scrape;
use prometheus_parse::Value;

use super::*;

const KUBERNETES_FEATURES: &str = "kubernetes_feature_enabled";

impl Kubectl {
    pub async fn features(&self) -> kube::Result<()> {
        let features = self
            .metrics()
            .await?
            .samples
            .iter()
            .filter(|sample| sample.metric == KUBERNETES_FEATURES)
            .filter_map(Feature::from_sample)
            .collect::<Vec<_>>();

        let show_params = default();
        println!("{}", features.output(false, &show_params, self.output()));
        Ok(())
    }

    pub async fn metrics(&self) -> kube::Result<Scrape> {
        let text = self.raw("metrics").await?;
        let lines = text.lines().map(String::from).map(Ok);
        Scrape::parse(lines).map_err(kube::Error::ReadEvents)
    }
}

#[derive(Debug, serde::Serialize, tabled::Tabled)]
#[tabled(rename_all = "UPPERCASE")]
pub struct Feature {
    pub name: String,
    pub stage: String,
    pub enabled: bool,
}

impl Feature {
    fn from_sample(sample: &Sample) -> Option<Self> {
        let name = sample.labels.get("name")?.to_string();
        let stage = sample.labels.get("stage")?.to_string();
        let enabled = sample.value == Value::Gauge(1.0);
        Some(Self {
            name,
            stage,
            enabled,
        })
    }
}
