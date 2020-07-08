use crate::db::*;



#[derive(
  PartialEq, Debug, Serialize, Deserialize, Clone,
)]
pub struct ModRemovePostView {
  pub id: i32,
  pub mod_user_id: i32,
  pub post_id: i32,
  pub reason: Option<String>,
  pub removed: Option<bool>,
  pub when_: chrono::NaiveDateTime,
  pub mod_user_name: String,
  pub post_name: String,
  pub community_id: i32,
  pub community_name: String,
}

impl ModRemovePostView {
  pub fn list(
    conn: &dgraph_tonic::Client,
    from_community_id: Option<i32>,
    from_mod_user_id: Option<i32>,
    page: Option<i64>,
    limit: Option<i64>,
  ) -> Result<Vec<Self>, Error> {
    use super::moderator_views::mod_remove_post_view::dsl::*;
    let mut query = mod_remove_post_view.into_boxed();

    let (limit, offset) = limit_and_offset(page, limit);

    if let Some(from_community_id) = from_community_id {
      query = query.filter(community_id.eq(from_community_id));
    };

    if let Some(from_mod_user_id) = from_mod_user_id {
      query = query.filter(mod_user_id.eq(from_mod_user_id));
    };

    query
      .limit(limit)
      .offset(offset)
      .order_by(when_.desc())
      .load::<Self>(conn)
  }
}



#[derive(
  PartialEq, Debug, Serialize, Deserialize, Clone,
)]
pub struct ModLockPostView {
  pub id: i32,
  pub mod_user_id: i32,
  pub post_id: i32,
  pub locked: Option<bool>,
  pub when_: chrono::NaiveDateTime,
  pub mod_user_name: String,
  pub post_name: String,
  pub community_id: i32,
  pub community_name: String,
}

impl ModLockPostView {
  pub fn list(
    conn: &dgraph_tonic::Client,
    from_community_id: Option<i32>,
    from_mod_user_id: Option<i32>,
    page: Option<i64>,
    limit: Option<i64>,
  ) -> Result<Vec<Self>, Error> {
    use super::moderator_views::mod_lock_post_view::dsl::*;
    let mut query = mod_lock_post_view.into_boxed();

    let (limit, offset) = limit_and_offset(page, limit);

    if let Some(from_community_id) = from_community_id {
      query = query.filter(community_id.eq(from_community_id));
    };

    if let Some(from_mod_user_id) = from_mod_user_id {
      query = query.filter(mod_user_id.eq(from_mod_user_id));
    };

    query
      .limit(limit)
      .offset(offset)
      .order_by(when_.desc())
      .load::<Self>(conn)
  }
}



#[derive(
  PartialEq, Debug, Serialize, Deserialize, Clone,
)]
pub struct ModStickyPostView {
  pub id: i32,
  pub mod_user_id: i32,
  pub post_id: i32,
  pub stickied: Option<bool>,
  pub when_: chrono::NaiveDateTime,
  pub mod_user_name: String,
  pub post_name: String,
  pub community_id: i32,
  pub community_name: String,
}

impl ModStickyPostView {
  pub fn list(
    conn: &dgraph_tonic::Client,
    from_community_id: Option<i32>,
    from_mod_user_id: Option<i32>,
    page: Option<i64>,
    limit: Option<i64>,
  ) -> Result<Vec<Self>, Error> {
    use super::moderator_views::mod_sticky_post_view::dsl::*;
    let mut query = mod_sticky_post_view.into_boxed();

    let (limit, offset) = limit_and_offset(page, limit);

    if let Some(from_community_id) = from_community_id {
      query = query.filter(community_id.eq(from_community_id));
    };

    if let Some(from_mod_user_id) = from_mod_user_id {
      query = query.filter(mod_user_id.eq(from_mod_user_id));
    };

    query
      .limit(limit)
      .offset(offset)
      .order_by(when_.desc())
      .load::<Self>(conn)
  }
}



#[derive(
  PartialEq, Debug, Serialize, Deserialize, Clone,
)]
pub struct ModRemoveCommentView {
  pub id: i32,
  pub mod_user_id: i32,
  pub comment_id: i32,
  pub reason: Option<String>,
  pub removed: Option<bool>,
  pub when_: chrono::NaiveDateTime,
  pub mod_user_name: String,
  pub comment_user_id: i32,
  pub comment_user_name: String,
  pub comment_content: String,
  pub post_id: i32,
  pub post_name: String,
  pub community_id: i32,
  pub community_name: String,
}

impl ModRemoveCommentView {
  pub fn list(
    conn: &dgraph_tonic::Client,
    from_community_id: Option<i32>,
    from_mod_user_id: Option<i32>,
    page: Option<i64>,
    limit: Option<i64>,
  ) -> Result<Vec<Self>, Error> {
    use super::moderator_views::mod_remove_comment_view::dsl::*;
    let mut query = mod_remove_comment_view.into_boxed();

    let (limit, offset) = limit_and_offset(page, limit);

    if let Some(from_community_id) = from_community_id {
      query = query.filter(community_id.eq(from_community_id));
    };

    if let Some(from_mod_user_id) = from_mod_user_id {
      query = query.filter(mod_user_id.eq(from_mod_user_id));
    };

    query
      .limit(limit)
      .offset(offset)
      .order_by(when_.desc())
      .load::<Self>(conn)
  }
}



#[derive(
  PartialEq, Debug, Serialize, Deserialize, Clone,
)]
pub struct ModRemoveCommunityView {
  pub id: i32,
  pub mod_user_id: i32,
  pub community_id: i32,
  pub reason: Option<String>,
  pub removed: Option<bool>,
  pub expires: Option<chrono::NaiveDateTime>,
  pub when_: chrono::NaiveDateTime,
  pub mod_user_name: String,
  pub community_name: String,
}

impl ModRemoveCommunityView {
  pub fn list(
    conn: &dgraph_tonic::Client,
    from_mod_user_id: Option<i32>,
    page: Option<i64>,
    limit: Option<i64>,
  ) -> Result<Vec<Self>, Error> {
    use super::moderator_views::mod_remove_community_view::dsl::*;
    let mut query = mod_remove_community_view.into_boxed();

    let (limit, offset) = limit_and_offset(page, limit);

    if let Some(from_mod_user_id) = from_mod_user_id {
      query = query.filter(mod_user_id.eq(from_mod_user_id));
    };

    query
      .limit(limit)
      .offset(offset)
      .order_by(when_.desc())
      .load::<Self>(conn)
  }
}



#[derive(
  PartialEq, Debug, Serialize, Deserialize, Clone,
)]
pub struct ModBanFromCommunityView {
  pub id: i32,
  pub mod_user_id: i32,
  pub other_user_id: i32,
  pub community_id: i32,
  pub reason: Option<String>,
  pub banned: Option<bool>,
  pub expires: Option<chrono::NaiveDateTime>,
  pub when_: chrono::NaiveDateTime,
  pub mod_user_name: String,
  pub other_user_name: String,
  pub community_name: String,
}

impl ModBanFromCommunityView {
  pub fn list(
    conn: &dgraph_tonic::Client,
    from_community_id: Option<i32>,
    from_mod_user_id: Option<i32>,
    page: Option<i64>,
    limit: Option<i64>,
  ) -> Result<Vec<Self>, Error> {
    use super::moderator_views::mod_ban_from_community_view::dsl::*;
    let mut query = mod_ban_from_community_view.into_boxed();

    let (limit, offset) = limit_and_offset(page, limit);

    if let Some(from_community_id) = from_community_id {
      query = query.filter(community_id.eq(from_community_id));
    };

    if let Some(from_mod_user_id) = from_mod_user_id {
      query = query.filter(mod_user_id.eq(from_mod_user_id));
    };

    query
      .limit(limit)
      .offset(offset)
      .order_by(when_.desc())
      .load::<Self>(conn)
  }
}



#[derive(
  PartialEq, Debug, Serialize, Deserialize, Clone,
)]
pub struct ModBanView {
  pub id: i32,
  pub mod_user_id: i32,
  pub other_user_id: i32,
  pub reason: Option<String>,
  pub banned: Option<bool>,
  pub expires: Option<chrono::NaiveDateTime>,
  pub when_: chrono::NaiveDateTime,
  pub mod_user_name: String,
  pub other_user_name: String,
}

impl ModBanView {
  pub fn list(
    conn: &dgraph_tonic::Client,
    from_mod_user_id: Option<i32>,
    page: Option<i64>,
    limit: Option<i64>,
  ) -> Result<Vec<Self>, Error> {
    use super::moderator_views::mod_ban_view::dsl::*;
    let mut query = mod_ban_view.into_boxed();

    let (limit, offset) = limit_and_offset(page, limit);

    if let Some(from_mod_user_id) = from_mod_user_id {
      query = query.filter(mod_user_id.eq(from_mod_user_id));
    };

    query
      .limit(limit)
      .offset(offset)
      .order_by(when_.desc())
      .load::<Self>(conn)
  }
}



#[derive(
  PartialEq, Debug, Serialize, Deserialize, Clone,
)]
pub struct ModAddCommunityView {
  pub id: i32,
  pub mod_user_id: i32,
  pub other_user_id: i32,
  pub community_id: i32,
  pub removed: Option<bool>,
  pub when_: chrono::NaiveDateTime,
  pub mod_user_name: String,
  pub other_user_name: String,
  pub community_name: String,
}

impl ModAddCommunityView {
  pub fn list(
    conn: &dgraph_tonic::Client,
    from_community_id: Option<i32>,
    from_mod_user_id: Option<i32>,
    page: Option<i64>,
    limit: Option<i64>,
  ) -> Result<Vec<Self>, Error> {
    use super::moderator_views::mod_add_community_view::dsl::*;
    let mut query = mod_add_community_view.into_boxed();

    let (limit, offset) = limit_and_offset(page, limit);

    if let Some(from_community_id) = from_community_id {
      query = query.filter(community_id.eq(from_community_id));
    };

    if let Some(from_mod_user_id) = from_mod_user_id {
      query = query.filter(mod_user_id.eq(from_mod_user_id));
    };

    query
      .limit(limit)
      .offset(offset)
      .order_by(when_.desc())
      .load::<Self>(conn)
  }
}



#[derive(
  PartialEq, Debug, Serialize, Deserialize, Clone,
)]
pub struct ModAddView {
  pub id: i32,
  pub mod_user_id: i32,
  pub other_user_id: i32,
  pub removed: Option<bool>,
  pub when_: chrono::NaiveDateTime,
  pub mod_user_name: String,
  pub other_user_name: String,
}

impl ModAddView {
  pub fn list(
    conn: &dgraph_tonic::Client,
    from_mod_user_id: Option<i32>,
    page: Option<i64>,
    limit: Option<i64>,
  ) -> Result<Vec<Self>, Error> {
    use super::moderator_views::mod_add_view::dsl::*;
    let mut query = mod_add_view.into_boxed();

    let (limit, offset) = limit_and_offset(page, limit);

    if let Some(from_mod_user_id) = from_mod_user_id {
      query = query.filter(mod_user_id.eq(from_mod_user_id));
    };

    query
      .limit(limit)
      .offset(offset)
      .order_by(when_.desc())
      .load::<Self>(conn)
  }
}
