use std::collections::HashMap;

struct UniversityResult {
  universities: HashMap<String, u32>,
}

struct GetDepartmentInfo {
  university_code: u32,
}

struct DepartmentInfo {
  departments: HashMap<String, u32>,
}
