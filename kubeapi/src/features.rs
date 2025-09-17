use prometheus_parse::Scrape;

use super::*;

impl Kubeapi {
    pub async fn features(&self) -> kube::Result<Vec<Feature>> {
        self.metrics("metrics").await.map(Feature::from_scrape)
    }

    pub async fn metrics(&self, name: &str) -> kube::Result<Scrape> {
        let text = self.raw_get(name).await?;
        let lines = text.lines().map(String::from).map(Ok);
        Scrape::parse(lines).map_err(kube::Error::ReadEvents)
    }
}
