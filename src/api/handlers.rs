use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use crate::errors::app_error::AppError;
use crate::services::astronomy::AstronomyService;

pub async fn get_positions() -> impl IntoResponse {
    let service = AstronomyService::new();
    let response = service.get_astro_position().await;

    match response {
        Ok(data) => Ok(Json({
            data.locations.iter()
                .map(|location| location.id.clone())
                .collect::<Vec<String>>() // Explicitly specify Vec<String>
        })),
        Err(error) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            error
        )),
    }
}