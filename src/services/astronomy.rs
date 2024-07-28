use libtad_rs::{ServiceClient};
use libtad_rs::models::astronomy::AstronomyObjectType;
use libtad_rs::models::time::DateTime;
use libtad_rs::service::astronomy::{AstronomyResponse, AstroPositionRequest};

pub struct AstronomyService {
    client: ServiceClient,
}

impl AstronomyService {
    pub fn new() -> Self {
        AstronomyService {
            client: ServiceClient::new("gTAaJ9UTA7".into(), "04WxCo6NeWoJAA4dQiWy".into()),
        }
    }

    pub async fn get_astro_position(&self) -> Result<AstronomyResponse, String> {
        let request = AstroPositionRequest::new()
            .with_object(AstronomyObjectType::Moon)
            .with_placeid("netherlands/amsterdam")
            .with_interval(DateTime::from("2000-01-01T14:14:34"));

        let result = self.client.get_astro_position(&request);

        match result {
            Ok(Ok(response)) => Ok(response), // Clone the response
            Ok(Err(api_error)) => Err(format!("API error: {:?}", api_error)),
            Err(error) => Err(format!("Underlying error: {:?}", error)),
        }
    }
}