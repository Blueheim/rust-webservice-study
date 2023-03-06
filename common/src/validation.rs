use validator::ValidationError;

/// Check password format helper
/// Minimum eight characters, at least one upper case letter, one lower case letter, one number and one punctuation ascii character
fn check_password_complexity(password: &str) -> bool {
    let mut has_number = false;
    let mut has_uppercase = false;
    let mut has_lowercase = false;
    let mut has_punctuation = false;

    if password.len() < 8 || !password.is_ascii() {
        return false;
    }

    for c in password.chars() {
        if c.is_ascii_alphabetic() && !c.is_uppercase() {
            has_lowercase = true;
        }
        if c.is_ascii_alphabetic() && c.is_uppercase() {
            has_uppercase = true;
        }
        if c.is_ascii_digit() {
            has_number = true;
        }
        if c.is_ascii_punctuation() {
            has_punctuation = true;
        }
    }

    return has_lowercase && has_uppercase && has_number && has_punctuation;
}

/// Validator for password
/// To be used with the validator crate
pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    match check_password_complexity(password) {
        true => Ok(()),
        false => return Err(ValidationError::new("password_complexity")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_password_complexity() {
        assert_eq!(check_password_complexity("Aaδ1:M7"), false);
        assert_eq!(check_password_complexity("Aaδ1:M78"), false);
        assert_eq!(check_password_complexity("AaB1M78"), false);
        assert_eq!(check_password_complexity("Aab1M78o"), false);
        assert_eq!(check_password_complexity("aab1m78o"), false);
        assert_eq!(check_password_complexity("aAb1m7:"), false);
        assert_eq!(check_password_complexity("aAb1m7:/"), true);
        assert_eq!(check_password_complexity("aAb1m7:|"), true);
        assert_eq!(check_password_complexity("aAb1m7:|u?7"), true);
    }
}
