use super::*;

/// Check whether an action is allowed.
#[derive(Clone, Debug, Args)]
pub struct WhoAmI;

impl WhoAmI {
    pub async fn ask(&self, context: &Context) -> kube::Result<()> {
        let kubectl = context.kubectl();
        let ssr = authenticationv1::SelfSubjectReview::default();
        let pp = kubectl.post_params();
        let ssr = kubectl
            .selfsubjectreviews()?
            .create(&pp, &ssr)
            .await
            .inspect(|k| kubectl.inspect(k))
            .inspect_err(|err| kubectl.inspect_err(err))?;
        let show_params = default();
        let output = context.output_deprecated();
        println!("{}", ssr.output(false, &show_params, output));
        Ok(())
    }
}
