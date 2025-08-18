use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Package {
    package: String,
    version: String,
}

impl Package {
    pub(crate) fn get_package(&self) -> &String {
        &self.package
    }
    pub(crate) fn get_version(&self) -> &String {
        &self.version
    }
}