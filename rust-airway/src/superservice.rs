use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "lannonbr.com",
    version = "v1",
    kind = "SuperService",
    namespaced
)]
pub struct SuperServiceSpec {
    pub port: i32,
}
