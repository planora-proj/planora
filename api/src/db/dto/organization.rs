use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateOrg {
    pub name: String,
    pub subdomain: String,
}
