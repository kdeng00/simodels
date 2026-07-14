use std::default::Default;

use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize, serde::Serialize)]
pub struct Token {
    pub scope: String,
    pub expiration: i64,
    pub audience: String,
    pub issuer: String,
    pub issued: i64,
}

#[derive(Clone, Debug, Deserialize, serde::Serialize)]
pub struct AccessToken {
    #[serde(alias = "init::is_uuid_nil")]
    pub user_id: uuid::Uuid,
    #[serde(alias = "username")]
    pub username: String,
    #[serde(alias = "token")]
    pub token: String,
    #[serde(alias = "token_type")]
    pub token_type: String,
    #[serde(alias = "expiration")]
    pub expiration: i64,
    #[serde(alias = "message")]
    pub message: String,
}

#[derive(Clone, Debug, serde::Serialize, Deserialize)]
pub struct UserClaims {
    pub iss: String,
    pub aud: String, // Audience
    pub sub: String, // Subject (user ID)
    #[serde(deserialize_with = "deserialize_i64_from_f64")]
    pub exp: i64, // Expiration time (UTC timestamp)
    #[serde(deserialize_with = "deserialize_i64_from_f64")]
    pub iat: i64, // Issued at (UTC timestamp)
    // pub azp: String,
    // pub gty: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>, // Optional roles
}

impl AccessToken {
    /// Get the token fit for Bearer authentication
    pub fn bearer_token(&self) -> String {
        format!("Bearer {}", self.token)
    }

    pub fn token_expired(&self) -> bool {
        let current_time = time::OffsetDateTime::now_utc();
        let expired = time::OffsetDateTime::from_unix_timestamp(self.expiration).unwrap();
        current_time > expired
    }
}

impl Token {
    pub fn _to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self)
    }

    pub fn token_expired(&self) -> bool {
        let current_time = time::OffsetDateTime::now_utc();
        let expired = time::OffsetDateTime::from_unix_timestamp(self.expiration).unwrap();
        current_time > expired
    }

    pub fn contains_scope(&self, des_scope: &String) -> bool {
        self.scope.contains(des_scope)
    }
}

fn deserialize_i64_from_f64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let val = f64::deserialize(deserializer)?;
    // Handle NaN and infinity cases
    if val.is_nan() || val.is_infinite() {
        return Err(serde::de::Error::custom("invalid float value"));
    }
    // Round to nearest integer and convert
    let rounded = val.round();
    // Check if the rounded value can fit in i64
    if rounded < (i64::MIN as f64) || rounded > (i64::MAX as f64) {
        Err(serde::de::Error::custom("float out of i64 range"))
    } else {
        Ok(rounded as i64)
    }
}

pub fn get_issued() -> time::Result<time::OffsetDateTime> {
    Ok(time::OffsetDateTime::now_utc())
}

mod util {
    pub fn time_to_std_time(
        provided_time: &time::OffsetDateTime,
    ) -> Result<std::time::SystemTime, std::time::SystemTimeError> {
        let converted = std::time::SystemTime::from(*provided_time);
        Ok(converted)
    }
}

#[derive(Debug)]
pub struct TokenResource {
    pub message: String,
    pub issuer: String,
    pub audiences: Vec<String>,
    pub id: uuid::Uuid,
}

pub const TOKEN_TYPE: &str = "JWT";

pub fn create_token(
    key: &String,
    token_resource: &TokenResource,
    duration: time::Duration,
) -> Result<(String, i64), josekit::JoseError> {
    let mut header = josekit::jws::JwsHeader::new();
    header.set_token_type(TOKEN_TYPE);

    let mut payload = josekit::jwt::JwtPayload::new();
    let message = &token_resource.message;
    let issuer = &token_resource.issuer;
    let audiences: &Vec<String> = &token_resource.audiences;
    payload.set_subject(message);
    payload.set_issuer(issuer);
    payload.set_audience(audiences.clone());
    if !token_resource.id.is_nil() {
        match payload.set_claim("id", Some(serde_json::json!(token_resource.id))) {
            Ok(_) => {}
            Err(err) => {
                return Err(err);
            }
        }
    }
    match get_issued() {
        Ok(issued) => {
            let expire = issued + duration;
            payload.set_issued_at(&util::time_to_std_time(&issued).unwrap());
            payload.set_expires_at(&util::time_to_std_time(&expire).unwrap());

            let signer = josekit::jws::alg::hmac::HmacJwsAlgorithm::Hs256
                .signer_from_bytes(key.as_bytes())
                .unwrap();
            Ok((
                josekit::jwt::encode_with_signer(&payload, &header, &signer).unwrap(),
                (expire - time::OffsetDateTime::UNIX_EPOCH).whole_seconds(),
            ))
        }
        Err(e) => Err(josekit::JoseError::InvalidClaim(e.into())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key() -> String {
        String::from("c3092urmc2219ix320i40m293ic29IM09IN0u879Y8B98YB8yb86TN7B55R4yv4RCVU6Bi8YO8U")
    }

    fn test_resource() -> TokenResource {
        TokenResource {
            issuer: String::from("soaricarus_auth_test"),
            message: String::from("Authorization"),
            audiences: vec![String::from("soaricarus_test")],
            id: uuid::Uuid::nil(),
        }
    }

    #[test]
    fn test_token_scope_check() {
        let mut token = Token::default();
        token.scope = String::from("song:read song:upload song:download");

        let check_scope = String::from("song:download");
        let result = token.contains_scope(&check_scope);

        assert!(
            result,
            "Error: The scope {:?} was not found in the token's scope {:?}",
            check_scope, token.scope
        );
    }

    #[test]
    fn test_token_creation() {
        let key = test_key();
        let test_token_resource = test_resource();
        let token_expiration_duration = time::Duration::hours(2);

        match create_token(&key, &test_token_resource, token_expiration_duration) {
            Ok((token, expire_duration)) => {
                assert_eq!(false, token.is_empty(), "Error: Token is empty");
                assert!(
                    expire_duration > 0,
                    "Token expire duration is invalid {expire_duration:?}"
                );
            }
            Err(err) => {
                assert!(false, "Error: {err:?}");
            }
        }
    }

    #[test]
    fn test_token_creation_with_id() {
        let key = test_key();
        let mut test_token_resource = test_resource();
        test_token_resource.id = uuid::Uuid::new_v4();
        let token_expiration_duration = time::Duration::hours(2);

        match create_token(&key, &test_token_resource, token_expiration_duration) {
            Ok((token, expire_duration)) => {
                assert_eq!(false, token.is_empty(), "Error: Token is empty");
                assert!(
                    expire_duration > 0,
                    "Token expire duration is invalid {expire_duration:?}"
                );
            }
            Err(err) => {
                assert!(false, "Error: {err:?}");
            }
        }
    }
}
