use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SendMessage {
  pub template_id: String,
  // pub page: String,
  /// user's open_id
  pub to_user: String,
  pub data: String,
  pub miniprogram_state: String,
  pub lang: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Value {
  value: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SendMessageResult {
  pub errcode: i32,
  pub errmsg: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SubscribeTemplate {
  // TODO:
}

impl SubscribeTemplate {
  pub fn new(university: String, department: String, date: String) -> Self {
    // TODO:
    SubscribeTemplate {}
  }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TestSendMessageTemplate {
  thing2: Value,
  date5: Value,
}

impl TestSendMessageTemplate {
  pub fn new(title: String, date: String) -> Self {
    TestSendMessageTemplate {
      thing2: Value { value: title },
      date5: Value { value: date },
    }
  }
}
