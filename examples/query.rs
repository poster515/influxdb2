use influxdb2::models::{LanguageRequest, Query};
use structmap::FromMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let influx_url = "http://localhost:8086";
    let token = "some-token";

    let client = influxdb2::Client::new(influx_url, "org", token);

    client.query_suggestions().await?;
    client.query_suggestions_name("some-name").await?;

    #[derive(structmap_derive::FromMap)]
    struct Measurement {
        value: f64,
    }
    impl Default for Measurement {
        fn default() -> Self {
            Self { value: 0f64 }
        }
    }

    client
        .query::<Measurement>(Some(Query::new("some-query".to_string())))
        .await?;

    client
        .query_analyze(Some(Query::new("some-query".to_string())))
        .await?;

    client
        .query_ast(Some(LanguageRequest::new("some-query".to_string())))
        .await?;

    Ok(())
}
