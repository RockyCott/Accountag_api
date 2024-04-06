use axum::http::HeaderMap;

pub async fn handle_request(headers: HeaderMap) -> String {
    match headers.get("idUsuario") {
        Some(id) => id.to_str().unwrap().to_owned(),
        None => "No se encontró el idUsuario".to_owned(),
    }
}