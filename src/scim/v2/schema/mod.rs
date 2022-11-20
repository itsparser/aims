use serde::{Deserialize, Serialize};

use crate::scim::v2::schema::core::{Attribute, Meta};
use crate::scim::v2::schema::user::USER;

pub mod core;
mod user;

#[derive(Debug)]
pub enum SchemaType {
    ServiceProviderConfig,
    ResourceType,
    Schema,
    User,
    EnterpriseUser,
    Group,
    ListResponse,
    SearchRequest,
    PatchOp,
    BulkRequest,
    BulkResponse,
    Error,
}



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreSchema {
    pub id: String,
    pub name: String,
    pub description: String,
    pub attributes: Vec<Attribute>,
    pub meta: Meta,
}

impl CoreSchema {
    pub fn load(schema: SchemaType) -> Self {
        let result_schema = match schema {
            SchemaType::User => serde_json::from_str::<CoreSchema>(USER),
            _ => serde_json::from_str::<CoreSchema>(USER)
            // SchemaType::ServiceProviderConfig => {}
            // SchemaType::ResourceType => {}
            // SchemaType::Schema => {}
            // SchemaType::EnterpriseUser => {}
            // SchemaType::Group => {}
            // SchemaType::ListResponse => {}
            // SchemaType::SearchRequest => {}
            // SchemaType::PatchOp => {}
            // SchemaType::BulkRequest => {}
            // SchemaType::BulkResponse => {}
            // SchemaType::Error => {}
        }.expect("Error parsing schema");
        result_schema
    }
}


#[cfg(test)]
mod tests {
    use crate::scim::v2::schema::{CoreSchema, SchemaType};

    #[test]
    fn serialization() {
        let tag = CoreSchema::load(SchemaType::User).expect("Error Panics");
        println!("{:?}", tag)
    }
}