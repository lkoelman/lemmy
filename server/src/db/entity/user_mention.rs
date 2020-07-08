use crate::db::*;
use super::comment::Comment;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserMention {
  #[serde(skip)]
  pub recipient_id: i64,
  #[serde(skip)]
  pub comment_id: i64,
  pub read: bool,
  pub published: chrono::NaiveDateTime,
}

#[derive(Clone, Debug)]
pub struct UserMentionForm {
  pub recipient_id: i64,
  pub comment_id: i64,
  pub read: Option<bool>,
}

impl Edge for UserMention {
  fn from(&self) -> i64 {
    self.comment_id
  }
  fn to(&self) -> i64 {
    self.recipient_id
  }
  fn db_type_name() -> &'static str {
    "Comment.MentionsUser"
  }
}

impl From<UserMentionForm> for UserMention {
  fn from(form: UserMentionForm) -> Self {
      UserMention {
        recipient_id: form.recipient_id,
        comment_id: form.comment_id,
        read: form.read.unwrap_or(false),
        published: chrono::Utc::now().naive_utc(),
      }
  }
}


// impl CrudEdge<UserMentionForm> for UserMention {}

// #[cfg(test)]
// mod tests {
//   use super::super::comment::*;
//   use super::super::community::*;
//   use super::super::post::*;
//   use super::super::user::*;
//   use super::*;
//   #[test]
//   fn test_crud() {
//     let conn = establish_unpooled_connection();

//     let new_user = UserForm {
//       name: "terrylake".into(),
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

//     let recipient_form = UserForm {
//       name: "terrylakes recipient".into(),
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

//     let new_community = CommunityForm {
//       name: "test community lake".to_string(),
//       title: "nada".to_owned(),
//       description: None,
//       category_id: 1,
//       creator_id: inserted_user.id,
//       removed: None,
//       deleted: None,
//       updated: None,
//       nsfw: false,
//     };

//     let inserted_community = Community::create(&conn, &new_community).unwrap();

//     let new_post = PostForm {
//       name: "A test post".into(),
//       creator_id: inserted_user.id,
//       url: None,
//       body: None,
//       community_id: inserted_community.id,
//       removed: None,
//       deleted: None,
//       locked: None,
//       stickied: None,
//       updated: None,
//       nsfw: false,
//       embed_title: None,
//       embed_description: None,
//       embed_html: None,
//       thumbnail_url: None,
//     };

//     let inserted_post = Post::create(&conn, &new_post).unwrap();

//     let comment_form = CommentForm {
//       content: "A test comment".into(),
//       creator_id: inserted_user.id,
//       post_id: inserted_post.id,
//       removed: None,
//       deleted: None,
//       read: None,
//       parent_id: None,
//       updated: None,
//     };

//     let inserted_comment = Comment::create(&conn, &comment_form).unwrap();

//     let user_mention_form = UserMentionForm {
//       recipient_id: inserted_recipient.id,
//       comment_id: inserted_comment.id,
//       read: None,
//     };

//     let inserted_mention = UserMention::create(&conn, &user_mention_form).unwrap();

//     let expected_mention = UserMention {
//       id: inserted_mention.id,
//       recipient_id: inserted_mention.recipient_id,
//       comment_id: inserted_mention.comment_id,
//       read: false,
//       published: inserted_mention.published,
//     };

//     let read_mention = UserMention::read(&conn, inserted_mention.id).unwrap();
//     let updated_mention =
//       UserMention::update(&conn, inserted_mention.id, &user_mention_form).unwrap();
//     let num_deleted = UserMention::delete(&conn, inserted_mention.id).unwrap();
//     Comment::delete(&conn, inserted_comment.id).unwrap();
//     Post::delete(&conn, inserted_post.id).unwrap();
//     Community::delete(&conn, inserted_community.id).unwrap();
//     User_::delete(&conn, inserted_user.id).unwrap();
//     User_::delete(&conn, inserted_recipient.id).unwrap();

//     assert_eq!(expected_mention, read_mention);
//     assert_eq!(expected_mention, inserted_mention);
//     assert_eq!(expected_mention, updated_mention);
//     assert_eq!(1, num_deleted);
//   }
// }
