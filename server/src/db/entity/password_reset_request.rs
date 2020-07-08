use crate::db::*;
use sha2::{Digest, Sha256};

#[derive(Debug, Deserialize, Serialize, PartialEq, Setters, CopyGetters)]
#[serde(rename_all = "camelCase")]
pub struct PasswordResetRequest {
  #[getset(get_copy = "pub with_prefix", set = "pub")]
  pub id: i64,
  pub user_id: i64,
  pub token_encrypted: String,
  pub published: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordResetRequestForm {
  pub user_id: i64,
  pub token_encrypted: String,
}

impl From<PasswordResetRequestForm> for PasswordResetRequest {
  fn from(form: PasswordResetRequestForm) -> Self {
      PasswordResetRequest {
        id: 0,
        user_id: form.user_id,
        token_encrypted: form.token_encrypted,
        published: chrono::Utc::now().naive_utc()
      }
  }
}

impl Node for PasswordResetRequest {
  fn db_type_name() -> &'static str {
    PasswordResetRequest::GDB_TYPE
  }
}


// impl CrudNode<PasswordResetRequestForm> for PasswordResetRequest {}

impl PasswordResetRequest {

  /// Dgraph type
  const GDB_TYPE: &'static str = "PasswordResetRequest";

  /**
   * Create request from token
   */
  pub async fn create_token(
      conn: &dgraph::Client,
      from_user_id: i64,
      token: &str
    ) -> Result<Self, Error> {

    let mut hasher = Sha256::new();
    hasher.input(token);
    let token_hash: String = PasswordResetRequest::bytes_to_hex(
                                hasher.result().to_vec());

    let mut form = PasswordResetRequest {
      id: 0,
      user_id: from_user_id,
      token_encrypted: token_hash,
      published: chrono::Utc::now().naive_utc(),
    };

    let uid = create_node::<PasswordResetRequest>(conn, &mut form).await?;
    Ok(form)
  }

  /**
   * Read request from token
   */
  pub async fn read_from_token(conn: &dgraph::Client, token: &str) -> Result<Self, Error> {
    let mut hasher = Sha256::new();
    hasher.input(token);
    let token_hash: String = PasswordResetRequest::bytes_to_hex(
                                hasher.result().to_vec());

    let pred_name = "tokenEncrypted";
    let pred_repr = format!("{:?}", token);

    let nodes = find_nodes::<PasswordResetRequest>(
      conn, pred_name, &pred_repr).await?;

    if nodes.len() != 1 {
      failure::bail!(format!("Expected 1 node but found {}", nodes.len()));
    }
    Ok(nodes[0])
  }

  fn bytes_to_hex(bytes: Vec<u8>) -> String {
    let mut str = String::new();
    for byte in bytes {
      str = format!("{}{:02x}", str, byte);
    }
    str
  }
}

// #[cfg(test)]
// mod tests {
//   use super::super::user::*;
//   use super::*;

//   #[test]
//   fn test_crud() {
//     let conn = dgraph::Client::new(vec!["http://localhost:9080"])
//                   .expect("connected client");

//     let new_user = UserForm {
//       name: "thommy prw".into(),
//       fedi_name: "rrf".into(),
//       preferred_username: None,
//       password_encrypted: "nope".into(),
//       email: None,
//       matrix_user_id: None,
//       avatar: None,
//       admin: false,
//       banned: false,
//       updated: None,
//       show_nsfw: false,
//       theme: "darkly".into(),
//       default_sort_type: SortType::Hot as i16,
//       default_listing_type: ListingType::Subscribed as i16,
//       lang: "browser".into(),
//       show_avatars: true,
//       send_notifications_to_email: false,
//     };

//     let inserted_user = User_::create(&conn, &new_user).unwrap();

//     let token = "nope";
//     let token_encrypted_ = "ca3704aa0b06f5954c79ee837faa152d84d6b2d42838f0637a15eda8337dbdce";

//     let inserted_password_reset_request =
//       PasswordResetRequest::create_token(&conn, inserted_user.id, token).unwrap();

//     let expected_password_reset_request = PasswordResetRequest {
//       id: inserted_password_reset_request.id,
//       user_id: inserted_user.id,
//       token_encrypted: token_encrypted_.to_string(),
//       published: inserted_password_reset_request.published,
//     };

//     let read_password_reset_request = PasswordResetRequest::read_from_token(&conn, token).unwrap();
//     let num_deleted = User_::delete(&conn, inserted_user.id).unwrap();

//     assert_eq!(expected_password_reset_request, read_password_reset_request);
//     assert_eq!(
//       expected_password_reset_request,
//       inserted_password_reset_request
//     );
//     assert_eq!(1, num_deleted);
//   }
// }
