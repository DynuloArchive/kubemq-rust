pub mod kubemq_proto {
    tonic::include_proto!("kubemq");
}

use kubemq_proto::{kubemq_client::KubemqClient, Empty};

lazy_static::lazy_static! {
    static ref KUBEMQ_HOST: String = std::env::var("KUBEMQ_HOST").unwrap_or_else(|_| String::from("kubemq-cluster-grpc.kubemq.svc.cluster.local"));
    static ref KUBEMQ_PORT: String = std::env::var("KUBEMQ_PORT").unwrap_or_else(|_| String::from("50000"));
}

impl KubemqClient {
    pub fn host_from_env() -> String {
        format!("http://{}:{}", *KUBEMQ_HOST, *KUBEMQ_PORT)
    }
}
