use super::*;
use crate::schema::{post, post_like, post_read, post_saved};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Post {
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

#[derive(Clone)]
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

impl CrudNode<PostForm, dgraph::Client> for Post {
  fn read(conn: &dgraph::Client, post_id: i64) -> Result<Self> {
    use crate::schema::post::dsl::*;
    post.find(post_id).first::<Self>(conn)
  }

  fn delete(conn: &dgraph::Client, post_id: i64) -> Result<usize> {
    use crate::schema::post::dsl::*;
    diesel::delete(post.find(post_id)).execute(conn)
  }

  fn create(conn: &dgraph::Client, new_post: &PostForm) -> Result<Self> {
    use crate::schema::post::dsl::*;
    insert_into(post).values(new_post).get_result::<Self>(conn)
  }

  fn update(conn: &dgraph::Client, post_id: i64, new_post: &PostForm) -> Result<Self> {
    use crate::schema::post::dsl::*;
    diesel::update(post.find(post_id))
      .set(new_post)
      .get_result::<Self>(conn)
  }
}

#[derive(PartialEq, Debug)]
#[belongs_to(Post)]
pub struct PostLike {
  pub id: i64,
  pub post_id: i64,
  pub user_id: i64,
  pub score: i16,
  pub published: chrono::NaiveDateTime,
}

#[derive(Clone)]
pub struct PostLikeForm {
  pub post_id: i64,
  pub user_id: i64,
  pub score: i16,
}

impl Likeable<PostLikeForm> for PostLike {
  fn read(conn: &dgraph::Client, post_id_from: i32) -> Result<Vec<Self>> {
    use crate::schema::post_like::dsl::*;
    post_like
      .filter(post_id.eq(post_id_from))
      .load::<Self>(conn)
  }
  fn like(conn: &dgraph::Client, post_like_form: &PostLikeForm) -> Result<Self> {
    use crate::schema::post_like::dsl::*;
    insert_into(post_like)
      .values(post_like_form)
      .get_result::<Self>(conn)
  }
  fn remove(conn: &dgraph::Client, post_like_form: &PostLikeForm) -> Result<usize> {
    use crate::schema::post_like::dsl::*;
    diesel::delete(
      post_like
        .filter(post_id.eq(post_like_form.post_id))
        .filter(user_id.eq(post_like_form.user_id)),
    )
    .execute(conn)
  }
}

#[derive(PartialEq, Debug)]
#[belongs_to(Post)]
pub struct PostSaved {
  pub id: i64,
  pub post_id: i64,
  pub user_id: i64,
  pub published: chrono::NaiveDateTime,
}

#[derive(Clone)]
pub struct PostSavedForm {
  pub post_id: i64,
  pub user_id: i64,
}

impl Saveable<PostSavedForm> for PostSaved {
  fn save(conn: &dgraph::Client, post_saved_form: &PostSavedForm) -> Result<Self> {
    use crate::schema::post_saved::dsl::*;
    insert_into(post_saved)
      .values(post_saved_form)
      .get_result::<Self>(conn)
  }
  fn unsave(conn: &dgraph::Client, post_saved_form: &PostSavedForm) -> Result<usize> {
    use crate::schema::post_saved::dsl::*;
    diesel::delete(
      post_saved
        .filter(post_id.eq(post_saved_form.post_id))
        .filter(user_id.eq(post_saved_form.user_id)),
    )
    .execute(conn)
  }
}

#[derive(PartialEq, Debug)]
#[belongs_to(Post)]
pub struct PostRead {
  pub id: i64,
  pub post_id: i64,
  pub user_id: i64,
  pub published: chrono::NaiveDateTime,
}

#[derive(Clone)]
pub struct PostReadForm {
  pub post_id: i64,
  pub user_id: i64,
}

impl Readable<PostReadForm> for PostRead {
  fn mark_as_read(conn: &dgraph::Client, post_read_form: &PostReadForm) -> Result<Self> {
    use crate::schema::post_read::dsl::*;
    insert_into(post_read)
      .values(post_read_form)
      .get_result::<Self>(conn)
  }
  fn mark_as_unread(conn: &dgraph::Client, post_read_form: &PostReadForm) -> Result<usize> {
    use crate::schema::post_read::dsl::*;
    diesel::delete(
      post_read
        .filter(post_id.eq(post_read_form.post_id))
        .filter(user_id.eq(post_read_form.user_id)),
    )
    .execute(conn)
  }
}

#[cfg(test)]
mod tests {
  use super::super::community::*;
  use super::super::user::*;
  use super::*;
  #[test]
  fn test_crud() {
    let conn = establish_unpooled_connection();

    let new_user = UserForm {
      name: "jim".into(),
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

    let new_community = CommunityForm {
      name: "test community_3".to_string(),
      title: "nada".to_owned(),
      description: None,
      category_id: 1,
      creator_id: inserted_user.id,
      removed: None,
      deleted: None,
      updated: None,
      nsfw: false,
    };

    let inserted_community = Community::create(&conn, &new_community).unwrap();

    let new_post = PostForm {
      name: "A test post".into(),
      url: None,
      body: None,
      creator_id: inserted_user.id,
      community_id: inserted_community.id,
      removed: None,
      deleted: None,
      locked: None,
      stickied: None,
      nsfw: false,
      updated: None,
      embed_title: None,
      embed_description: None,
      embed_html: None,
      thumbnail_url: None,
    };

    let inserted_post = Post::create(&conn, &new_post).unwrap();

    let expected_post = Post {
      id: inserted_post.id,
      name: "A test post".into(),
      url: None,
      body: None,
      creator_id: inserted_user.id,
      community_id: inserted_community.id,
      published: inserted_post.published,
      removed: false,
      locked: false,
      stickied: false,
      nsfw: false,
      deleted: false,
      updated: None,
      embed_title: None,
      embed_description: None,
      embed_html: None,
      thumbnail_url: None,
    };

    // Post Like
    let post_like_form = PostLikeForm {
      post_id: inserted_post.id,
      user_id: inserted_user.id,
      score: 1,
    };

    let inserted_post_like = PostLike::like(&conn, &post_like_form).unwrap();

    let expected_post_like = PostLike {
      id: inserted_post_like.id,
      post_id: inserted_post.id,
      user_id: inserted_user.id,
      published: inserted_post_like.published,
      score: 1,
    };

    // Post Save
    let post_saved_form = PostSavedForm {
      post_id: inserted_post.id,
      user_id: inserted_user.id,
    };

    let inserted_post_saved = PostSaved::save(&conn, &post_saved_form).unwrap();

    let expected_post_saved = PostSaved {
      id: inserted_post_saved.id,
      post_id: inserted_post.id,
      user_id: inserted_user.id,
      published: inserted_post_saved.published,
    };

    // Post Read
    let post_read_form = PostReadForm {
      post_id: inserted_post.id,
      user_id: inserted_user.id,
    };

    let inserted_post_read = PostRead::mark_as_read(&conn, &post_read_form).unwrap();

    let expected_post_read = PostRead {
      id: inserted_post_read.id,
      post_id: inserted_post.id,
      user_id: inserted_user.id,
      published: inserted_post_read.published,
    };

    let read_post = Post::read(&conn, inserted_post.id).unwrap();
    let updated_post = Post::update(&conn, inserted_post.id, &new_post).unwrap();
    let like_removed = PostLike::remove(&conn, &post_like_form).unwrap();
    let saved_removed = PostSaved::unsave(&conn, &post_saved_form).unwrap();
    let read_removed = PostRead::mark_as_unread(&conn, &post_read_form).unwrap();
    let num_deleted = Post::delete(&conn, inserted_post.id).unwrap();
    Community::delete(&conn, inserted_community.id).unwrap();
    User_::delete(&conn, inserted_user.id).unwrap();

    assert_eq!(expected_post, read_post);
    assert_eq!(expected_post, inserted_post);
    assert_eq!(expected_post, updated_post);
    assert_eq!(expected_post_like, inserted_post_like);
    assert_eq!(expected_post_saved, inserted_post_saved);
    assert_eq!(expected_post_read, inserted_post_read);
    assert_eq!(1, like_removed);
    assert_eq!(1, saved_removed);
    assert_eq!(1, read_removed);
    assert_eq!(1, num_deleted);
  }
}
