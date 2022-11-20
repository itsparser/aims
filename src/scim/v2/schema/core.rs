//! All the Basic Field - https://www.rfc-editor.org/rfc/rfc7643.html#section-7


use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Uniqueness {
    None,
    Server,
    Global,
}


impl Default for Uniqueness {
    fn default() -> Self {
        Uniqueness::None
    }
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug, Copy)]
#[serde(rename_all = "camelCase")]
pub enum AttributeType {
    String,
    Boolean,
    Decimal,
    Integer,
    DateTime,
    Reference,
    Complex,
}

impl Default for AttributeType {
    fn default() -> Self {
        AttributeType::String
    }
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug, Copy)]
#[serde(rename_all = "camelCase")]
pub enum CanonicalValue {
    Work,
    Home,
    Mobile,
    Fax,
    Pager,
    Other,
    Aim,
    Gtalk,
    Icq
}

impl Default for CanonicalValue {
    fn default() -> Self {
        CanonicalValue::Work
    }
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Mutability {
    ReadOnly,
    ReadWrite,
    Immutable,
    WriteOnly,
}

impl Default for Mutability {
    fn default() -> Self {
        Mutability::ReadOnly
    }
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Attribute - is definition to individual Attribute Config
pub struct Attribute {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: AttributeType,
    pub multi_valued: bool,
    pub description: String,
    pub required: bool,
    pub case_exact: Option<bool>,
    pub mutability: Mutability,
    pub returned: String,
    pub uniqueness: Option<Uniqueness>,
    #[serde(default)]
    pub sub_attributes: Option<Vec<Attribute>>,
    #[serde(default)]
    pub canonical_values: Option<Vec<String>>,
    pub reference_types: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub resource_type: String,
    pub location: String,
}


#[cfg(test)]
mod tests {
    use crate::scim::v2::schema::core::AttributeType;

    #[test]
    fn serialization() {
        let tag = serde_json::to_string(&AttributeType::DateTime).unwrap();
        assert_eq!(tag.to_string(), "\"dateTime\"".to_string());
    }
}
