use hello_world::{greeter_client::GreeterClient, HelloRequest};
use hyper_util::rt::TokioIo;
use tokio_vsock::{VsockAddr, VsockStream};
use tonic::{
    transport::{Endpoint, Uri},
    Request,
};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() {
    let endpoint = Endpoint::try_from("http://[::1]:8000")
        .unwrap()
        .connect_with_connector(tower::service_fn(|_: Uri| async {
            Ok::<_, std::io::Error>(TokioIo::new(
                VsockStream::connect(VsockAddr::new(1, 8000)).await?,
            ))
        }))
        .await
        .unwrap();
    let mut client = GreeterClient::new(endpoint);
    let response = client
        .say_hello(Request::new(HelloRequest {
            name: "My name".to_string(),
        }))
        .await
        .unwrap();
    dbg!(response);
}
