use std::collections::BTreeMap;
use crate::AppState;
use actix_web::{
    get, post,
    web::Json,
    web::Data,
    HttpResponse,
};
use chrono;
use serde::{Deserialize, Serialize};
use surrealdb::sql::parse;
use argon2::PasswordHasher;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2,
};
use surrealdb::sql::Value as ValueX;
 

#[derive(Serialize, Deserialize, Debug)]
pub struct UserReg {
    fullname: String,
    dob: String,
    gender: String,
    mob_phone: String,
    email: String,
    passwd: String,
    address: String,
    city: String,
    postcode: String
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


#[post("/api/v1/users/regroute")]
pub async fn userregister(state: Data<AppState>, body: Json<UserReg>) -> Result<HttpResponse, SurrealDbError> {
    let usid = sqlx::types::Uuid::from_u128(uuid::Uuid::new_v4().as_u128());
    let addrid = sqlx::types::Uuid::from_u128(uuid::Uuid::new_v4().as_u128());
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(body.passwd.as_bytes(), &salt)
        .unwrap()
        .to_string();
    
    let db = &state.db;
    let ses = &state.session.clone();
    //check if session has a scope for users
    // let ast_ref = "INSERT INTO users (usid, fullname, dob, gender, mob_phone, email, passwd, created_at) VALUES ($usid, $fullname, $dob, $gender, $mob_phone, $email, $passwd, $created_at) RETURN *";
    //add a scope for users
    let ast_ref = "
    CREATE users SET usid = $usid, fullname = $fullname, dob = $dob, gender = $gender, mob_phone = $mob_phone, email = $email, passwd = $passwd, created_at = $created_at;
    CREATE useraddr SET addrid = $addrid, usid = $usid, address = $address, city = $city, postcode = $postcode;
    ";
    // println!("{:?}", ast_ref);
    let now = chrono::Utc::now();
    let now_str = now.to_rfc3339();
    let values: BTreeMap<String, ValueX> = map![
        "usid".into() => ValueX::from(usid.to_string()),
        "fullname".into() => ValueX::from(body.fullname.clone()),
        "dob".into() => ValueX::from(body.dob.clone()),
        "gender".into() => ValueX::from(body.gender.clone()),
        "mob_phone".into() => ValueX::from(body.mob_phone.clone()),
        "email".into() => ValueX::from(body.email.clone()),
        "passwd".into() => ValueX::from(password_hash.clone()),
        "created_at".into() => ValueX::from(now_str.clone()),
        "addrid".into() => ValueX::from(addrid.to_string()),
        "address".into() => ValueX::from(body.address.clone()),
        "city".into() => ValueX::from(body.city.clone()),
        "postcode".into() => ValueX::from(body.postcode.clone())
    ];
    let res = db.execute(&ast_ref.to_string(), &ses, Some(values), false).await.map_err(|error| SurrealDbError(error));
    Ok(HttpResponse::Ok().json(res))
}



#[get("/api/v1/users")]
pub async fn fetchusers(state: Data<AppState>) -> Result<HttpResponse, SurrealDbError> {
    let db = &state.db;
    let ses = state.session.clone();
    //add a scope to the transaction
    let ast_ref = parse("SELECT * FROM users;").expect("error");
    let ast = ast_ref.clone();
    let res = db.process(ast, &ses, None, false).await.map_err(|error| SurrealDbError(error));
    Ok(HttpResponse::Ok().json(res))
}
































