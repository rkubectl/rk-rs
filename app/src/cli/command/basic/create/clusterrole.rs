use indexmap::IndexMap;
use k8s_openapi_ext::LabelSelectorExt;

use super::*;

/// Create a cluster role.
///
/// Examples:
///   # Create a cluster role named "pod-reader" that allows user to perform "get", "watch" and "list" on pods
///   kubectl create clusterrole pod-reader --verb=get,list,watch --resource=pods
///
///   # Create a cluster role named "pod-reader" with ResourceName specified
///   kubectl create clusterrole pod-reader --verb=get --resource=pods --resource-name=readablepod
/// --resource-name=anotherpod
///
///   # Create a cluster role named "foo" with API Group specified
///   kubectl create clusterrole foo --verb=get,list,watch --resource=rs.apps
///
///   # Create a cluster role named "foo" with SubResource specified
///   kubectl create clusterrole foo --verb=get,list,watch --resource=pods,pods/status
///
///   # Create a cluster role name "foo" with NonResourceURL specified
///   kubectl create clusterrole "foo" --verb=get --non-resource-url=/logs/*
///
///   # Create a cluster role name "monitoring" with AggregationRule specified
///   kubectl create clusterrole monitoring --aggregation-rule="rbac.example.com/aggregate-to-monitoring=true"
#[derive(Clone, Debug, Args)]
pub struct CreateClusterRole {
    /// Cluster Role object name
    name: String,

    #[command(flatten)]
    policy_rules: PolicyRulesArgs,

    #[arg(
        long,
        conflicts_with = "PolicyRulesArgs",
        value_delimiter = ',',
        value_parser = KeyValue::<String>::value_parser()
    )]
    aggregation_rule: Option<Vec<KeyValue<String>>>,
}

#[derive(Clone, Debug, Args)]
struct PolicyRulesArgs {
    #[arg(long, value_delimiter = ',')]
    non_resource_url: Vec<String>,

    #[arg(long, value_delimiter = ',')]
    resource: Vec<String>,

    #[arg(long, value_delimiter = ',')]
    resource_name: Vec<String>,

    #[arg(long, num_args(1..), value_delimiter = ',')]
    verb: Vec<String>,
}

impl CreateClusterRole {
    pub async fn exec(&self, kubeapi: &Kubeapi, pp: &api::PostParams) -> RkResult<Box<dyn Show>> {
        let data = self.cluster_role();

        let k = kubeapi
            .clusterroles()?
            .create(pp, &data)
            .await
            .inspect(|ns| kubeapi.inspect(ns))?;

        let created = Created { k };
        Ok(Box::new(created))
    }

    fn cluster_role(&self) -> rbacv1::ClusterRole {
        let cluster_role = rbacv1::ClusterRole::new(&self.name);
        if let Some(aggregation_rule) = self.aggregation_rule() {
            rbacv1::ClusterRole {
                aggregation_rule: Some(aggregation_rule),
                ..cluster_role
            }
        } else {
            let rules = self.policy_rules.policy_rules();
            cluster_role.rules(rules)
        }
    }

    fn aggregation_rule(&self) -> Option<rbacv1::AggregationRule> {
        let labels = self
            .aggregation_rule
            .as_ref()?
            .iter()
            .map(|kv| kv.as_pair());
        let label_selector = metav1::LabelSelector::new().match_labels(labels);
        let cluster_role_selectors = Some(vec![label_selector]);

        Some(rbacv1::AggregationRule {
            cluster_role_selectors,
        })
    }
}

impl PolicyRulesArgs {
    fn policy_rules(&self) -> Vec<rbacv1::PolicyRule> {
        let verbs = &self.verb;
        let names = &self.resource_name;
        self.resources()
            .into_iter()
            .map(|(api_group, kinds)| {
                rbacv1::PolicyRule::default()
                    .api_group(api_group)
                    .resources(kinds)
                    .verbs(verbs)
                    .resource_names(names)
            })
            .chain(self.non_resource_urls())
            .collect()
    }

    fn non_resource_urls(&self) -> Option<rbacv1::PolicyRule> {
        if self.non_resource_url.is_empty() {
            None
        } else {
            let verbs = self.verb.clone();
            let non_resource_urls = Some(self.non_resource_url.clone());
            Some(rbacv1::PolicyRule {
                non_resource_urls,
                verbs,
                ..default()
            })
        }
    }

    fn resources(&self) -> IndexMap<&str, Vec<&str>> {
        self.resource
            .iter()
            .map(|resource| resource_to_gk(resource))
            .fold(IndexMap::new(), |mut map, (key, value)| {
                map.entry(key).or_default().push(value);
                map
            })
    }
}

fn resource_to_gk(resource: &str) -> (&str, &str) {
    if let Some((resource, group)) = resource.split_once('.') {
        (group, resource)
    } else {
        ("", resource)
    }
}
