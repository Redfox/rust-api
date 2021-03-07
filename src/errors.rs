use validator::{ Validate, ValidationError, ValidationErrors };

#[derive(Debug)]
pub struct Errors {
  errors: ValidationErrors,
}

pub type FieldName = &'static str;
pub type FieldErrorCode = &'static str;

impl Errors {
  pub fn new(errs: &[(FieldName, FieldErrorCode)]) -> Self {
    let mut errors = ValidationErrors::new();
    for (field, code) in errs {
      errors.add(field, ValidationError::new(code));
    }
    Self { errors }
  }
}

