use super::*;

use docker::CreateDockerRegistrySecret;
use generic::CreateGenericSecret;
use tls::CreateTlsSecret;

mod docker;
mod generic;
mod tls;

/// Create a secret with specified type.
///
///  A docker-registry type secret is for accessing a container registry.
///
///  A generic type secret indicate an Opaque secret type.
///
///  A tls type secret holds TLS certificate and its associated key.
#[derive(Clone, Debug, Subcommand)]
pub enum CreateSecret {
    DockerRegistry(CreateDockerRegistrySecret),
    Generic(CreateGenericSecret),
    Tls(CreateTlsSecret),
}

impl CreateSecret {
    pub async fn exec(&self, kubeapi: &Kubeapi, pp: &api::PostParams) -> RkResult<Box<dyn Show>> {
        trace!(?kubeapi, ?pp);
        let data = match self {
            Self::DockerRegistry(docker_registry) => docker_registry.secret().await,
            Self::Generic(generic) => generic.secret().await,
            Self::Tls(tls) => tls.secret(),
        }?;

        let k = kubeapi
            .secrets()?
            .create(pp, &data)
            .await
            .inspect(|ns| kubeapi.inspect(ns))?;

        let created = Created { k };
        Ok(Box::new(created))
    }
}

// Create a secret with specified type.

//  A docker-registry type secret is for accessing a container registry.

//  A generic type secret indicate an Opaque secret type.

//  A tls type secret holds TLS certificate and its associated key.

// Available Commands:
//   docker-registry   Create a secret for use with a Docker registry
//   generic           Create a secret from a local file, directory, or literal value
//   tls               Create a TLS secret

// Usage:
//   kubectl create secret (docker-registry | generic | tls) [options]

// Use "kubectl create secret <command> --help" for more information about a given command.
// Use "kubectl options" for a list of global command-line options (applies to all commands).
