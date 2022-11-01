use std::collections::HashMap;

pub struct UniversityResult {
  pub universities: HashMap<String, u32>,
}

pub struct GetDepartmentInfo {
  pub university_code: u32,
}

pub struct DepartmentInfo {
  pub departments: HashMap<String, u32>,
}
