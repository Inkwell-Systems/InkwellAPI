use crate::domain::{DisplayName, Email, SignUpRequest};
use uuid::Uuid;

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
            profile_url: i.profile_url,
        }
    }
}

impl TryFrom<SignUpRequest> for UserIncomplete {
    type Error = String;

    fn try_from(value: SignUpRequest) -> Result<Self, Self::Error> {
        let display_name = DisplayName::new(value.display_name)?;
        let email = Email::new(value.email)?;

        Ok(UserIncomplete {
            email,
            display_name,
            profile_url: value.profile_url,
        })
    }
}
