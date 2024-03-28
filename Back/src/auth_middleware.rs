use std::sync::Arc;

use crate::database::model::Role::Admin;
use crate::{database::model::User, jwt_manager, AppState};

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};

// check if authenticated and add the local_user to the request
pub async fn auth_middleware(
    State(app_state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Response {
    if let Some(bearer_token) = request
        .headers()
        .get("authorization")
        .and_then(|header| header.to_str().ok())
    {
        let token = bearer_token.trim_start_matches("Bearer ");
        match jwt_manager::decode_jwt(token) {
            Ok(id) => {
                if let Some(local_user) = app_state.userdata.get_user_by_id(id) {
                    request.extensions_mut().insert(local_user);
                    next.run(request).await
                } else {
                    StatusCode::UNAUTHORIZED.into_response()
                }
            }
            Err(_) => {
                StatusCode::UNAUTHORIZED.into_response()
                // let response = HttpResponse::Unauthorized().finish().map_into_right_body();
                // let (request, _pl) = req.into_parts();

                // return Box::pin(async move { Ok(ServiceResponse::new(request, response)) });
            }
        }
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}

pub async fn admin_middleware(
    State(_): State<Arc<AppState>>,
    Extension(local_user): Extension<User>,
    request: Request,
    next: Next,
) -> Response {
    if local_user.user_role != Admin {
        StatusCode::UNAUTHORIZED.into_response()
    } else {
        next.run(request).await
    }
}
