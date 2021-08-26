use std::{ path::PathBuf};
use postman_collection::PostmanCollection;
use anyhow::Result;
use thiserror::Error;

#[derive(Error,Debug)]
pub enum RestopLibError {
   #[error(transparent)]
   PostmanError(#[from] postman_collection::errors::Error)
}

pub fn spec(path:PathBuf)-> Result<PostmanCollection,RestopLibError>{
  match postman_collection::from_path(path) {
    Ok(spec) => Ok(spec),
    Err(e)=> Err(RestopLibError::PostmanError(e))
  }
}