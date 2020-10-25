// use futures::stream::iter;

use hello::say_client::SayClient;
use hello::SayRequest;
use tonic::Request;

mod hello;

fn get_token() -> String {
    String::from("token")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cert=include_str!("../client.pem");
    let key=include_str!("../client.key");
    let id=tonic::transport::Identity::from_pem(cert.as_bytes(),key.as_bytes());
    let s=include_str!("../my_ca.pem");
    let ca=tonic::transport::Certificate::from_pem(s.as_bytes());
    let tls=tonic::transport::ClientTlsConfig::new().domain_name("localhost").identity(id).ca_certificate(ca);
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .tls_config(tls)
        .connect()
        .await?;
    let token = get_token();
    let mut client = SayClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert(
            "authorization",
            tonic::metadata::MetadataValue::from_str(&token).unwrap(),
        );
        Ok(req)
    });
    // let request = tonic::Request::new(iter(vec![
    //     SayRequest {
    //        name:String::from("anshul")
    //     },
    //     SayRequest {
    //        name:String::from("anshul")
    //     },
    //     SayRequest {
    //        name:String::from("anshul")
    //     },
    // ]));

    // let request = tonic::Request::new(iter(vec![
    // SayRequest {
    //     name: String::from("anshul"),
    // },
    //     SayRequest {
    //         name: String::from("rahul"),
    //     },
    //     SayRequest {
    //         name: String::from("vijay"),
    //     },
    // ]));
    let request = tonic::Request::new(SayRequest {
        name: String::from("anshul"),
    });
    let response = client.send(request).await?.into_inner();

    // while let Some(res) = response.message().await? {
    //     println!("NOTE = {:?}", res);
    // }

    println!("RESPONSE={}", response.message);

    Ok(())
}
