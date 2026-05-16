use birnio_core::{Environment, Variable};
use uuid::Uuid;

use crate::StorageResult;

#[derive(Clone, Debug)]
pub struct EnvironmentRecord {
    pub id: String,
    pub name: String,
    pub variables_json: String,
}

impl EnvironmentRecord {
    pub fn from_domain(environment: &Environment) -> StorageResult<Self> {
        Ok(Self {
            id: environment.id.to_string(),
            name: environment.name.clone(),
            variables_json: serde_json::to_string(&environment.variables)?,
        })
    }

    pub fn into_domain(self) -> StorageResult<Environment> {
        Ok(Environment {
            id: Uuid::parse_str(&self.id)?,
            name: self.name,
            variables: serde_json::from_str::<Vec<Variable>>(&self.variables_json)?,
        })
    }
}
