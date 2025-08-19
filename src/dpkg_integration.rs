use dpkg_query_json::dpkg_list_packages::DpkgListPackages;
use dpkg_query_json::dpkg_options::DpkgOptions;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
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

pub fn create_package_list(root_dir_str: &str) -> serde_json::Result<Vec<Package>> {
    let mut dpkg_list_packages_operation = DpkgListPackages::new(
        vec![
            String::from("Package"),
            String::from("Version"),
            //String::from("Status")
        ],
        vec![]);
    dpkg_list_packages_operation.set_options(DpkgOptions::new().set_root_dir(root_dir_str.to_string()));

    let packages_raw = dpkg_list_packages_operation.json_string();
    serde_json::from_str(packages_raw.to_string().as_str())
}
