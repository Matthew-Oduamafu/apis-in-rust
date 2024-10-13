use rocket::http::Status;
use rocket::request::{FromRequest, Request, Outcome};

#[derive(Debug)]
pub struct BasicAuth {
    username: String,
    password: String,
}

impl BasicAuth {

    fn from_authorization_header(header: &str) -> Option<BasicAuth> {
        let split = header.split_whitespace().collect::<Vec<_>>();
        if split.len() != 2 {
            return None
        }

        if split[0] != "Basic" {
            return None
        }

        Self::from_base64_encoded(split[1])
    }

    fn from_base64_encoded(encoded: &str) -> Option<BasicAuth> {
        let decoded = base64::decode(encoded).ok()?;
        let decoded_str = String::from_utf8(decoded).ok()?;
        let split = decoded_str.split(":").collect::<Vec<_>>();
        if split.len() != 2 {
            return None
        }

        Some(BasicAuth {
            username: split[0].to_string(),
            password: split[1].to_string(),
        })
    }
}

#[rocket::async_trait]

impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        if let Some(header) = auth_header {
            if let Some(auth) = BasicAuth::from_authorization_header(header) {
                return Outcome::Success(auth)
            }
        }

        Outcome::Forward(Status::Unauthorized)
    }
}
