use std::collections::BTreeMap;

use crate::AppState;
use actix_web::web::Json;
use actix_web::{
    get,
    web::Data,
    HttpResponse, post,
};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use surrealdb::sql::parse;
use surrealdb::sql::Value as ValueX;


#[derive(Serialize)]

struct Products {
    productid : Uuid,
    prodname: String,
    proddescr: String,
    prodsku: String,
    descr: String,
    availableqty: i64,
    price: String,
    imageone: String,
    imagetwo: String,
    imagethree: String,
    imagefour: String,
    created_at: chrono::DateTime<chrono::Utc>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    prodname: String,
    proddescr: String,
    prodsku: String,
    descr: String,
    availableqty: i64,
    price: String
}

#[derive(Serialize, Deserialize, Debug)]

pub struct ProductImages {
    prodsku: String,
    imageone: String,
    imagetwo: String,
    imagethree: String,
    imagefour: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Favourites {
    usid: String,
    productid: String,
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


#[get("/api/v1/products")]
pub async fn getproducts(state: Data<AppState>) -> Result<HttpResponse, SurrealDbError> {
    let db = &state.db;
    let ses = &state.session.clone();
    let mut tx = db.transaction(true, false).await.expect("error creating transaction");
    let ast_ref = parse("SELECT * FROM products;").expect("error");
    let ast = ast_ref.clone();
    tx.commit().await.expect("error committing transaction");
    let res = db.process(ast, &ses, None, false).await.map_err(|error| SurrealDbError(error));
    Ok(HttpResponse::Ok().json(res))
}


#[post("/api/v1/products")]
pub async fn insertproduct(state: Data<AppState>, body: Json<Product> ) -> Result<HttpResponse, SurrealDbError> {
    let productid = sqlx::types::Uuid::from_u128(uuid::Uuid::new_v4().as_u128());
    let db = &state.db;
    let ses = &state.session.clone();
    let now = chrono::Utc::now();
    let now_str = now.to_rfc3339();
    let ast_ref = "
    CREATE products SET productid = $productid, prodname = $prodname, proddescr = $proddescr, prodsku = $prodsku, descr = $descr, availableqty = $availableqty, price = <float> $price, created_at = $created_at;
    ";
    let values: BTreeMap<String, ValueX> = map![
        "productid".into() => ValueX::from(productid.to_string()),
        "prodname".into() => ValueX::from(body.prodname.clone()),
        "proddescr".into() => ValueX::from(body.proddescr.clone()),
        "prodsku".into() => ValueX::from(body.prodsku.clone()),
        "descr".into() => ValueX::from(body.descr.clone()),
        "availableqty".into() => ValueX::from(body.availableqty.clone()),
        "price".into() => ValueX::from(body.price.clone()),
        "created_at".into() => ValueX::from(now_str)

    ];
    let res = db.execute(&ast_ref.to_string(), &ses, Some(values), false).await.map_err(|error| SurrealDbError(error));
    Ok(HttpResponse::Ok().json(res))
}



#[post("/api/v1/product/images")]
pub async fn insertproductimages(state: Data<AppState>, body: Json<ProductImages> ) -> Result<HttpResponse, SurrealDbError> {
    let productimgid = sqlx::types::Uuid::from_u128(uuid::Uuid::new_v4().as_u128());
    let db = &state.db;
    let ses = &state.session.clone();
    let ast_ref = "
    CREATE productimages SET productimgid = $productimgid, prodsku = $prodsku, imageone = $imageone, imagetwo = $imagetwo, imagethree = $imagethree, imagefour = $imagefour;
    ";

    let values: BTreeMap<String, ValueX> = map![
        "productimgid".into() => ValueX::from(productimgid.to_string()),
        "prodsku".into() => ValueX::from(body.prodsku.clone()),
        "imageone".into() => ValueX::from(body.imageone.clone()),
        "imagetwo".into() => ValueX::from(body.imagetwo.clone()),
        "imagethree".into() => ValueX::from(body.imagethree.clone()),
        "imagefour".into() => ValueX::from(body.imagefour.clone())

    ];
    let res = db.execute(&ast_ref.to_string(), &ses, Some(values), false).await.map_err(|error| SurrealDbError(error));
    Ok(HttpResponse::Ok().json(res))
}


#[post("/api/v1/datainsert/products/favourites")]
pub async fn favourites(state: Data<AppState>, body: Json<Favourites> ) -> Result<HttpResponse, SurrealDbError> {
    let favid = sqlx::types::Uuid::from_u128(uuid::Uuid::new_v4().as_u128());
    let db = &state.db;
    let ses = &state.session.clone();
    let ast_ref = "
    CREATE favourites SET favid = $favid, usid = $usid, productid = $productid;
    ";

    let values: BTreeMap<String, ValueX> = map![
        "favid".into() => ValueX::from(favid.to_string()),
        "usid".into() => ValueX::from(body.usid.clone()),
        "productid".into() => ValueX::from(body.productid.clone())
    ];
    
    let res = db.execute(&ast_ref.to_string(), &ses, Some(values), false).await.map_err(|error| SurrealDbError(error));
    Ok(HttpResponse::Ok().json(res))
}

