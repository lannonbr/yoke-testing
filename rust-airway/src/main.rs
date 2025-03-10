use k8s_openapi::{
    apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionSpec,
    apimachinery::pkg::apis::meta::v1::ObjectMeta,
};
use kube::CustomResourceExt;
use rust_airway::superservice::SuperService;

use serde::{Deserialize, Serialize};

// Structs recreated from Yoke go pkg https://pkg.go.dev/github.com/yokecd/yoke/pkg/apis/airway/v1alpha1
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Airway {
    metadata: ObjectMeta,
    spec: AirwaySpec,
    kind: String,
    api_version: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct AirwaySpec {
    cluster_access: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    fix_drift_interval: Option<String>,
    wasm_urls: WasmUrls,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_path: Option<Vec<String>>,
    cross_namespace: bool,
    skip_admission_webhook: bool,
    template: CustomResourceDefinitionSpec,
}

#[derive(Serialize, Deserialize, Default)]
struct WasmUrls {
    flight: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    converter: Option<String>,
}

fn create_airway(metadata: ObjectMeta, spec: AirwaySpec) -> Airway {
    Airway {
        kind: "Airway".to_string(),
        api_version: "yoke.cd/v1alpha1".to_string(),
        metadata,
        spec,
    }
}

// The main program deploying the airway & setting up the CRD
fn main() {
    let airway = create_airway(
        ObjectMeta {
            name: Some("superservices.lannonbr.com".to_string()),
            ..Default::default()
        },
        AirwaySpec {
            wasm_urls: WasmUrls {
                // URL here is for testing purposes, you'd likely want to set this to something on a CDN
                // or eventually an OCI registry.
                flight: "http://host.docker.internal:3000/superservice.wasm.gz".to_string(),
                ..Default::default()
            },
            template: SuperService::crd().spec,
            ..Default::default()
        },
    );

    println!("{}", serde_json::to_string(&airway).unwrap());
}
