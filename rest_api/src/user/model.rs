use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct User {
  pub id: i32,
  pub email: String,
}
