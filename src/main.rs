use actix_web::{web::Data, App, HttpServer};
use otherroutes::{getinfo, createtable, proddescr};
use std::sync::Arc;
use surrealdb::{Datastore, Session};
mod productroutes;
mod userroutes;
use productroutes::{getproducts, insertproductimages, insertproduct};
use userroutes::{userregister, fetchusers};
mod otherroutes;
mod orderroutes;
use orderroutes::{orderhandler};

use dotenv::dotenv;

pub struct AppState {
    db: Arc<Datastore>,
    session: Session
}

impl AppState {
    fn _clone(&self) -> Arc<Datastore> {
        self.db.clone()
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let clang_lib_path = std::env::var("CLANG_LIB_PATH").unwrap_or("/usr/lib/llvm-10/lib".to_string());
    // println!("cargo:rustc-link-lib=static=clang");
    // println!("cargo:rustc-link-search=native={}", clang_lib_path);
    dotenv().ok();
    let db_name: String = std::env::var("DB_URL").expect("DB_URL must be set");
    let db_loc: String = std::env::var("DB_SESS").expect("DB_SESS must be set");
    let ses = Session::for_kv().with_ns(db_name.as_str()).with_db(db_loc.as_str());
    // let ses: Session = Session::for_sc(db_name.as_str(), db_loc.as_str(), "admin");
    let ds = Datastore::new("file://database.db")
        .await
        .expect("error creating datastore");
    let ds = Arc::new(ds);
    let appstart = move || {
        App::new()
            .app_data(Data::new(AppState { db: ds.clone(), session: ses.clone()}))
            .service(getproducts)
            .service(userregister)
            .service(fetchusers)
            .service(createtable)
            .service(getinfo)
            .service(proddescr)
            .service(insertproduct)
            .service(insertproductimages)
            .service(orderhandler)
    };
    HttpServer::new(appstart)
        .bind("0.0.0.0:10000")?
        .run()
        .await
}
