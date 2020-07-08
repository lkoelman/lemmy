use crate::db::*;

#[derive(PartialEq, Debug, Serialize, Deserialize, Setters, CopyGetters)]
#[serde(rename_all = "camelCase")]
pub struct PrivateMessage {
  #[serde(deserialize_with = "deserialize_number_from_string")]
  #[serde(rename(deserialize = "uid"))]
  #[serde(skip_serializing)]
  #[getset(get_copy = "pub with_prefix", set = "pub")]
  pub id: i64,
  pub creator_id: i64,
  pub recipient_id: i64,
  pub content: String,
  pub deleted: bool,
  pub read: bool,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivateMessageForm {
  pub creator_id: i64,
  pub recipient_id: i64,
  pub content: Option<String>,
  pub deleted: Option<bool>,
  pub read: Option<bool>,
  pub updated: Option<chrono::NaiveDateTime>,
}

impl PrivateMessage {
  /// DB type name
  const GDB_TYPE: &'static str = "PrivateMessage";
}

impl Node for PrivateMessage {
  fn db_type_name() -> &'static str {
    PrivateMessage::GDB_TYPE
  }
}

impl From<PrivateMessageForm> for PrivateMessage {
  fn from(form: PrivateMessageForm) -> Self {
      PrivateMessage {
        id: 0,
        creator_id: form.creator_id,
        recipient_id: form.recipient_id,
        content: form.content.unwrap_or("".into()),
        deleted: form.deleted.unwrap_or(false),
        read: form.read.unwrap_or(false),
        updated: form.updated,
        published: chrono::Utc::now().naive_utc(),
      }
  }
}

// impl CrudNode<PrivateMessageForm> for PrivateMessage {}

// #[cfg(test)]
// mod tests {
//   use super::super::user::*;
//   use super::*;
//   #[test]
//   fn test_crud() {
//     let conn = establish_unpooled_connection();

//     let creator_form = UserForm {
//       name: "creator_pm".into(),
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

//     let inserted_creator = User_::create(&conn, &creator_form).unwrap();

//     let recipient_form = UserForm {
//       name: "recipient_pm".into(),
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

//     let inserted_recipient = User_::create(&conn, &recipient_form).unwrap();

//     let private_message_form = PrivateMessageForm {
//       content: Some("A test private message".into()),
//       creator_id: inserted_creator.id,
//       recipient_id: inserted_recipient.id,
//       deleted: None,
//       read: None,
//       updated: None,
//     };

//     let inserted_private_message = PrivateMessage::create(&conn, &private_message_form).unwrap();

//     let expected_private_message = PrivateMessage {
//       id: inserted_private_message.id,
//       content: "A test private message".into(),
//       creator_id: inserted_creator.id,
//       recipient_id: inserted_recipient.id,
//       deleted: false,
//       read: false,
//       updated: None,
//       published: inserted_private_message.published,
//     };

//     let read_private_message = PrivateMessage::read(&conn, inserted_private_message.id).unwrap();
//     let updated_private_message =
//       PrivateMessage::update(&conn, inserted_private_message.id, &private_message_form).unwrap();
//     let num_deleted = PrivateMessage::delete(&conn, inserted_private_message.id).unwrap();
//     User_::delete(&conn, inserted_creator.id).unwrap();
//     User_::delete(&conn, inserted_recipient.id).unwrap();

//     assert_eq!(expected_private_message, read_private_message);
//     assert_eq!(expected_private_message, updated_private_message);
//     assert_eq!(expected_private_message, inserted_private_message);
//     assert_eq!(1, num_deleted);
//   }
// }
