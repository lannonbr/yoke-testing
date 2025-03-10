use std::io::stdin;

use k8s_openapi::api::core::v1::{Service, ServicePort, ServiceSpec};
use kube::core::ObjectMeta;
use rust_airway::superservice::SuperService;

fn main() {
    let super_service_json: serde_json::Value =
        serde_yaml::from_reader(stdin()).expect("Failed to read yaml into serde_json::Value");

    let super_service: SuperService =
        serde_json::from_value(super_service_json).expect("Failed to serialize into SuperService");

    let service: Service = Service {
        metadata: ObjectMeta {
            name: Some(format!(
                "{}-svc",
                super_service.metadata.name.expect("expect to have a name")
            )),
            ..Default::default()
        },
        spec: Some(ServiceSpec {
            ports: Some(vec![ServicePort {
                port: super_service.spec.port,
                ..Default::default()
            }]),
            ..Default::default()
        }),
        ..Default::default()
    };

    let resources = vec![serde_json::value::to_value(service).unwrap()];

    println!("{}", serde_json::to_string(&resources).unwrap());
}
