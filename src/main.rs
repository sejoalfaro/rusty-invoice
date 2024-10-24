use actix_web::{web, App, HttpServer, HttpResponse, Responder, get, post};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Estructura de la Factura
#[derive(Serialize, Deserialize)]
struct Factura {
    id: Uuid,
    cliente: String,
    total: f64,
    detalle: Vec<DetalleFactura>,
}

#[derive(Serialize, Deserialize)]
struct DetalleFactura {
    descripcion: String,
    cantidad: u32,
    precio_unitario: f64,
}

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("ok!")
}

#[get("/factura")]
async fn obtener_factura() -> impl Responder {
    HttpResponse::Ok().json(Factura {
        id: Uuid::new_v4(),
        cliente: String::from("Cliente Ejemplo"),
        total: 1000.0,
        detalle: vec![
            DetalleFactura {
                descripcion: String::from("Producto A"),
                cantidad: 2,
                precio_unitario: 500.0,
            }
        ],
    })
}

#[post("/factura")]
async fn crear_factura(factura: web::Json<Factura>) -> impl Responder {
    HttpResponse::Ok().json(factura.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(obtener_factura)  // GET /factura
            .service(crear_factura)    // POST /factura
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}