use super::*;

impl CreateResource {
    pub async fn namespace(
        &self,
        name: &str,
        kubectl: &Kubectl,
        pp: &api::PostParams,
    ) -> kube::Result<Box<dyn Show>> {
        let data = corev1::Namespace::new(name);
        let k = kubectl
            .namespaces()?
            .create(pp, &data)
            .await
            .inspect(|ns| kubectl.inspect(ns))?;
        let created = Created { k };
        Ok(Box::new(created))
    }
}

// Create a namespace with the specified name.

// Aliases:
// namespace, ns

// Examples:
//   # Create a new namespace named my-namespace
//   kubectl create namespace my-namespace

// Options:
//     --allow-missing-template-keys=true:
// 	If true, ignore any errors in templates when a field or map key is missing in the template. Only applies to
// 	golang and jsonpath output formats.

//     --dry-run='none':
// 	Must be "none", "server", or "client". If client strategy, only print the object that would be sent, without
// 	sending it. If server strategy, submit server-side request without persisting the resource.

//     --field-manager='kubectl-create':
// 	Name of the manager used to track field ownership.

//     -o, --output='':
// 	Output format. One of: (json, yaml, name, go-template, go-template-file, template, templatefile, jsonpath,
// 	jsonpath-as-json, jsonpath-file).

//     --save-config=false:
// 	If true, the configuration of current object will be saved in its annotation. Otherwise, the annotation will
// 	be unchanged. This flag is useful when you want to perform kubectl apply on this object in the future.

//     --show-managed-fields=false:
// 	If true, keep the managedFields when printing objects in JSON or YAML format.

//     --template='':
// 	Template string or path to template file to use when -o=go-template, -o=go-template-file. The template format
// 	is golang templates [http://golang.org/pkg/text/template/#pkg-overview].

//     --validate='strict':
// 	Must be one of: strict (or true), warn, ignore (or false). "true" or "strict" will use a schema to validate
// 	the input and fail the request if invalid. It will perform server side validation if ServerSideFieldValidation
// 	is enabled on the api-server, but will fall back to less reliable client-side validation if not. "warn" will
// 	warn about unknown or duplicate fields without blocking the request if server-side field validation is enabled
// 	on the API server, and behave as "ignore" otherwise. "false" or "ignore" will not perform any schema
// 	validation, silently dropping any unknown or duplicate fields.

// Usage:
//   kubectl create namespace NAME [--dry-run=server|client|none] [options]

// Use "kubectl options" for a list of global command-line options (applies to all commands).
