use prometheus_parse::Sample;
use prometheus_parse::Scrape;
use prometheus_parse::Value;

#[derive(Debug, serde::Serialize, tabled::Tabled)]
#[tabled(rename_all = "UPPERCASE")]
pub struct Feature {
    pub name: String,
    pub stage: String,
    pub enabled: bool,
}

impl Feature {
    pub const KUBERNETES_FEATURES: &str = "kubernetes_feature_enabled";

    pub fn from_scrape(scrape: Scrape) -> Vec<Self> {
        scrape
            .samples
            .into_iter()
            .filter(|sample| sample.metric == Self::KUBERNETES_FEATURES)
            .filter_map(Self::from_sample)
            .collect::<Vec<_>>()
    }

    pub fn from_sample(sample: Sample) -> Option<Self> {
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
