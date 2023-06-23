use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SignUpRequest {
    pub display_name: String,
    pub email: String,
    pub profile_url: String,
}
