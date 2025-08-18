use super::*;

use generic::CreateGenericSecret;

mod generic;

/// Create a secret with specified type.
#[derive(Clone, Debug, Subcommand)]
pub enum CreateSecret {
    DockerRegistry,
    Generic(CreateGenericSecret),
    Tls,
}

impl CreateSecret {
    pub async fn exec(
        &self,
        kubectl: &Kubectl,
        pp: &api::PostParams,
    ) -> kube::Result<Box<dyn Show>> {
        let k = match self {
            Self::DockerRegistry => todo!(),
            Self::Generic(create_generic_secret) => create_generic_secret.exec(kubectl, pp).await,
            Self::Tls => todo!(),
        }?;
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
