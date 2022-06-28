//! Validator: trivial input validation
//! Also: play around with tap
use log::info;
use serde::Deserialize;
use std::error::Error;

// A trait that the Validate derive will impl
use validator::{Validate, ValidationError, ValidationErrors};

// Deserialize for the firstname serde macro
// validator = { version = "0.15", features = ["derive","phone"] }
#[derive(Debug, Default, Validate, Deserialize)]
struct SignupData {
    #[validate(email)]
    mail: String,
    #[validate(phone)]
    phone: String,
    #[validate(url)]
    site: String,
    #[validate(length(min = 1), custom = "validate_unique_username")]
    #[serde(rename = "firstName")]
    first_name: String,
    #[validate(range(min = 18, max = 20))]
    age: u32,
}

fn validate_unique_username(username: &str) -> Result<(), ValidationError> {
    if username == "xXxShad0wxXx" {
        // the value of the username will automatically be added later
        return Err(ValidationError::new("terrible_username"));
    }

    Ok(())
}

// static LOGGER: SimpleLogger = SimpleLogger;
pub fn init_log() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
}

use tap::prelude::*;
fn main() -> Result<(), Box<dyn Error>> {
    init_log();
    let signup_data = SignupData::default();
    // works:
    signup_data.validate()?;
    // or, in one line:
    let signup_data = SignupData::default().pipe(|data| data.validate().and(Ok(data)))?;

    let v_sorted = vec![2, 5, 3]
        .tap_mut(|v| v.sort())
        .tap(|v| info!("sorted: {v:?}"));
    // instead of this
    let mut v = vec![2, 5, 3];
    v.sort();
    info!("sorted: {v:?}");
    assert_eq!(v, v_sorted);

    Ok(())
}
