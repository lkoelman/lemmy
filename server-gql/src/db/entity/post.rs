use crate::db::*;

#[derive(Debug, Deserialize, Serialize, PartialEq, Setters, CopyGetters)]
#[serde(rename_all = "camelCase")]
pub struct Post {
  #[serde(deserialize_with = "deserialize_number_from_string")]
  #[serde(rename(deserialize = "uid"))]
  #[serde(skip_serializing)]
  #[getset(get_copy = "pub with_prefix", set = "pub")]
  pub id: i64,
  pub name: String,
  pub url: Option<String>,
  pub body: Option<String>,
  pub creator_id: i64,
  pub community_id: i64,
  pub removed: bool,
  pub locked: bool,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
  pub deleted: bool,
  pub nsfw: bool,
  pub stickied: bool,
  pub embed_title: Option<String>,
  pub embed_description: Option<String>,
  pub embed_html: Option<String>,
  pub thumbnail_url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostForm {
  pub name: String,
  pub url: Option<String>,
  pub body: Option<String>,
  pub creator_id: i64,
  pub community_id: i64,
  pub removed: Option<bool>,
  pub locked: Option<bool>,
  pub updated: Option<chrono::NaiveDateTime>,
  pub deleted: Option<bool>,
  pub nsfw: bool,
  pub stickied: Option<bool>,
  pub embed_title: Option<String>,
  pub embed_description: Option<String>,
  pub embed_html: Option<String>,
  pub thumbnail_url: Option<String>,
}

impl Post {
  /// Dgraph type
  const GDB_TYPE: &'static str = "Post";
}

impl Node for Post {
  fn db_type_name() -> &'static str {
    Post::GDB_TYPE
  }
}

impl From<PostForm> for Post {
  fn from(form: PostForm) -> Self {
      Post {
        id: 0,
        name: form.name,
        url: form.url,
        body: form.body,
        creator_id: form.creator_id,
        community_id: form.community_id,
        removed: form.removed.unwrap_or(false),
        locked: form.locked.unwrap_or(false),
        updated: form.updated,
        deleted: form.deleted.unwrap_or(false),
        nsfw: form.nsfw,
        stickied: form.stickied.unwrap_or(false),
        embed_title: form.embed_title,
        embed_description: form.embed_description,
        embed_html: form.embed_html,
        thumbnail_url: form.thumbnail_url,
        published: chrono::Utc::now().naive_utc(),
      }
  }
}

impl Node for PostForm {
  fn db_type_name() -> &'static str {
    Post::GDB_TYPE
  }
}

// impl CrudNode<PostForm> for Post {}

//#############################################################################

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostLike {
  #[serde(skip)]
  pub post_id: i64,
  #[serde(skip)]
  pub user_id: i64,
  pub score: i16,
  pub published: chrono::NaiveDateTime,
}

#[derive(Clone, Debug)]
pub struct PostLikeForm {
  pub post_id: i64,
  pub user_id: i64,
  pub score: i16,
}

impl Edge for PostLike {
  fn from(&self) -> i64 {
    self.post_id
  }
  fn to(&self) -> i64 {
    self.user_id
  }
  fn db_type_name() -> &'static str {
    "Post.UserLike"
  }
}

impl From<PostLikeForm> for PostLike {
  fn from(form: PostLikeForm) -> Self {
      PostLike {
        post_id: form.post_id,
        user_id: form.user_id,
        score: form.score,
        published: chrono::Utc::now().naive_utc(),
      }
  }
}

// impl CrudEdge<PostLikeForm> for PostLike {}


//#############################################################################

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostSaved {
  #[serde(skip)]
  pub post_id: i64,
  #[serde(skip)]
  pub user_id: i64,
  pub published: chrono::NaiveDateTime,
}

#[derive(Clone, Debug)]
pub struct PostSavedForm {
  pub post_id: i64,
  pub user_id: i64,
}

impl Edge for PostSaved {
  fn from(&self) -> i64 {
    self.user_id
  }
  fn to(&self) -> i64 {
    self.post_id
  }
  fn db_type_name() -> &'static str {
    "User.SavedPost"
  }
}

impl From<PostSavedForm> for PostSaved {
  fn from(form: PostSavedForm) -> Self {
      PostSaved {
        post_id: form.post_id,
        user_id: form.user_id,
        published: chrono::Utc::now().naive_utc(),
      }
  }
}

// impl CrudEdge<PostSavedForm> for PostSaved {}

//#############################################################################

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostRead {
  #[serde(skip)]
  pub post_id: i64,
  #[serde(skip)]
  pub user_id: i64,
  pub published: chrono::NaiveDateTime,
}

#[derive(Clone)]
pub struct PostReadForm {
  pub post_id: i64,
  pub user_id: i64,
}

impl Edge for PostRead {
  fn from(&self) -> i64 {
    self.post_id
  }
  fn to(&self) -> i64 {
    self.user_id
  }
  fn db_type_name() -> &'static str {
    "Post.ReadByUser"
  }
}

impl From<PostReadForm> for PostRead {
  fn from(form: PostReadForm) -> Self {
      PostRead {
        post_id: form.post_id,
        user_id: form.user_id,
        published: chrono::Utc::now().naive_utc(),
      }
  }
}

// impl CrudEdge<PostReadForm> for PostRead {}

// #[cfg(test)]
// mod tests {
//   use super::super::community::*;
//   use super::super::user::*;
//   use super::*;
//   #[test]
//   fn test_crud() {
//     let conn = establish_unpooled_connection();

//     let new_user = UserForm {
//       name: "jim".into(),
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

//     let new_community = CommunityForm {
//       name: "test community_3".to_string(),
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
//       url: None,
//       body: None,
//       creator_id: inserted_user.id,
//       community_id: inserted_community.id,
//       removed: None,
//       deleted: None,
//       locked: None,
//       stickied: None,
//       nsfw: false,
//       updated: None,
//       embed_title: None,
//       embed_description: None,
//       embed_html: None,
//       thumbnail_url: None,
//     };

//     let inserted_post = Post::create(&conn, &new_post).unwrap();

//     let expected_post = Post {
//       id: inserted_post.id,
//       name: "A test post".into(),
//       url: None,
//       body: None,
//       creator_id: inserted_user.id,
//       community_id: inserted_community.id,
//       published: inserted_post.published,
//       removed: false,
//       locked: false,
//       stickied: false,
//       nsfw: false,
//       deleted: false,
//       updated: None,
//       embed_title: None,
//       embed_description: None,
//       embed_html: None,
//       thumbnail_url: None,
//     };

//     // Post Like
//     let post_like_form = PostLikeForm {
//       post_id: inserted_post.id,
//       user_id: inserted_user.id,
//       score: 1,
//     };

//     let inserted_post_like = PostLike::like(&conn, &post_like_form).unwrap();

//     let expected_post_like = PostLike {
//       id: inserted_post_like.id,
//       post_id: inserted_post.id,
//       user_id: inserted_user.id,
//       published: inserted_post_like.published,
//       score: 1,
//     };

//     // Post Save
//     let post_saved_form = PostSavedForm {
//       post_id: inserted_post.id,
//       user_id: inserted_user.id,
//     };

//     let inserted_post_saved = PostSaved::save(&conn, &post_saved_form).unwrap();

//     let expected_post_saved = PostSaved {
//       id: inserted_post_saved.id,
//       post_id: inserted_post.id,
//       user_id: inserted_user.id,
//       published: inserted_post_saved.published,
//     };

//     // Post Read
//     let post_read_form = PostReadForm {
//       post_id: inserted_post.id,
//       user_id: inserted_user.id,
//     };

//     let inserted_post_read = PostRead::mark_as_read(&conn, &post_read_form).unwrap();

//     let expected_post_read = PostRead {
//       id: inserted_post_read.id,
//       post_id: inserted_post.id,
//       user_id: inserted_user.id,
//       published: inserted_post_read.published,
//     };

//     let read_post = Post::read(&conn, inserted_post.id).unwrap();
//     let updated_post = Post::update(&conn, inserted_post.id, &new_post).unwrap();
//     let like_removed = PostLike::remove(&conn, &post_like_form).unwrap();
//     let saved_removed = PostSaved::unsave(&conn, &post_saved_form).unwrap();
//     let read_removed = PostRead::mark_as_unread(&conn, &post_read_form).unwrap();
//     let num_deleted = Post::delete(&conn, inserted_post.id).unwrap();
//     Community::delete(&conn, inserted_community.id).unwrap();
//     User_::delete(&conn, inserted_user.id).unwrap();

//     assert_eq!(expected_post, read_post);
//     assert_eq!(expected_post, inserted_post);
//     assert_eq!(expected_post, updated_post);
//     assert_eq!(expected_post_like, inserted_post_like);
//     assert_eq!(expected_post_saved, inserted_post_saved);
//     assert_eq!(expected_post_read, inserted_post_read);
//     assert_eq!(1, like_removed);
//     assert_eq!(1, saved_removed);
//     assert_eq!(1, read_removed);
//     assert_eq!(1, num_deleted);
//   }
// }
