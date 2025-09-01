use prometheus_parse::Scrape;

use super::*;

trait FeaturesExt {
    fn features(self) -> Vec<Feature>;
}

impl FeaturesExt for Scrape {
    fn features(self) -> Vec<Feature> {
        Feature::from_scrape(self)
    }
}

impl Kubectl {
    pub async fn features(&self) -> kube::Result<()> {
        let features = self.metrics().await?.features();
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
