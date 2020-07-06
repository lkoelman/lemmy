use super::*;
use crate::schema::{
  mod_add, mod_add_community, mod_ban, mod_ban_from_community, mod_lock_post, mod_remove_comment,
  mod_remove_community, mod_remove_post, mod_sticky_post,
};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct ModRemovePost {
  pub id: i64,
  pub mod_user_id: i64,
  pub post_id: i64,
  pub reason: Option<String>,
  pub removed: Option<bool>,
  pub when_: chrono::NaiveDateTime,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ModRemovePostForm {
  pub mod_user_id: i64,
  pub post_id: i64,
  pub reason: Option<String>,
  pub removed: Option<bool>,
}

impl CrudNode<ModRemovePostForm, dgraph::Client> for ModRemovePost {
  fn read(conn: &dgraph::Client, from_id: i64) -> Result<Self> {
    use crate::schema::mod_remove_post::dsl::*;
    mod_remove_post.find(from_id).first::<Self>(conn)
  }

  fn delete(conn: &dgraph::Client, from_id: i64) -> Result<usize> {
    use crate::schema::mod_remove_post::dsl::*;
    diesel::delete(mod_remove_post.find(from_id)).execute(conn)
  }

  fn create(conn: &dgraph::Client, form: &ModRemovePostForm) -> Result<Self> {
    use crate::schema::mod_remove_post::dsl::*;
    insert_into(mod_remove_post)
      .values(form)
      .get_result::<Self>(conn)
  }

  fn update(conn: &dgraph::Client, from_id: i64, form: &ModRemovePostForm) -> Result<Self> {
    use crate::schema::mod_remove_post::dsl::*;
    diesel::update(mod_remove_post.find(from_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct ModLockPost {
  pub id: i64,
  pub mod_user_id: i64,
  pub post_id: i64,
  pub locked: Option<bool>,
  pub when_: chrono::NaiveDateTime,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ModLockPostForm {
  pub mod_user_id: i64,
  pub post_id: i64,
  pub locked: Option<bool>,
}

impl CrudNode<ModLockPostForm, dgraph::Client> for ModLockPost {
  fn read(conn: &dgraph::Client, from_id: i64) -> Result<Self> {
    use crate::schema::mod_lock_post::dsl::*;
    mod_lock_post.find(from_id).first::<Self>(conn)
  }

  fn delete(conn: &dgraph::Client, from_id: i64) -> Result<usize> {
    use crate::schema::mod_lock_post::dsl::*;
    diesel::delete(mod_lock_post.find(from_id)).execute(conn)
  }

  fn create(conn: &dgraph::Client, form: &ModLockPostForm) -> Result<Self> {
    use crate::schema::mod_lock_post::dsl::*;
    insert_into(mod_lock_post)
      .values(form)
      .get_result::<Self>(conn)
  }

  fn update(conn: &dgraph::Client, from_id: i64, form: &ModLockPostForm) -> Result<Self> {
    use crate::schema::mod_lock_post::dsl::*;
    diesel::update(mod_lock_post.find(from_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct ModStickyPost {
  pub id: i64,
  pub mod_user_id: i64,
  pub post_id: i64,
  pub stickied: Option<bool>,
  pub when_: chrono::NaiveDateTime,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ModStickyPostForm {
  pub mod_user_id: i64,
  pub post_id: i64,
  pub stickied: Option<bool>,
}

impl CrudNode<ModStickyPostForm, dgraph::Client> for ModStickyPost {
  fn read(conn: &dgraph::Client, from_id: i64) -> Result<Self> {
    use crate::schema::mod_sticky_post::dsl::*;
    mod_sticky_post.find(from_id).first::<Self>(conn)
  }

  fn delete(conn: &dgraph::Client, from_id: i64) -> Result<usize> {
    use crate::schema::mod_sticky_post::dsl::*;
    diesel::delete(mod_sticky_post.find(from_id)).execute(conn)
  }

  fn create(conn: &dgraph::Client, form: &ModStickyPostForm) -> Result<Self> {
    use crate::schema::mod_sticky_post::dsl::*;
    insert_into(mod_sticky_post)
      .values(form)
      .get_result::<Self>(conn)
  }

  fn update(conn: &dgraph::Client, from_id: i64, form: &ModStickyPostForm) -> Result<Self> {
    use crate::schema::mod_sticky_post::dsl::*;
    diesel::update(mod_sticky_post.find(from_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct ModRemoveComment {
  pub id: i64,
  pub mod_user_id: i64,
  pub comment_id: i64,
  pub reason: Option<String>,
  pub removed: Option<bool>,
  pub when_: chrono::NaiveDateTime,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ModRemoveCommentForm {
  pub mod_user_id: i64,
  pub comment_id: i64,
  pub reason: Option<String>,
  pub removed: Option<bool>,
}

impl CrudNode<ModRemoveCommentForm, dgraph::Client> for ModRemoveComment {
  fn read(conn: &dgraph::Client, from_id: i64) -> Result<Self> {
    use crate::schema::mod_remove_comment::dsl::*;
    mod_remove_comment.find(from_id).first::<Self>(conn)
  }

  fn delete(conn: &dgraph::Client, from_id: i64) -> Result<usize> {
    use crate::schema::mod_remove_comment::dsl::*;
    diesel::delete(mod_remove_comment.find(from_id)).execute(conn)
  }

  fn create(conn: &dgraph::Client, form: &ModRemoveCommentForm) -> Result<Self> {
    use crate::schema::mod_remove_comment::dsl::*;
    insert_into(mod_remove_comment)
      .values(form)
      .get_result::<Self>(conn)
  }

  fn update(conn: &dgraph::Client, from_id: i64, form: &ModRemoveCommentForm) -> Result<Self> {
    use crate::schema::mod_remove_comment::dsl::*;
    diesel::update(mod_remove_comment.find(from_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct ModRemoveCommunity {
  pub id: i64,
  pub mod_user_id: i64,
  pub community_id: i64,
  pub reason: Option<String>,
  pub removed: Option<bool>,
  pub expires: Option<chrono::NaiveDateTime>,
  pub when_: chrono::NaiveDateTime,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ModRemoveCommunityForm {
  pub mod_user_id: i64,
  pub community_id: i64,
  pub reason: Option<String>,
  pub removed: Option<bool>,
  pub expires: Option<chrono::NaiveDateTime>,
}

impl CrudNode<ModRemoveCommunityForm, dgraph::Client> for ModRemoveCommunity {
  fn read(conn: &dgraph::Client, from_id: i64) -> Result<Self> {
    use crate::schema::mod_remove_community::dsl::*;
    mod_remove_community.find(from_id).first::<Self>(conn)
  }

  fn delete(conn: &dgraph::Client, from_id: i64) -> Result<usize> {
    use crate::schema::mod_remove_community::dsl::*;
    diesel::delete(mod_remove_community.find(from_id)).execute(conn)
  }

  fn create(conn: &dgraph::Client, form: &ModRemoveCommunityForm) -> Result<Self> {
    use crate::schema::mod_remove_community::dsl::*;
    insert_into(mod_remove_community)
      .values(form)
      .get_result::<Self>(conn)
  }

  fn update(
    conn: &dgraph::Client,
    from_id: i64,
    form: &ModRemoveCommunityForm,
  ) -> Result<Self> {
    use crate::schema::mod_remove_community::dsl::*;
    diesel::update(mod_remove_community.find(from_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct ModBanFromCommunity {
  pub id: i64,
  pub mod_user_id: i64,
  pub other_user_id: i64,
  pub community_id: i64,
  pub reason: Option<String>,
  pub banned: Option<bool>,
  pub expires: Option<chrono::NaiveDateTime>,
  pub when_: chrono::NaiveDateTime,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ModBanFromCommunityForm {
  pub mod_user_id: i64,
  pub other_user_id: i64,
  pub community_id: i64,
  pub reason: Option<String>,
  pub banned: Option<bool>,
  pub expires: Option<chrono::NaiveDateTime>,
}

impl CrudNode<ModBanFromCommunityForm, dgraph::Client> for ModBanFromCommunity {
  fn read(conn: &dgraph::Client, from_id: i64) -> Result<Self> {
    use crate::schema::mod_ban_from_community::dsl::*;
    mod_ban_from_community.find(from_id).first::<Self>(conn)
  }

  fn delete(conn: &dgraph::Client, from_id: i64) -> Result<usize> {
    use crate::schema::mod_ban_from_community::dsl::*;
    diesel::delete(mod_ban_from_community.find(from_id)).execute(conn)
  }

  fn create(conn: &dgraph::Client, form: &ModBanFromCommunityForm) -> Result<Self> {
    use crate::schema::mod_ban_from_community::dsl::*;
    insert_into(mod_ban_from_community)
      .values(form)
      .get_result::<Self>(conn)
  }

  fn update(
    conn: &dgraph::Client,
    from_id: i64,
    form: &ModBanFromCommunityForm,
  ) -> Result<Self> {
    use crate::schema::mod_ban_from_community::dsl::*;
    diesel::update(mod_ban_from_community.find(from_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct ModBan {
  pub id: i64,
  pub mod_user_id: i64,
  pub other_user_id: i64,
  pub reason: Option<String>,
  pub banned: Option<bool>,
  pub expires: Option<chrono::NaiveDateTime>,
  pub when_: chrono::NaiveDateTime,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ModBanForm {
  pub mod_user_id: i64,
  pub other_user_id: i64,
  pub reason: Option<String>,
  pub banned: Option<bool>,
  pub expires: Option<chrono::NaiveDateTime>,
}

impl CrudNode<ModBanForm, dgraph::Client> for ModBan {
  fn read(conn: &dgraph::Client, from_id: i64) -> Result<Self> {
    use crate::schema::mod_ban::dsl::*;
    mod_ban.find(from_id).first::<Self>(conn)
  }

  fn delete(conn: &dgraph::Client, from_id: i64) -> Result<usize> {
    use crate::schema::mod_ban::dsl::*;
    diesel::delete(mod_ban.find(from_id)).execute(conn)
  }

  fn create(conn: &dgraph::Client, form: &ModBanForm) -> Result<Self> {
    use crate::schema::mod_ban::dsl::*;
    insert_into(mod_ban).values(form).get_result::<Self>(conn)
  }

  fn update(conn: &dgraph::Client, from_id: i64, form: &ModBanForm) -> Result<Self> {
    use crate::schema::mod_ban::dsl::*;
    diesel::update(mod_ban.find(from_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct ModAddCommunity {
  pub id: i64,
  pub mod_user_id: i64,
  pub other_user_id: i64,
  pub community_id: i64,
  pub removed: Option<bool>,
  pub when_: chrono::NaiveDateTime,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ModAddCommunityForm {
  pub mod_user_id: i64,
  pub other_user_id: i64,
  pub community_id: i64,
  pub removed: Option<bool>,
}

impl CrudNode<ModAddCommunityForm, dgraph::Client> for ModAddCommunity {
  fn read(conn: &dgraph::Client, from_id: i64) -> Result<Self> {
    use crate::schema::mod_add_community::dsl::*;
    mod_add_community.find(from_id).first::<Self>(conn)
  }

  fn delete(conn: &dgraph::Client, from_id: i64) -> Result<usize> {
    use crate::schema::mod_add_community::dsl::*;
    diesel::delete(mod_add_community.find(from_id)).execute(conn)
  }

  fn create(conn: &dgraph::Client, form: &ModAddCommunityForm) -> Result<Self> {
    use crate::schema::mod_add_community::dsl::*;
    insert_into(mod_add_community)
      .values(form)
      .get_result::<Self>(conn)
  }

  fn update(conn: &dgraph::Client, from_id: i64, form: &ModAddCommunityForm) -> Result<Self> {
    use crate::schema::mod_add_community::dsl::*;
    diesel::update(mod_add_community.find(from_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct ModAdd {
  pub id: i64,
  pub mod_user_id: i64,
  pub other_user_id: i64,
  pub removed: Option<bool>,
  pub when_: chrono::NaiveDateTime,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ModAddForm {
  pub mod_user_id: i64,
  pub other_user_id: i64,
  pub removed: Option<bool>,
}

impl CrudNode<ModAddForm, dgraph::Client> for ModAdd {
  fn read(conn: &dgraph::Client, from_id: i64) -> Result<Self> {
    use crate::schema::mod_add::dsl::*;
    mod_add.find(from_id).first::<Self>(conn)
  }

  fn delete(conn: &dgraph::Client, from_id: i64) -> Result<usize> {
    use crate::schema::mod_add::dsl::*;
    diesel::delete(mod_add.find(from_id)).execute(conn)
  }

  fn create(conn: &dgraph::Client, form: &ModAddForm) -> Result<Self> {
    use crate::schema::mod_add::dsl::*;
    insert_into(mod_add).values(form).get_result::<Self>(conn)
  }

  fn update(conn: &dgraph::Client, from_id: i64, form: &ModAddForm) -> Result<Self> {
    use crate::schema::mod_add::dsl::*;
    diesel::update(mod_add.find(from_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

#[cfg(test)]
mod tests {
  use super::super::comment::*;
  use super::super::community::*;
  use super::super::post::*;
  use super::super::user::*;
  use super::*;
  // use Crud;
  #[test]
  fn test_crud() {
    let conn = establish_unpooled_connection();

    let new_mod = UserForm {
      name: "the mod".into(),
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

    let inserted_mod = User_::create(&conn, &new_mod).unwrap();

    let new_user = UserForm {
      name: "jim2".into(),
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
      name: "mod_community".to_string(),
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
      name: "A test post thweep".into(),
      url: None,
      body: None,
      creator_id: inserted_user.id,
      community_id: inserted_community.id,
      removed: None,
      deleted: None,
      locked: None,
      stickied: None,
      updated: None,
      nsfw: false,
      embed_title: None,
      embed_description: None,
      embed_html: None,
      thumbnail_url: None,
    };

    let inserted_post = Post::create(&conn, &new_post).unwrap();

    let comment_form = CommentForm {
      content: "A test comment".into(),
      creator_id: inserted_user.id,
      post_id: inserted_post.id,
      removed: None,
      deleted: None,
      read: None,
      parent_id: None,
      updated: None,
    };

    let inserted_comment = Comment::create(&conn, &comment_form).unwrap();

    // Now the actual tests

    // remove post
    let mod_remove_post_form = ModRemovePostForm {
      mod_user_id: inserted_mod.id,
      post_id: inserted_post.id,
      reason: None,
      removed: None,
    };
    let inserted_mod_remove_post = ModRemovePost::create(&conn, &mod_remove_post_form).unwrap();
    let read_mod_remove_post = ModRemovePost::read(&conn, inserted_mod_remove_post.id).unwrap();
    let expected_mod_remove_post = ModRemovePost {
      id: inserted_mod_remove_post.id,
      post_id: inserted_post.id,
      mod_user_id: inserted_mod.id,
      reason: None,
      removed: Some(true),
      when_: inserted_mod_remove_post.when_,
    };

    // lock post

    let mod_lock_post_form = ModLockPostForm {
      mod_user_id: inserted_mod.id,
      post_id: inserted_post.id,
      locked: None,
    };
    let inserted_mod_lock_post = ModLockPost::create(&conn, &mod_lock_post_form).unwrap();
    let read_mod_lock_post = ModLockPost::read(&conn, inserted_mod_lock_post.id).unwrap();
    let expected_mod_lock_post = ModLockPost {
      id: inserted_mod_lock_post.id,
      post_id: inserted_post.id,
      mod_user_id: inserted_mod.id,
      locked: Some(true),
      when_: inserted_mod_lock_post.when_,
    };

    // sticky post

    let mod_sticky_post_form = ModStickyPostForm {
      mod_user_id: inserted_mod.id,
      post_id: inserted_post.id,
      stickied: None,
    };
    let inserted_mod_sticky_post = ModStickyPost::create(&conn, &mod_sticky_post_form).unwrap();
    let read_mod_sticky_post = ModStickyPost::read(&conn, inserted_mod_sticky_post.id).unwrap();
    let expected_mod_sticky_post = ModStickyPost {
      id: inserted_mod_sticky_post.id,
      post_id: inserted_post.id,
      mod_user_id: inserted_mod.id,
      stickied: Some(true),
      when_: inserted_mod_sticky_post.when_,
    };

    // comment

    let mod_remove_comment_form = ModRemoveCommentForm {
      mod_user_id: inserted_mod.id,
      comment_id: inserted_comment.id,
      reason: None,
      removed: None,
    };
    let inserted_mod_remove_comment =
      ModRemoveComment::create(&conn, &mod_remove_comment_form).unwrap();
    let read_mod_remove_comment =
      ModRemoveComment::read(&conn, inserted_mod_remove_comment.id).unwrap();
    let expected_mod_remove_comment = ModRemoveComment {
      id: inserted_mod_remove_comment.id,
      comment_id: inserted_comment.id,
      mod_user_id: inserted_mod.id,
      reason: None,
      removed: Some(true),
      when_: inserted_mod_remove_comment.when_,
    };

    // community

    let mod_remove_community_form = ModRemoveCommunityForm {
      mod_user_id: inserted_mod.id,
      community_id: inserted_community.id,
      reason: None,
      removed: None,
      expires: None,
    };
    let inserted_mod_remove_community =
      ModRemoveCommunity::create(&conn, &mod_remove_community_form).unwrap();
    let read_mod_remove_community =
      ModRemoveCommunity::read(&conn, inserted_mod_remove_community.id).unwrap();
    let expected_mod_remove_community = ModRemoveCommunity {
      id: inserted_mod_remove_community.id,
      community_id: inserted_community.id,
      mod_user_id: inserted_mod.id,
      reason: None,
      removed: Some(true),
      expires: None,
      when_: inserted_mod_remove_community.when_,
    };

    // ban from community

    let mod_ban_from_community_form = ModBanFromCommunityForm {
      mod_user_id: inserted_mod.id,
      other_user_id: inserted_user.id,
      community_id: inserted_community.id,
      reason: None,
      banned: None,
      expires: None,
    };
    let inserted_mod_ban_from_community =
      ModBanFromCommunity::create(&conn, &mod_ban_from_community_form).unwrap();
    let read_mod_ban_from_community =
      ModBanFromCommunity::read(&conn, inserted_mod_ban_from_community.id).unwrap();
    let expected_mod_ban_from_community = ModBanFromCommunity {
      id: inserted_mod_ban_from_community.id,
      community_id: inserted_community.id,
      mod_user_id: inserted_mod.id,
      other_user_id: inserted_user.id,
      reason: None,
      banned: Some(true),
      expires: None,
      when_: inserted_mod_ban_from_community.when_,
    };

    // ban

    let mod_ban_form = ModBanForm {
      mod_user_id: inserted_mod.id,
      other_user_id: inserted_user.id,
      reason: None,
      banned: None,
      expires: None,
    };
    let inserted_mod_ban = ModBan::create(&conn, &mod_ban_form).unwrap();
    let read_mod_ban = ModBan::read(&conn, inserted_mod_ban.id).unwrap();
    let expected_mod_ban = ModBan {
      id: inserted_mod_ban.id,
      mod_user_id: inserted_mod.id,
      other_user_id: inserted_user.id,
      reason: None,
      banned: Some(true),
      expires: None,
      when_: inserted_mod_ban.when_,
    };

    // mod add community

    let mod_add_community_form = ModAddCommunityForm {
      mod_user_id: inserted_mod.id,
      other_user_id: inserted_user.id,
      community_id: inserted_community.id,
      removed: None,
    };
    let inserted_mod_add_community =
      ModAddCommunity::create(&conn, &mod_add_community_form).unwrap();
    let read_mod_add_community =
      ModAddCommunity::read(&conn, inserted_mod_add_community.id).unwrap();
    let expected_mod_add_community = ModAddCommunity {
      id: inserted_mod_add_community.id,
      community_id: inserted_community.id,
      mod_user_id: inserted_mod.id,
      other_user_id: inserted_user.id,
      removed: Some(false),
      when_: inserted_mod_add_community.when_,
    };

    // mod add

    let mod_add_form = ModAddForm {
      mod_user_id: inserted_mod.id,
      other_user_id: inserted_user.id,
      removed: None,
    };
    let inserted_mod_add = ModAdd::create(&conn, &mod_add_form).unwrap();
    let read_mod_add = ModAdd::read(&conn, inserted_mod_add.id).unwrap();
    let expected_mod_add = ModAdd {
      id: inserted_mod_add.id,
      mod_user_id: inserted_mod.id,
      other_user_id: inserted_user.id,
      removed: Some(false),
      when_: inserted_mod_add.when_,
    };

    ModRemovePost::delete(&conn, inserted_mod_remove_post.id).unwrap();
    ModLockPost::delete(&conn, inserted_mod_lock_post.id).unwrap();
    ModStickyPost::delete(&conn, inserted_mod_sticky_post.id).unwrap();
    ModRemoveComment::delete(&conn, inserted_mod_remove_comment.id).unwrap();
    ModRemoveCommunity::delete(&conn, inserted_mod_remove_community.id).unwrap();
    ModBanFromCommunity::delete(&conn, inserted_mod_ban_from_community.id).unwrap();
    ModBan::delete(&conn, inserted_mod_ban.id).unwrap();
    ModAddCommunity::delete(&conn, inserted_mod_add_community.id).unwrap();
    ModAdd::delete(&conn, inserted_mod_add.id).unwrap();

    Comment::delete(&conn, inserted_comment.id).unwrap();
    Post::delete(&conn, inserted_post.id).unwrap();
    Community::delete(&conn, inserted_community.id).unwrap();
    User_::delete(&conn, inserted_user.id).unwrap();
    User_::delete(&conn, inserted_mod.id).unwrap();

    assert_eq!(expected_mod_remove_post, read_mod_remove_post);
    assert_eq!(expected_mod_lock_post, read_mod_lock_post);
    assert_eq!(expected_mod_sticky_post, read_mod_sticky_post);
    assert_eq!(expected_mod_remove_comment, read_mod_remove_comment);
    assert_eq!(expected_mod_remove_community, read_mod_remove_community);
    assert_eq!(expected_mod_ban_from_community, read_mod_ban_from_community);
    assert_eq!(expected_mod_ban, read_mod_ban);
    assert_eq!(expected_mod_add_community, read_mod_add_community);
    assert_eq!(expected_mod_add, read_mod_add);
  }
}
