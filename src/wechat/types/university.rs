use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UniversityResult {
  pub universities: HashMap<String, u32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetDepartmentInfo {
  pub university_code: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DepartmentInfo {
  pub departments: HashMap<String, u32>,
}
