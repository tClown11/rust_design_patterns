use crate::greeter::greeter_client::GreeterClient;
use crate::greeter::HelloRequest;

pub async fn run_client() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_say_hello() {
        let mut client = GreeterClient::connect("http://[::1]:50051").await.unwrap();

        let request = tonic::Request::new(HelloRequest {
            name: "Test".into(),
        });

        let response = client.say_hello(request).await.unwrap();

        assert_eq!(response.into_inner().message, "Hello Test!");
    }
}