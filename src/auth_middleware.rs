use crate::database::UserData;
use crate::jwt_manager::decode_jwt;
use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Auth;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let db = req.app_data::<web::Data<UserData>>().unwrap().clone();

        if let Some(bearer_token) = req.headers().get("authorization") {
            if let Ok(bearer_token) = bearer_token.to_str() {
                let token = bearer_token.trim_start_matches("Bearer ");
                match decode_jwt(token) {
                    Ok(id) => {
                        if let Some(local_user) = db.get_user_by_id(id) {
                            req.extensions_mut().insert(local_user);
                        }
                    }
                    Err(_) => {
                        let response = HttpResponse::Unauthorized().finish().map_into_right_body();
                        let (request, _pl) = req.into_parts();

                        return Box::pin(
                            async move { Ok(ServiceResponse::new(request, response)) },
                        );
                    }
                }
            }
        }

        let res = self.service.call(req);
        Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
    }
}
