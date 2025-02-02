use actix_web::{body::MessageBody, cookie::Cookie, dev::{ServiceFactory, ServiceRequest, ServiceResponse}, middleware::from_fn, post, web::{Data, Json}, Error, HttpMessage, HttpRequest, HttpResponse, Scope};
use sea_orm::DatabaseConnection;
use validator::Validate;

use crate::{user::{user_dto::UserDto, user_service::{create, verify_account}}, utils::{jwt_guard, Claims}};

#[post("/signup")]
async fn signup_handler(user: Json<UserDto>, db: Data<DatabaseConnection>)-> HttpResponse {
    if let Err(errors) = user.validate() {
        let error_message = format!("{}", errors);
        return HttpResponse::BadRequest().body(error_message);
    }

    let response = create(user.0, db.as_ref().clone()).await;
    if let Err(error) = response {
        return HttpResponse::from_error(error);
    }

    let cookie = Cookie::new("auth", response.unwrap());

    HttpResponse::Ok().cookie(cookie).finish()
}

#[post("/verify_account")]
async fn verification_handler(token: Json<String>, db: Data<DatabaseConnection>, req: HttpRequest)-> HttpResponse{
    let extensions = req.extensions();
    let user: Option<&Claims> = extensions.get();
    if user.is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let response = verify_account(token.0, db.get_ref().clone()).await;

    if let Err(error) = response {
        return HttpResponse::from_error(error);
    }

    let msg_body = serde_json::to_string(&response.unwrap()).unwrap();

    HttpResponse::Ok().body(msg_body)
}

pub fn user_controller()-> Scope<impl ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse<impl MessageBody>, Error = Error, InitError = ()>> {
    Scope::new("/user")
        .service(signup_handler)
        .service(
            Scope::new("")
                .wrap(from_fn(jwt_guard))
                .service(verification_handler), 
        )
}