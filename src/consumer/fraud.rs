// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::consumer::fraud_fingerprint::Fingerprint;
use crate::consumer::fraud_rules::Rules;
use serde::{Deserialize, Serialize};

/// ASNProperties:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ASNProperties {
    /// asn: The Autonomous System Number of the user's network.
    pub asn: String,
    /// name: Public name associated with the ASN.
    pub name: String,
    /// network: The CIDR block associated with the ASN.
    pub network: String,
}
/// BrowserProperties:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BrowserProperties {
    /// user_agent: The user agent of the user's browser.
    pub user_agent: String,
}
/// Fingerprints:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fingerprints {
    /// network_fingerprint: Combination of signals associated with a specific network commonly known as TLS
    /// fingerprinting.
    pub network_fingerprint: String,
    /// hardware_fingerprint: Combinations of signals to identify an operating system and architecture.
    pub hardware_fingerprint: String,
    /// browser_fingerprint: Combination of signals to identify a browser and its specific version.
    pub browser_fingerprint: String,
    /// visitor_fingerprint: Cookie-less way of identifying a unique user.
    pub visitor_fingerprint: String,
    /// visitor_id: The cookie stored on the user's device that uniquely identifies them.
    pub visitor_id: std::option::Option<String>,
    /// browser_id: Combination of VisitorID and NetworkFingerprint to create a clear identifier of a browser.
    pub browser_id: std::option::Option<String>,
}
/// IPGeoProperties:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IPGeoProperties {
    /// city: The city where the IP is located.
    pub city: String,
    /// region: The region where the IP is located.
    pub region: String,
    /// country: The country where the IP is located.
    pub country: String,
}
/// Metadata:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    /// external_id: An external ID, such as a user ID, that you wish to associate with the telemetry ID.
    pub external_id: std::option::Option<String>,
    /// organization_id: The organization ID you wish to associate with the telemetry ID.
    pub organization_id: std::option::Option<String>,
    /// user_action: The user action, such as 'login', that you wish to associate with the telemetry ID.
    pub user_action: std::option::Option<String>,
}
/// NetworkProperties:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkProperties {
    /// ip_address: The IP address of the user.
    pub ip_address: String,
    /// asn: Information about the network's ASN (Autonomous System Number).
    pub asn: ASNProperties,
    /// ip_geolocation: Information about the geolocation of the user's IP address.
    pub ip_geolocation: IPGeoProperties,
    /// is_proxy: Whether the user is using a proxy.
    pub is_proxy: bool,
    /// is_vpn: Whether the user is using a VPN.
    pub is_vpn: bool,
}
/// Properties:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Properties {
    pub network_properties: NetworkProperties,
    pub browser_properties: BrowserProperties,
}
/// Verdict:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Verdict {
    /// action: The suggested action based on the fingerprint review. The available actions are:
    ///   * `ALLOW` - This is a known valid device grouping or device profile that is part of the default ALLOW
    /// listed set of known devices by Stytch. This grouping is made up of verified device profiles that match
    /// the characteristics of known/authentic traffic origins
    ///   * `BLOCK` - This is a known bad or malicious device profile that is undesirable and should be blocked
    /// from completing the privileged action in question
    ///   * `CHALLENGE` - This is an unknown or potentially malicious device that should be put through
    /// increased friction such as 2FA or other forms of extended user verification before allowing the
    /// privileged action to proceed
    ///
    pub action: VerdictAction,
    /// reasons: A set of contextual clues to inform why a `CHALLENGE` or `BLOCK` action was suggested. For a
    /// list of possible Reasons, please [contact support](mailto:support@stytch.com).
    pub reasons: std::vec::Vec<String>,
    /// detected_device_type: The operating system and architecture that took the fingerprint.
    pub detected_device_type: String,
    /// is_authentic_device: The assessment of whether this is an authentic device. It will be false if hardware
    /// or browser deception is detected.
    pub is_authentic_device: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum RuleAction {
    #[serde(rename = "ALLOW")]
    #[default]
    ALLOW,
    #[serde(rename = "CHALLENGE")]
    CHALLENGE,
    #[serde(rename = "BLOCK")]
    BLOCK,
    #[serde(rename = "NONE")]
    NONE,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum VerdictAction {
    #[serde(rename = "ALLOW")]
    #[default]
    ALLOW,
    #[serde(rename = "CHALLENGE")]
    CHALLENGE,
    #[serde(rename = "BLOCK")]
    BLOCK,
}

pub struct Fraud {
    pub fingerprint: Fingerprint,
    pub rules: Rules,
}

impl Fraud {
    pub fn new(http_client: crate::client::Client) -> Self {
        Self {
            fingerprint: Fingerprint::new(http_client.clone()),
            rules: Rules::new(http_client.clone()),
        }
    }
}