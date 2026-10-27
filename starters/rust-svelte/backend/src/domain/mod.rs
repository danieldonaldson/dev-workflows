//! Domain layer — auth entities, value objects, errors.

pub mod entities;
pub mod errors;
pub mod services;
pub mod validation;
pub mod value_objects;

#[cfg(test)]
pub mod test_helpers;
