use std::collections::BTreeMap;
use actix_web::{
    post,
    web::Json,
    web::Data,
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use crate::AppState;
use surrealdb::sql::Value as ValueX;



#[derive(Serialize, Deserialize, Debug)]
pub struct ProductUpdate {
    usid: String,
    total: f32
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Listitems {
    productid: String,
    orderidretr: i64,
    quantity: i64
}

#[derive(Debug)]
pub struct SurrealDbError(surrealdb::Error);

impl actix_web::ResponseError for SurrealDbError {
    fn error_response(&self) -> HttpResponse {
        // Create an appropriate HTTP response for the error
        HttpResponse::InternalServerError().finish()
    }
}

impl std::fmt::Display for SurrealDbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SurrealDbError({})", self.0)
    }
}

impl Serialize for SurrealDbError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self.0))
    }
}

macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {{
		let mut m = ::std::collections::BTreeMap::new();
        $(m.insert($k, $v);)+
        m
    }};
  }



#[post("/api/v1/orders")]
pub async fn orderhandler(state: Data<AppState>, body: Json<ProductUpdate> ) -> Result<HttpResponse, SurrealDbError> {
    let orderid = sqlx::types::Uuid::from_u128(uuid::Uuid::new_v4().as_u128());
    let db = &state.db;
    let ses = &state.session.clone();
    let now = chrono::Utc::now();
    let now_str = now.to_rfc3339();

    let ast_ref = "
    CREATE orderdet SET orderid = $orderid, usid = $usid, total = $total, created_at = $created_at;
    ";

    let values: BTreeMap<String, ValueX> = map![
        "orderid".into() => ValueX::from(orderid.to_string()),
        "usid".into() => ValueX::from(body.usid.clone()),
        "total".into() => ValueX::from(body.total.clone()),
        "created_at".into() => ValueX::from(now_str)

    ];
    let res = db.execute(&ast_ref.to_string(), &ses, Some(values), false).await.map_err(|error| SurrealDbError(error));
    Ok(HttpResponse::Ok().json(res))
}

#[post("/api/v1/orders/listitems")]
pub async fn listitems(state: Data<AppState>, body: Json<Listitems> ) -> Result<HttpResponse, SurrealDbError> {
    let db = &state.db;
    let ses = &state.session.clone();
    let ast_ref = "
    CREATE listitems SET productid = $productid, orderidretr = $orderidretr, quantity = $quantity;
    ";
    let values: BTreeMap<String, ValueX> = map![
        "productid".into() => ValueX::from(body.productid.clone()),
        "orderidretr".into() => ValueX::from(body.orderidretr.clone()),
        "quantity".into() => ValueX::from(body.quantity.clone()),
    ];
    let res = db.execute(&ast_ref.to_string(), &ses, Some(values), false).await.map_err(|error| SurrealDbError(error));
    Ok(HttpResponse::Ok().json(res))
}

        

