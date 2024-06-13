use clap::Parser;
use std::time::Duration;
use tonic::transport::Endpoint;
struct App;
mod proto {
    tonic::include_proto!("ping");
}
#[tonic::async_trait]
impl proto::ping_server::Ping for App {
    async fn ping(
        &self,
        _: tonic::Request<()>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        tokio::time::sleep(Duration::from_secs(1)).await;
        Ok(tonic::Response::new(()))
    }
}
#[derive(Parser)]
struct Args {
    test_no: u8,
}
#[tokio::main]
async fn main() {
    let Args { test_no } = Args::parse();
    tokio::spawn(async {
        let svc = proto::ping_server::PingServer::new(App);
        let socket = format!("0.0.0.0:50000").parse().unwrap();
        let mut builder = tonic::transport::Server::builder();
        builder.add_service(svc).serve(socket).await.unwrap();
    });
    tokio::time::sleep(Duration::from_secs(3)).await;

    match test_no {
        0 => {
            eprintln!("test 0 (pass)");
            let mut cli = proto::ping_client::PingClient::connect("http://localhost:50000")
                .await
                .unwrap();
            cli.ping(()).await.unwrap();
        }
        1 => {
            eprintln!("test 1 (fail)");
            let endpoint = Endpoint::from_static("http://localhost:50000")
                // commenting out this line turn it into pass.
                .http2_keep_alive_interval(std::time::Duration::from_secs(1))
                .keep_alive_while_idle(true)
                .timeout(std::time::Duration::from_secs(5))
                .connect_timeout(std::time::Duration::from_secs(5));
            let chan = endpoint.connect().await.unwrap();
            let mut cli = proto::ping_client::PingClient::new(chan);
            cli.ping(()).await.unwrap();
        }
        _ => unreachable!(),
    }
}
