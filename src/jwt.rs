use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Claims {
    /// 该JWT的主题，即它所代表的实体，可以是用户、设备或其他任何东西。
    pub sub: String,
    /// 该JWT所代表的实体的公司或组织。
    pub company: String,
    /// 该JWT的过期时间，以Unix时间戳表示。
    pub exp: u64,
}

pub struct JWT;
impl JWT {
    pub fn encode(claims: &Claims, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let token = encode(
            &Header::default(),
            claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )?;
        Ok(token)
    }
    pub fn decode(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )?;
        Ok(token_data.claims)
    }
}
