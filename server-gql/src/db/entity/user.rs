use crate::db::*;
use crate::{is_email_regex, Settings};
use bcrypt::{hash, DEFAULT_COST};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};

#[derive(PartialEq, Debug, Serialize, Deserialize, Setters, CopyGetters)]
#[serde(rename_all = "camelCase")]
pub struct User_ {
  #[serde(deserialize_with = "deserialize_number_from_string")]
  #[serde(rename(deserialize = "uid"))]
  #[serde(skip_serializing)]
  #[getset(get_copy = "pub with_prefix", set = "pub")]
  pub id: i64,
  pub name: String,
  pub fedi_name: String,
  pub preferred_username: Option<String>,
  pub password_encrypted: String,
  pub email: Option<String>,
  pub avatar: Option<String>,
  pub admin: bool,
  pub banned: bool,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
  pub show_nsfw: bool,
  pub theme: String,
  pub default_sort_type: i16,
  pub default_listing_type: i16,
  pub lang: String,
  pub show_avatars: bool,
  pub send_notifications_to_email: bool,
  pub matrix_user_id: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserForm {
  pub name: String,
  pub fedi_name: String,
  pub preferred_username: Option<String>,
  pub password_encrypted: String,
  pub admin: bool,
  pub banned: bool,
  pub email: Option<String>,
  pub avatar: Option<String>,
  pub updated: Option<chrono::NaiveDateTime>,
  pub show_nsfw: bool,
  pub theme: String,
  pub default_sort_type: i16,
  pub default_listing_type: i16,
  pub lang: String,
  pub show_avatars: bool,
  pub send_notifications_to_email: bool,
  pub matrix_user_id: Option<String>,
}

// impl CrudNode<UserForm> for User_ {}

impl From<UserForm> for User_ {
  fn from(form: UserForm) -> Self {
    User_ {
      id: 0,
      name: form.name,
      fedi_name: form.fedi_name,
      preferred_username: form.preferred_username,
      password_encrypted: form.password_encrypted,
      admin: form.admin,
      banned: form.banned,
      email: form.email,
      avatar: form.avatar,
      updated: form.updated,
      show_nsfw: form.show_avatars,
      theme: form.theme,
      default_sort_type: form.default_sort_type,
      default_listing_type: form.default_listing_type,
      lang: form.lang,
      show_avatars: form.show_avatars,
      send_notifications_to_email: form.send_notifications_to_email,
      matrix_user_id: form.matrix_user_id,
      published: chrono::Utc::now().naive_utc(),
    }
  }
}

impl User_ {

  /// DB type name
  const GDB_TYPE: &'static str = "Site";

  /**
   * Register new user
   */
  pub async fn register(conn: &dgraph::Client, form: &UserForm) -> Result<Self, Error> {
    let mut edited_user: User_ = form.clone().into();
    let password_hash =
      hash(&form.password_encrypted, DEFAULT_COST).expect("Couldn't hash password");
    
    edited_user.password_encrypted = password_hash;
    let uid = create_node::<User_>(conn, &mut edited_user).await;
    match uid {
      Ok(i) => Ok(edited_user),
      Err(e) => Err(e),
    }
  }

  /**
   * Update user password
   */
  pub async fn update_password(
    conn: &dgraph::Client,
    user_id: i64,
    new_password: &str,
  ) -> Result<Self, Error> {
    let password_hash = hash(new_password, DEFAULT_COST).expect("Couldn't hash password");

    let mut dict = serde_json::Map::new();
    let pred_name = format!("{}.{}", Self::GDB_TYPE, "password_encrypted");
    dict.insert(pred_name, serde_json::Value::String(password_hash));

    update_node_dict::<Self>(conn, user_id,
      &serde_json::Value::Object(dict)).await
  }

  /**
   * Get User from name.
   */
  pub async fn read_from_name(conn: &dgraph::Client, from_user_name: String) -> Result<Self, Error> {
    let pred_name = "name";
    let pred_repr = format!("{:?}", from_user_name);
    find_node::<Self>(conn, pred_name, &pred_repr).await
  }
}

impl Node for User_ {
  fn db_type_name() -> &'static str {
    User_::GDB_TYPE
  }
}


impl Node for UserForm {
  fn db_type_name() -> &'static str {
    User_::GDB_TYPE
  }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Setters, CopyGetters)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
  #[getset(get_copy = "pub with_prefix", set = "pub")]
  pub id: i64,
  pub username: String,
  pub iss: String,
  pub show_nsfw: bool,
  pub theme: String,
  pub default_sort_type: i16,
  pub default_listing_type: i16,
  pub lang: String,
  pub avatar: Option<String>,
  pub show_avatars: bool,
}

impl Claims {

  /**
   * Decode and validate JSON Web Token
   */
  pub fn decode(jwt: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let v = Validation {
      validate_exp: false,
      ..Validation::default()
    };
    decode::<Claims>(
      &jwt,
      &DecodingKey::from_secret(Settings::get().jwt_secret.as_ref()),
      &v,
    )
  }
}

type Jwt = String;

impl User_ {

  /**
   * Generate JSON Web Token
   */
  pub fn jwt(&self) -> Jwt {
    let my_claims = Claims {
      id: self.id,
      username: self.name.to_owned(),
      iss: self.fedi_name.to_owned(),
      show_nsfw: self.show_nsfw,
      theme: self.theme.to_owned(),
      default_sort_type: self.default_sort_type,
      default_listing_type: self.default_listing_type,
      lang: self.lang.to_owned(),
      avatar: self.avatar.to_owned(),
      show_avatars: self.show_avatars.to_owned(),
    };
    encode(
      &Header::default(),
      &my_claims,
      &EncodingKey::from_secret(Settings::get().jwt_secret.as_ref()),
    )
    .unwrap()
  }

  pub async fn find_by_username(conn: &dgraph::Client, username: &str) -> Result<Self, Error> {
    let pred_name = "name";
    let pred_repr = format!("{:?}", username);
    find_node::<Self>(conn, pred_name, &pred_repr).await
  }

  pub async fn find_by_email(conn: &dgraph::Client, from_email: &str) -> Result<Self, Error> {
    find_node::<Self>(conn, "email", &format!("{:?}", from_email)).await
  }

  pub async fn find_by_email_or_username(
    conn: &dgraph::Client,
    username_or_email: &str,
  ) -> Result<Self, Error> {
    if is_email_regex(username_or_email) {
      User_::find_by_email(conn, username_or_email).await
    } else {
      User_::find_by_username(conn, username_or_email).await
    }
  }

  pub fn get_profile_url(&self) -> String {
    format!("https://{}/u/{}", Settings::get().hostname, self.name)
  }

  pub async fn find_by_jwt(conn: &dgraph::Client, jwt: &str) -> Result<Self, Error> {
    let claims: Claims = Claims::decode(&jwt).expect("Invalid token").claims;
    read_node::<Self>(conn, claims.id).await
  }
}

// #[cfg(test)]
// mod tests {
//   use super::User_;
//   use super::*;

//   #[test]
//   fn test_crud() {
//     let conn = establish_unpooled_connection();

//     let new_user = UserForm {
//       name: "thommy".into(),
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

//     let expected_user = User_ {
//       id: inserted_user.id,
//       name: "thommy".into(),
//       fedi_name: "rrf".into(),
//       preferred_username: None,
//       password_encrypted: "nope".into(),
//       email: None,
//       matrix_user_id: None,
//       avatar: None,
//       admin: false,
//       banned: false,
//       published: inserted_user.published,
//       updated: None,
//       show_nsfw: false,
//       theme: "darkly".into(),
//       default_sort_type: SortType::Hot as i16,
//       default_listing_type: ListingType::Subscribed as i16,
//       lang: "browser".into(),
//       show_avatars: true,
//       send_notifications_to_email: false,
//     };

//     let read_user = User_::read(&conn, inserted_user.id).unwrap();
//     let updated_user = User_::update(&conn, inserted_user.id, &new_user).unwrap();
//     let num_deleted = User_::delete(&conn, inserted_user.id).unwrap();

//     assert_eq!(expected_user, read_user);
//     assert_eq!(expected_user, inserted_user);
//     assert_eq!(expected_user, updated_user);
//     assert_eq!(1, num_deleted);
//   }
// }
