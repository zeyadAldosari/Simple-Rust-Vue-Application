use hyper::StatusCode;


pub fn hash_password(password: String) -> Result<String, StatusCode> {
    bcrypt::hash(password, 12).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn verify_password(password: String, hash:&str) -> Result<bool, StatusCode> {
    bcrypt::verify(password, hash).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}
