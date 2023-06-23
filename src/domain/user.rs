use uuid::Uuid;
use crate::domain::DisplayName;

pub struct User {
    pub uuid: Uuid,
    pub email: String,
    pub display_name: DisplayName,
    pub profile_url: String,
}

pub struct UserIncomplete {
    pub email: String,
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