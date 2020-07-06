use super::*;
use crate::schema::password_reset_request;
use crate::schema::password_reset_request::dsl::*;
use sha2::{Digest, Sha256};

#[derive(PartialEq, Debug)]
pub struct PasswordResetRequest {
  pub id: i64,
  pub user_id: i64,
  pub token_encrypted: String,
  pub published: chrono::NaiveDateTime,
}

#[derive(Clone)]
pub struct PasswordResetRequestForm {
  pub user_id: i64,
  pub token_encrypted: String,
}

impl CrudNode<PasswordResetRequestForm, dgraph::Client> for PasswordResetRequest {
  fn read(conn: &dgraph::Client, password_reset_request_id: i64) -> Result<Self> {
    use crate::schema::password_reset_request::dsl::*;
    password_reset_request
      .find(password_reset_request_id)
      .first::<Self>(conn)
  }
  fn delete(conn: &dgraph::Client, password_reset_request_id: i64) -> Result<usize> {
    diesel::delete(password_reset_request.find(password_reset_request_id)).execute(conn)
  }
  fn create(conn: &dgraph::Client, form: &PasswordResetRequestForm) -> Result<Self> {
    insert_into(password_reset_request)
      .values(form)
      .get_result::<Self>(conn)
  }
  fn update(
    conn: &dgraph::Client,
    password_reset_request_id: i64,
    form: &PasswordResetRequestForm,
  ) -> Result<Self> {
    diesel::update(password_reset_request.find(password_reset_request_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

impl PasswordResetRequest {
  pub fn create_token(conn: &dgraph::Client, from_user_id: i64, token: &str) -> Result<Self> {
    let mut hasher = Sha256::new();
    hasher.input(token);
    let token_hash: String = PasswordResetRequest::bytes_to_hex(hasher.result().to_vec());

    let form = PasswordResetRequestForm {
      user_id: from_user_id,
      token_encrypted: token_hash,
    };

    Self::create(&conn, &form)
  }
  pub fn read_from_token(conn: &dgraph::Client, token: &str) -> Result<Self> {
    let mut hasher = Sha256::new();
    hasher.input(token);
    let token_hash: String = PasswordResetRequest::bytes_to_hex(hasher.result().to_vec());
    password_reset_request
      .filter(token_encrypted.eq(token_hash))
      .filter(published.gt(now - 1.days()))
      .first::<Self>(conn)
  }

  fn bytes_to_hex(bytes: Vec<u8>) -> String {
    let mut str = String::new();
    for byte in bytes {
      str = format!("{}{:02x}", str, byte);
    }
    str
  }
}

#[cfg(test)]
mod tests {
  use super::super::user::*;
  use super::*;

  #[test]
  fn test_crud() {
    let conn = establish_unpooled_connection();

    let new_user = UserForm {
      name: "thommy prw".into(),
      fedi_name: "rrf".into(),
      preferred_username: None,
      password_encrypted: "nope".into(),
      email: None,
      matrix_user_id: None,
      avatar: None,
      admin: false,
      banned: false,
      updated: None,
      show_nsfw: false,
      theme: "darkly".into(),
      default_sort_type: SortType::Hot as i16,
      default_listing_type: ListingType::Subscribed as i16,
      lang: "browser".into(),
      show_avatars: true,
      send_notifications_to_email: false,
    };

    let inserted_user = User_::create(&conn, &new_user).unwrap();

    let token = "nope";
    let token_encrypted_ = "ca3704aa0b06f5954c79ee837faa152d84d6b2d42838f0637a15eda8337dbdce";

    let inserted_password_reset_request =
      PasswordResetRequest::create_token(&conn, inserted_user.id, token).unwrap();

    let expected_password_reset_request = PasswordResetRequest {
      id: inserted_password_reset_request.id,
      user_id: inserted_user.id,
      token_encrypted: token_encrypted_.to_string(),
      published: inserted_password_reset_request.published,
    };

    let read_password_reset_request = PasswordResetRequest::read_from_token(&conn, token).unwrap();
    let num_deleted = User_::delete(&conn, inserted_user.id).unwrap();

    assert_eq!(expected_password_reset_request, read_password_reset_request);
    assert_eq!(
      expected_password_reset_request,
      inserted_password_reset_request
    );
    assert_eq!(1, num_deleted);
  }
}
