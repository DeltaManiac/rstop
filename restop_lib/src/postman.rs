use anyhow::Result;
use postman_collection::PostmanCollection;
use std::path::PathBuf;
use std::string::ToString;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RestopLibError {
    #[error(transparent)]
    PostmanCollectionError(#[from] postman_collection::errors::Error),
    #[error(
        "Found document with spec `{0}`. rstop supports postman collections with spec ≥ 2.0.0 "
    )]
    UnsupportedSpecification(String),
}

#[derive(Debug)]
pub enum SpecVersion {
    V2_0_0,
    V2_1_0,
}

#[derive(Debug)]
pub struct Information {
    pub postman_id: Option<String>,
    pub description: Option<String>,
    pub name: String,
    pub schema: String,
    pub version: Option<String>,
}

impl From<postman_collection::v2_0_0::Information> for Information {
    fn from(info: postman_collection::v2_0_0::Information) -> Self {
        let description = match info.description {
            Some(desc) => Some(desc.as_string()),
            None => None,
        };
        let version = match info.version {
            Some(version) => Some(version.as_string()),
            None => None,
        };
        Information {
            postman_id: info.postman_id,
            description,
            name: info.name,
            schema: info.schema,
            version,
        }
    }
}

impl From<postman_collection::v2_1_0::Information> for Information {
    fn from(info: postman_collection::v2_1_0::Information) -> Self {
        let description = match info.description {
            Some(desc) => Some(desc.as_string()),
            None => None,
        };
        let version = match info.version {
            Some(version) => Some(version.as_string()),
            None => None,
        };
        Information {
            postman_id: info.postman_id,
            description,
            name: info.name,
            schema: info.schema,
            version,
        }
    }
}

trait Stringify {
    fn as_string(&self) -> String;
}

impl Stringify for postman_collection::v2_0_0::DescriptionUnion {
    fn as_string(&self) -> String {
        match self {
            postman_collection::v2_0_0::DescriptionUnion::Description(c) => "".to_string(),
            postman_collection::v2_0_0::DescriptionUnion::String(c) => c.to_owned(),
        }
    }
}

impl Stringify for postman_collection::v2_1_0::DescriptionUnion {
    fn as_string(&self) -> String {
        match self {
            postman_collection::v2_1_0::DescriptionUnion::Description(c) => "".to_string(),
            postman_collection::v2_1_0::DescriptionUnion::String(c) => c.to_owned(),
        }
    }
}

impl Stringify for postman_collection::v2_0_0::CollectionVersion {
    fn as_string(&self) -> String {
        match self {
            postman_collection::v2_0_0::CollectionVersion::CollectionVersionClass(c) => {
                "".to_string()
            }
            postman_collection::v2_0_0::CollectionVersion::String(c) => c.to_owned(),
        }
    }
}

impl Stringify for postman_collection::v2_1_0::CollectionVersion {
    fn as_string(&self) -> String {
        match self {
            postman_collection::v2_1_0::CollectionVersion::CollectionVersionClass(c) => {
                "".to_string()
            }
            postman_collection::v2_1_0::CollectionVersion::String(c) => c.to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct PostmanInfo {
    pub spec: SpecVersion,
    pub items: usize,
    pub events: usize,
    pub variables: usize,
    pub information: Information,
}

pub fn spec(path: PathBuf) -> Result<PostmanInfo, RestopLibError> {
    match postman_collection::from_path(path) {
        Ok(spec) => match spec {
            PostmanCollection::V1_0_0(_) => Err(RestopLibError::UnsupportedSpecification(
                "v 1.0.0".to_string(),
            )),
            PostmanCollection::V2_0_0(spec) => {
                let events = match spec.event {
                    Some(v) => v.len(),
                    None => 0,
                };
                let variables = match spec.variable {
                    Some(v) => v.len(),
                    None => 0,
                };
                let information = Information::from(spec.info);
                Ok(PostmanInfo {
                    spec: SpecVersion::V2_0_0,
                    items: spec.item.len(),
                    events,
                    variables,
                    information,
                })
            }
            PostmanCollection::V2_1_0(spec) => {
                let events = match spec.event {
                    Some(v) => v.len(),
                    None => 0,
                };
                let variables = match spec.variable {
                    Some(v) => v.len(),
                    None => 0,
                };
                let information = Information::from(spec.info);
                Ok(PostmanInfo {
                    spec: SpecVersion::V2_1_0,
                    items: spec.item.len(),
                    events,
                    variables,
                    information,
                })
            }
        },
        Err(e) => Err(RestopLibError::PostmanCollectionError(e)),
    }
}
