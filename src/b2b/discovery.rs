// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::b2b::discovery_intermediate_sessions::IntermediateSessions;
use crate::b2b::discovery_organizations::Organizations;
use crate::b2b::mfa::MfaRequired;
use crate::b2b::organizations::Member;
use crate::b2b::organizations::Organization;
use crate::b2b::sessions::PrimaryRequired;
use serde::{Deserialize, Serialize};

/// DiscoveredOrganization:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscoveredOrganization {
    /// member_authenticated: Indicates whether the Member has all of the factors needed to fully authenticate
    /// to this Organization. If false, the Member may need to complete an MFA step or complete a different
    /// primary authentication flow. See the `primary_required` and `mfa_required` fields for more details on
    /// each.
    pub member_authenticated: bool,
    /// organization: The [Organization object](https://stytch.com/docs/b2b/api/organization-object).
    pub organization: std::option::Option<Organization>,
    /// membership: Information about the membership.
    pub membership: std::option::Option<Membership>,
    /// primary_required: Information about the primary authentication requirements of the Organization.
    pub primary_required: std::option::Option<PrimaryRequired>,
    /// mfa_required: Information about the MFA requirements of the Organization and the Member's options for
    /// fulfilling MFA.
    pub mfa_required: std::option::Option<MfaRequired>,
}
/// Membership:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Membership {
    /// type_: Either `active_member`, `pending_member`, `invited_member`, `eligible_to_join_by_email_domain`,
    /// or `eligible_to_join_by_oauth_tenant`
    #[serde(rename = "type")]
    pub type_: String,
    /// details: An object containing additional metadata about the membership, if available.
    pub details: std::option::Option<serde_json::Value>,
    /// member: The [Member object](https://stytch.com/docs/b2b/api/member-object) if one already exists, or
    /// null if one does not.
    pub member: std::option::Option<Member>,
}

pub struct Discovery {
    pub intermediate_sessions: IntermediateSessions,
    pub organizations: Organizations,
}

impl Discovery {
    pub fn new(http_client: crate::client::Client) -> Self {
        Self {
            intermediate_sessions: IntermediateSessions::new(http_client.clone()),
            organizations: Organizations::new(http_client.clone()),
        }
    }
}
