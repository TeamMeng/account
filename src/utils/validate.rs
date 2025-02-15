use regex::Regex;
use validator::ValidationError;

pub fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    let ret = Regex::new(r"^1[3456789]\d{9}$");

    match ret {
        Ok(r) => {
            if r.is_match(phone) {
                Ok(())
            } else {
                Err(ValidationError::new("10001"))
            }
        }
        Err(_) => Err(ValidationError::new("10001")),
    }
}
