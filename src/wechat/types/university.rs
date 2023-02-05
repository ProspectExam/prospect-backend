use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use crate::wechat::types::Error;

#[derive(Deserialize, Serialize, Debug)]
pub struct UniversityResult {
  pub err_code: i32,
  pub message: String,
  pub universities: HashMap<u32, String>,
}

impl UniversityResult {
  pub fn new(arg: Result<HashMap<u32, String>, Error>) -> Self {
    match arg {
      Ok(hashmap) => UniversityResult {
        err_code: Error::Success.into(),
        message: Error::Success.into(),
        universities: hashmap,
      },
      Err(e) => UniversityResult {
        err_code: e.into(),
        message: e.into(),
        universities: HashMap::new(),
      },
    }
  }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetDepartmentInfo {
  pub university_code: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DepartmentResult {
  pub err_code: i32,
  pub message: String,
  pub departments: HashMap<u32, String>,
}

impl DepartmentResult {
  pub fn new(arg: Result<HashMap<u32, String>, Error>) -> Self {
    match arg {
      Ok(hashmap) => DepartmentResult {
        err_code: Error::Success.into(),
        message: Error::Success.into(),
        departments: hashmap,
      },
      Err(e) => DepartmentResult {
        err_code: e.into(),
        message: e.into(),
        departments: HashMap::new(),
      },
    }
  }
}

#[derive(Clone)]
pub struct UniversityContext {
  pub university_id: u32,
  pub university_name: String,
  pub department_id: u32,
  pub department_name: String,
}
