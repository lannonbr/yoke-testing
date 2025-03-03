use std::collections::BTreeMap;

use k8s_openapi::{
    api::{
        apps::v1::{Deployment, DeploymentSpec},
        core::v1::{Container, PodSpec, PodTemplateSpec, Service, ServicePort, ServiceSpec},
    },
    apimachinery::pkg::apis::meta::v1::{LabelSelector, ObjectMeta},
    serde_json,
};

// Deployment based on the Go code from the Yoke docs: https://yokecd.github.io/docs/examples/basics/
struct DeployConfig {
    name: String,
    namespace: String,
    labels: BTreeMap<String, String>,
    replicas: i32,
}

fn create_deploy(cfg: DeployConfig) -> Deployment {
    let deploy = Deployment {
        metadata: ObjectMeta {
            name: Some(cfg.name.clone()),
            namespace: Some(cfg.namespace),
            ..Default::default()
        },
        spec: Some(DeploymentSpec {
            selector: LabelSelector {
                match_labels: Some(cfg.labels.clone()),
                ..Default::default()
            },
            replicas: Some(cfg.replicas),
            template: PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    labels: Some(cfg.labels.clone()),
                    ..Default::default()
                }),
                spec: Some(PodSpec {
                    containers: vec![Container {
                        name: cfg.name.clone(),
                        image: Some(String::from("alpine:latest")),
                        command: Some(vec![
                            "watch".to_string(),
                            "echo".to_string(),
                            "hello world (from rust)".to_string(),
                        ]),
                        ..Default::default()
                    }],
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        }),
        ..Default::default()
    };

    deploy
}

fn main() {
    let mut labels = BTreeMap::new();
    labels.insert(String::from("app"), "example-rust".to_string());

    let deploy = create_deploy(DeployConfig {
        labels,
        name: String::from("example-app"),
        namespace: String::from("default"),
        replicas: 2,
    });

    // A test Service class to verify you can deploy multiple Kinds in a single takeoff. This is not connected to the deployment above.
    let svc = Service {
        metadata: ObjectMeta {
            name: Some("new-svc".to_string()),
            ..Default::default()
        },
        spec: Some(ServiceSpec {
            ports: Some(vec![ServicePort {
                port: 30012,
                ..Default::default()
            }]),
            ..Default::default()
        }),
        ..Default::default()
    };

    // The rust_openapi crate does have a generic Resource type, but it did not want to play ball when creating a Vec<Resource> and then dropping the above resources into it.
    // Instead, I just make a Vec<Value> using serde_json and then write that out as a string at the end of the program
    let resources = vec![
        serde_json::value::to_value(deploy).unwrap(),
        serde_json::value::to_value(svc).unwrap(),
    ];

    println!("{}", serde_json::to_string(&resources).unwrap());
}
