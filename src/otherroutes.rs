use std::collections::BTreeMap;
use actix_web::{
    get, post,
    web::Json,
    web::Data,
    HttpResponse,
};
use surrealdb::sql::parse;
use serde::{Serialize, Deserialize};
use surrealdb::sql::Value as ValueX;
use crate::AppState;


#[derive(Serialize, Deserialize, Debug)]
pub struct DataInsertProddescr {
    descr: String,

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


#[post("/api/v1/products/proddescr")]
pub async fn proddescr(state: Data<AppState>, body: Json<DataInsertProddescr>) -> Result<HttpResponse, SurrealDbError> {
    let db = &state.db;
    let ses = &state.session.clone();
    let ast_ref = "
    CREATE prodcategory SET descr = $descr;";
    let values: BTreeMap<String, ValueX> = map![
        "descr".into() => ValueX::from(body.descr.clone())
    ];
    let res = db.execute(&ast_ref.to_string(), &ses, Some(values), false).await.map_err(|error| SurrealDbError(error));
    Ok(HttpResponse::Ok().json(res))
}


#[post("/api/v1/createtable")]
pub async fn createtable(state: Data<AppState>) -> Result<HttpResponse, SurrealDbError> {
    let db = &state.db;
    let ses = state.session.clone();

    let ast_ref = parse("
    DEFINE TABLE users SCHEMAFULL;
    DEFINE FIELD usid ON users TYPE string ASSERT is::uuid($value);
    DEFINE FIELD fullname ON users TYPE string; 
    DEFINE FIELD dob ON users TYPE string;
    DEFINE FIELD gender ON users TYPE string;
    DEFINE FIELD email ON users TYPE string ASSERT is::email($value);
    DEFINE FIELD mob_phone ON users TYPE string;
    DEFINE FIELD passwd ON users TYPE string;
    DEFINE FIELD created_at ON users TYPE string;
    DEFINE INDEX email_idx ON users COLUMNS email UNIQUE;
    DEFINE INDEX mob_phone_idx ON users COLUMNS mob_phone UNIQUE;

    DEFINE TABLE useraddr SCHEMAFULL;
    DEFINE FIELD addrid ON useraddr TYPE string;
    DEFINE FIELD usid ON useraddr TYPE record(users);
    DEFINE FIELD address ON useraddr TYPE string;
    DEFINE FIELD city ON useraddr TYPE string;
    DEFINE FIELD postcode ON useraddr TYPE string;

    DEFINE TABLE prodcategory SCHEMAFULL;
    DEFINE FIELD descr ON prodcategory TYPE string;
    DEFINE INDEX descr_idx ON prodcategory COLUMNS descr UNIQUE;

    DEFINE TABLE products SCHEMAFULL;
    DEFINE FIELD productid ON products TYPE string;
    DEFINE FIELD prodname ON products TYPE string;
    DEFINE FIELD proddescr ON products TYPE string;
    DEFINE FIELD prodsku ON products TYPE string;
    DEFINE FIELD descr ON products TYPE record(prodcategory);
    DEFINE FIELD availableqty ON products TYPE int;
    DEFINE FIELD price ON products TYPE string;
    DEFINE FIELD created_at ON products TYPE string;
    DEFINE INDEX prodname_idx ON products COLUMNS prodname UNIQUE;
    DEFINE INDEX prodsku_idx ON products COLUMNS prodsku UNIQUE;

    DEFINE TABLE productimages SCHEMAFULL;
    DEFINE FIELD productimgid ON productimages TYPE string;
    DEFINE FIELD prodsku ON productimages TYPE record(products);
    DEFINE FIELD imageone ON productimages TYPE string;
    DEFINE FIELD imagetwo ON productimages TYPE string;
    DEFINE FIELD imagethree ON productimages TYPE string;
    DEFINE FIELD imagefour ON productimages TYPE string;

    DEFINE TABLE orderdet SCHEMAFULL;
    DEFINE FIELD orderid ON orderdet TYPE string;
    DEFINE FIELD usid ON orderdet TYPE record(users);
    DEFINE FIELD total ON orderdet TYPE float;
    DEFINE FIELD created_at ON orderdet TYPE string;


    DEFINE TABLE listitems SCHEMAFULL;
    DEFINE FIELD productid ON listitems TYPE record(products);
    DEFINE FIELD orderidretr ON listitems TYPE int;
    DEFINE FIELD quantity ON listitems TYPE int;

    DEFINE TABLE favourites SCHEMAFULL;
    DEFINE FIELD favid ON favourites TYPE string;
    DEFINE FIELD usid ON favourites TYPE record(users);
    DEFINE FIELD productid ON favourites TYPE record(products);

    ").expect("error");
    let res = db.process(ast_ref, &ses, None, false).await.map_err(|error| SurrealDbError(error));
    Ok(HttpResponse::Ok().json(res))
}



#[get("/api/v1/getinfo")]
pub async fn getinfo(state: Data<AppState>) -> Result<HttpResponse, SurrealDbError> {
    let db = &state.db;
    let ses = state.session.clone();
    //add a scope to the transaction
    let ast_ref = parse(
        "INFO FOR KV;
        INFO FOR NS;
        INFO FOR DB;
        INFO FOR TABLE users;

    ").expect("error");
    let ast = ast_ref.clone();
    let res = db.process(ast, &ses, None, false).await.map_err(|error| SurrealDbError(error));
    Ok(HttpResponse::Ok().json(res))
}



