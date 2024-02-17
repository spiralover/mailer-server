use validator::ValidationError;

pub fn make_validation_message(
    state: bool,
    entity: &str,
    variants: &str,
) -> Result<(), ValidationError> {
    match state {
        true => Ok(()),
        false => {
            let msg = format!("{} can only be one of ({})", entity, variants);
            Err(ValidationError::new(Box::leak(Box::new(msg))))
        }
    }
}
