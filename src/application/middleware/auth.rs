
use axum::{body::{Body}, extract::Request, http::{header::{AUTHORIZATION}, StatusCode}, middleware::Next, response::{IntoResponse, Response}, Json};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub async fn authorization_middleware(mut req:Request, next:Next) ->  Result<Response,AuthError>{
    let auth_header = req.headers_mut().get(AUTHORIZATION);
    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_|{
            AuthError{
                message : "Invalid Token".to_string(),
                status_code:StatusCode::UNAUTHORIZED
            }
        })?,
        None => return Err(
            AuthError{
                message : "Not Authorized".to_string(),
                status_code:StatusCode::UNAUTHORIZED
            }
        )
    };
    let mut header = auth_header.split_whitespace();
    let (bearer, token) = (header.next(),header.next());

    if bearer != Some("Bearer") || token.is_none() {
        return Err(AuthError {
            message: "Invalid Token Format".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        });
    }
    let token_data = match decode_jwt(token.unwrap().to_string()){
        Ok(data)=> data,
        Err(_)=> return Err(
            AuthError{
                message : "Invalid Token".to_string(),
                status_code:StatusCode::UNAUTHORIZED
            }
        )
    };
    let claims = token_data.claims;

    // Attach the user ID to the request extensions
    req.extensions_mut().insert(claims.id);
    Ok(next.run(req).await)
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,  // Expiry time of the token
    pub iat: usize,  // Issued at time of the token
    pub id: u32,  // Email associated with the token
}

pub struct AuthError {
    message: String,
    status_code: StatusCode,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response<Body> {
        let body = Json(json!({
            "error": self.message,
        }));

        (self.status_code, body).into_response()
    }
}


pub fn encode_jwt (id:u32) -> Result<String,StatusCode>{
    let jwt_token: String = "randomstring".to_string();
    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::days(31);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;
    let claim = Claims{iat,exp,id};
    let secret = jwt_token.clone();
    encode(&Header::default(), &claim, &EncodingKey::from_secret(secret.as_ref())
    ) .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}


pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>, StatusCode> {
    let secret = "randomstring".to_string();

    let result: Result<TokenData<Claims>, StatusCode> = decode(
        &jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    result
}

