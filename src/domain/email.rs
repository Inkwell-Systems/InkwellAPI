use regex::Regex;

pub struct Email(String);

impl Email {
    pub fn new(s: String) -> Result<Email, &'static str> {
        if !is_valid_email(&s) {
            return Err("Email is invalid.");
        }

        Ok(Email(s))
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsMut<str> for Email {
    fn as_mut(&mut self) -> &mut str {
        &mut self.0
    }
}

// TAKEN FROM
// https://docs.rs/validator/latest/src/validator/validation/email.rs.html
fn is_valid_email(email: &str) -> bool {
    if email.is_empty() || !email.contains('@') {
        return false;
    }

    let parts: Vec<&str> = email.rsplitn(2, '@').collect();
    let user_part = parts[1];
    let domain_part = parts[0];
    if user_part.len() > 64 || domain_part.len() > 255 {
        return false;
    }

    let user_re: Regex =
        Regex::new(r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+\z").unwrap();
    if !user_re.is_match(user_part) {
        return false;
    }

    let domain_re: Regex = Regex::new(
        r"(?i)^[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)*$"
    ).unwrap();
    if !domain_re.is_match(domain_part) {
        return false;
    }

    true
}
