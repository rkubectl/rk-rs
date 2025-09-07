use super::*;

/// Check whether an action is allowed.
#[derive(Clone, Debug, Args)]
pub struct WhoAmI;

impl WhoAmI {
    pub async fn ask(&self, context: &Context) -> RkResult<()> {
        let kubeapi = context.kubeapi();
        let ssr = authenticationv1::SelfSubjectReview::default();
        let pp = kubeapi.post_params();
        let ssr = kubeapi
            .selfsubjectreviews()?
            .create(&pp, &ssr)
            .await
            .inspect(|k| kubeapi.inspect(k))
            .inspect_err(|err| kubeapi.inspect_err(err))?;
        let show_params = default();
        let output = context.output_deprecated();
        println!("{}", ssr.output(false, &show_params, output));
        Ok(())
    }
}
