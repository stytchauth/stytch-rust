// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use serde::{Deserialize, Serialize};

/// Attributes:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attributes {
    /// ip_address: The IP address of the user.
    pub ip_address: std::option::Option<String>,
    /// user_agent: The user agent of the User.
    pub user_agent: std::option::Option<String>,
}
