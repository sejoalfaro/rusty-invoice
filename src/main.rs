use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[tokio::main]
async fn main() {
    // Inicializar el sistema de logging
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // Crear el enrutador con las rutas de facturas y capa de logging
    let app = Router::new()
        .route("/invoice", post(create_invoice))
        .route("/health", get(health_status))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO)) // Registra las solicitudes a nivel INFO
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)), // Registra las respuestas a nivel INFO
        );

    // Iniciar el servidor
    axum::Server::bind(&"127.0.0.1:4321".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Estructura para la factura (invoice)
#[derive(Serialize, Deserialize)]
struct Invoice {
    id: String,
    total: f64,
    customer: String,
}

// Ruta para crear una nueva factura
async fn create_invoice(Json(payload): Json<Invoice>) -> Result<Json<Invoice>, StatusCode> {
    if payload.total <= 0.0 {
        // Si el total de la factura es inválido, devolver un error 400 (Bad Request)
        Err(StatusCode::BAD_REQUEST)
    } else {
        // Enviar la factura creada como respuesta (status 200 OK)
        Ok(Json(payload))
    }
}

// Ruta para verificar el estado del sistema
async fn health_status() -> Result<&'static str, StatusCode> {
    // Devolver un mensaje de estado (status 200 OK)
    Ok("UP")
}
