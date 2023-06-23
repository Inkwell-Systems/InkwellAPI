use uuid::Uuid;
use crate::domain::{DisplayName, Email};

pub struct User {
    pub uuid: Uuid,
    pub email: Email,
    pub display_name: DisplayName,
    pub profile_url: String,
}

pub struct UserIncomplete {
    pub email: Email,
    pub display_name: DisplayName,
    pub profile_url: String,
}

impl User {
    pub fn new(i: UserIncomplete, uuid: Uuid) -> User {
        User {
            uuid,
            email: i.email,
            display_name: i.display_name,
            profile_url: i.profile_url
        }
    }
}

impl UserIncomplete {
    pub fn parse(email: String, display_name: String, profile_url: String) -> Result<UserIncomplete, String> {
        let display_name = DisplayName::new(display_name)?;
        let email = Email::new(email)?;
        
        Ok(UserIncomplete {
            email,
            display_name,
            profile_url
        })
    }
}