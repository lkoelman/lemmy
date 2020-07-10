use crate::db::*;
use dgraph_monkey::builder::{
  QueryBuilder, DgraphFunction as DFn
};

// TODO: annotate with Serde attributes for de-serializing from DB
#[derive(
  PartialEq, Debug, Serialize, Deserialize, Clone,
)]
pub struct CommentView {
  pub id: i32,
  pub creator_id: i32,
  pub post_id: i32,
  pub parent_id: Option<i32>,
  pub content: String,
  pub removed: bool,
  pub read: bool,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
  pub deleted: bool,
  pub community_id: i32,
  pub community_name: String,
  pub banned: bool,
  pub banned_from_community: bool,
  pub creator_name: String,
  pub creator_avatar: Option<String>,
  pub score: i64,
  pub upvotes: i64,
  pub downvotes: i64,
  pub hot_rank: i32,
  pub user_id: Option<i32>,
  pub my_vote: Option<i32>,
  pub subscribed: Option<bool>,
  pub saved: Option<bool>,
}

pub struct CommentQueryBuilder<'a> {
  conn: &'a dgraph_tonic::Client,
  query: QueryBuilder,
  listing_type: ListingType,
  sort: &'a SortType,
  for_community_id: Option<i32>,
  for_post_id: Option<i32>,
  for_creator_id: Option<i32>,
  search_term: Option<String>,
  my_user_id: Option<i32>,
  saved_only: bool,
  page: Option<i64>,
  limit: Option<i64>,
}

// TODO: alternative: use query wit vars
//- https://dgraph.io/docs/query-language/#graphql-variables
//- https://docs.rs/dgraph-tonic/0.6.3/dgraph_tonic/trait.Query.html#tymethod.query_with_vars

const COMMENT_QUERY = r#"
query comment_view($user_id: int, $now: dateTime) {

  # pre-filter eligible comments
  subscribed as var(func: uid(&a)) {
    ...
  }

  # pre-filter eligible comments
  saved as var(func: uid($a)) {
    ...
  }

  # Calculate hot rank for each post
  hot_posts as var(func: type(User)) {
    User.likesPost @facets(valance : score) {

      # TODO: WRONG: score must be the sum of likes
      score as sum(valence)

      sign as math(cond(score < 0, -1, 1))

      hot_rank as math(10000 * sign * 
         log(1 + (score * sign)) / 
         pow((((since($now) - since(published))/3600) + 2), 1.8))
    }

  }

  root(func: uid(A,B,C)) @filter(type(Comment)) {
    # TODO: aggregate fields of CommentView here
    expand(Comment)
    post {
      hot_rank : val(hot_rank)
    }
  }
}

"#;

impl<'a> CommentQueryBuilder<'a> {

  pub fn create(conn: &'a dgraph_tonic::Client) -> Self {

    let query = QueryBuilder::new(conn, Some("users".to_string()));

    CommentQueryBuilder {
      conn,
      query,
      listing_type: ListingType::All,
      sort: &SortType::New,
      for_community_id: None,
      for_post_id: None,
      for_creator_id: None,
      search_term: None,
      my_user_id: None,
      saved_only: false,
      page: None,
      limit: None,
    }
  }

  pub fn listing_type(mut self, listing_type: ListingType) -> Self {
    self.listing_type = listing_type;
    self
  }

  pub fn sort(mut self, sort: &'a SortType) -> Self {
    self.sort = sort;
    self
  }

  pub fn for_post_id<T: MaybeOptional<i32>>(mut self, for_post_id: T) -> Self {
    self.for_post_id = for_post_id.get_optional();
    self
  }

  pub fn for_creator_id<T: MaybeOptional<i32>>(mut self, for_creator_id: T) -> Self {
    self.for_creator_id = for_creator_id.get_optional();
    self
  }

  pub fn for_community_id<T: MaybeOptional<i32>>(mut self, for_community_id: T) -> Self {
    self.for_community_id = for_community_id.get_optional();
    self
  }

  pub fn search_term<T: MaybeOptional<String>>(mut self, search_term: T) -> Self {
    self.search_term = search_term.get_optional();
    self
  }

  pub fn my_user_id<T: MaybeOptional<i32>>(mut self, my_user_id: T) -> Self {
    self.my_user_id = my_user_id.get_optional();
    self
  }

  pub fn saved_only(mut self, saved_only: bool) -> Self {
    self.saved_only = saved_only;
    self
  }

  pub fn page<T: MaybeOptional<i64>>(mut self, page: T) -> Self {
    self.page = page.get_optional();
    self
  }

  pub fn limit<T: MaybeOptional<i64>>(mut self, limit: T) -> Self {
    self.limit = limit.get_optional();
    self
  }

  /**
   * List results of query.
   */
  pub fn list2(self) -> Result<Vec<CommentView>, Error> {

    let mut query = String::with_capacity(500);


    // Filter comment that are saved or occur in followed communities
    if let Some(my_user_id) = self.my_user_id {

      query.push_str(r#"
      followed as var(func: uid($a)) {
        User.followsCommunity {
          Community.posts {
            filter1 as Post.comments
          }
        }
      }"#);

      // Comments where <User> <User.SavedComment> <Comment>
      if self.saved_only {
        query.push_str(r#"
        saved as var(func: uid($a)) {
          filter2 as User.savedComments
        }"#);
      }
    }

    // Comments where <Comment> <Comment.creator> <User>
    if let Some(for_creator_id) = self.for_creator_id {

      query.add_query_var(None)
           .root_query(DFn::uid(for_creator_id))
           .edge_as("User.comments", "filter_2")
           .pop(1);
    };

    // Comments posted in community, i.e.:
    // [Comment] <Comment.post> [Post] <Post.community> [Community]
    if let Some(for_community_id) = self.for_community_id {

      query.add_query_var(None)
            .root_query(DFn::uid(for_community_id))
            .edge("Community.posts");
            .filter_if("uid", self.for_post_id)
            .edge_as("Post.comments", "filter_3");
            .filter_if("allofterms", "content", search_term)
            .pop(2);
    }


    // FIXME: what relationship do they mean?
    // Comments where [User] <subscribe> [Community] <posts> [Post] <comments> [Comment]
    if let ListingType::Subscribed = self.listing_type {
      query = query.filter(subscribed.eq(true));

      // Narrow down comments in communities that user is subscribed to
      query.add_query_var(None)
           .root_query(DFn::uid(my_user_id))
           .edge("User.followsCommunity")
           .edge("Community.posts")
           .edge_as("Post.comments", "filter_1")
           .pop(2);
    }

    // Make root query, using query vars
    let num_filters = query.query_vars.len();
    let uid_filters = (1..=num_filters).map(|i| format!("filter_{}", i))
                                       .collect::<Vec<_>>()
                                       .join(", ");

    let candidates = format!("uid({})", uid_filters);
    query = query.root_query(DFn::uid(candidates));

    // Hot rank calculation
    // https://github.com/LemmyNet/lemmy/blob/397f65c81ef17f4c7e5e2847155347ae1377e25b/server/migrations/2019-03-30-212058_create_post_view/up.sql
    // math(10000*(score / max(score, -score))*log(1 + (score * cond(score<0,-1,1))) / pow(((since(now - published)/3600) + 2), 1.8))

    // Filter by data and and order/sort
    query = match self.sort {
      SortType::Hot => query
        .order_by(hot_rank.desc())
        .then_order_by(published.desc()),
      SortType::New => query.order_by("published", "desc"),
      SortType::TopAll => query.order_by("score", "desc"),
      SortType::TopYear => query
        .filter("gt", "published",
          (naive_now() - chrono::Duration::days(365)).to_string())
        .order_by("score", "desc"),
      SortType::TopMonth => query
        .filter("gt", "published",
          (naive_now() - chrono::Duration::days(30)).to_string())
        .order_by("score", "desc"),
      SortType::TopWeek => query
        .filter("gt", "published",
          (naive_now() - chrono::Duration::weeks(1)).to_string())
        .order_by("score", "desc"),
      SortType::TopDay => query
        .filter("gt", "published",
        (naive_now() - chrono::Duration::days(1)).to_string())
          .order_by("score", "desc"),
      // _ => query.order_by(published.desc()),
    };

    

    let (limit, offset) = limit_and_offset(self.page, self.limit);

    // Note: deleted and removed comments are done on the front side
    let resp = query.execute();

    // TODO: load into CommentView
    // - flatten (see serde flatten macro attributes)
  }

  /**
   * List results of query.
   */
  pub fn list(self) -> Result<Vec<CommentView>, Error> {

    let mut query = self.query;

    let mut num_filters: usize = 0;

    // The view lets you pass a null user_id, if you're not logged in
    // FIXME: does the CommentView depend on the user asking for it?
    // Filter [Comment] <post> [Post] <community> [Community] <follower> [User]
    if let Some(my_user_id) = self.my_user_id {


      // Comments where [User] <follows> [Community] <posts> [Post] <comments> [Comment]
      query.add_query_var(None)
           .root_query(DFn::uid(my_user_id))
           .edge("User.followsCommunity")
           .edge("Community.posts")
           .edge_as("Post.comments", "filter_1")
           .pop(2);

      // Comments where <User> <User.SavedComment> <Comment>
      if self.saved_only {
        query.add_query_var(None)
              .root_query(DFn::uid(my_user_id))
              .edge_as("User.savedComments", "filter_2")
              .pop(1);
      }
    }

    // Comments where <Comment> <Comment.creator> <User>
    if let Some(for_creator_id) = self.for_creator_id {

      query.add_query_var(None)
           .root_query(DFn::uid(for_creator_id))
           .edge_as("User.comments", "filter_2")
           .pop(1);
    };

    // Comments posted in community, i.e.:
    // [Comment] <Comment.post> [Post] <Post.community> [Community]
    if let Some(for_community_id) = self.for_community_id {

      query.add_query_var(None)
            .root_query(DFn::uid(for_community_id))
            .edge("Community.posts");
            .filter_if("uid", self.for_post_id)
            .edge_as("Post.comments", "filter_3");
            .filter_if("allofterms", "content", search_term)
            .pop(2);
    }


    // FIXME: what relationship do they mean?
    // Comments where [User] <subscribe> [Community] <posts> [Post] <comments> [Comment]
    if let ListingType::Subscribed = self.listing_type {
      query = query.filter(subscribed.eq(true));

      // Narrow down comments in communities that user is subscribed to
      query.add_query_var(None)
           .root_query(DFn::uid(my_user_id))
           .edge("User.followsCommunity")
           .edge("Community.posts")
           .edge_as("Post.comments", "filter_1")
           .pop(2);
    }

    // Make root query, using query vars
    let num_filters = query.query_vars.len();
    let uid_filters = (1..=num_filters).map(|i| format!("filter_{}", i))
                                       .collect::<Vec<_>>()
                                       .join(", ");

    let candidates = format!("uid({})", uid_filters);
    query = query.root_query(DFn::uid(candidates));

    // Hot rank calculation
    // https://github.com/LemmyNet/lemmy/blob/397f65c81ef17f4c7e5e2847155347ae1377e25b/server/migrations/2019-03-30-212058_create_post_view/up.sql
    // math(10000*(score / max(score, -score))*log(1 + (score * cond(score<0,-1,1))) / pow(((since(now - published)/3600) + 2), 1.8))

    // Filter by data and and order/sort
    query = match self.sort {
      SortType::Hot => query
        .order_by(hot_rank.desc())
        .then_order_by(published.desc()),
      SortType::New => query.order_by("published", "desc"),
      SortType::TopAll => query.order_by("score", "desc"),
      SortType::TopYear => query
        .filter("gt", "published",
          (naive_now() - chrono::Duration::days(365)).to_string())
        .order_by("score", "desc"),
      SortType::TopMonth => query
        .filter("gt", "published",
          (naive_now() - chrono::Duration::days(30)).to_string())
        .order_by("score", "desc"),
      SortType::TopWeek => query
        .filter("gt", "published",
          (naive_now() - chrono::Duration::weeks(1)).to_string())
        .order_by("score", "desc"),
      SortType::TopDay => query
        .filter("gt", "published",
        (naive_now() - chrono::Duration::days(1)).to_string())
          .order_by("score", "desc"),
      // _ => query.order_by(published.desc()),
    };

    

    let (limit, offset) = limit_and_offset(self.page, self.limit);

    // Note: deleted and removed comments are done on the front side
    let resp = query.execute();

    // TODO: load into CommentView
    // - flatten (see serde flatten macro attributes)
  }
}


#[derive(
  PartialEq, Debug, Serialize, Deserialize, Clone,
)]
pub struct ReplyView {
  pub id: i32,
  pub creator_id: i32,
  pub post_id: i32,
  pub parent_id: Option<i32>,
  pub content: String,
  pub removed: bool,
  pub read: bool,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
  pub deleted: bool,
  pub community_id: i32,
  pub community_name: String,
  pub banned: bool,
  pub banned_from_community: bool,
  pub creator_name: String,
  pub creator_avatar: Option<String>,
  pub score: i64,
  pub upvotes: i64,
  pub downvotes: i64,
  pub hot_rank: i32,
  pub user_id: Option<i32>,
  pub my_vote: Option<i32>,
  pub subscribed: Option<bool>,
  pub saved: Option<bool>,
  pub recipient_id: i32,
}

pub struct ReplyQueryBuilder<'a> {
  conn: &'a dgraph_tonic::Client,
  query: super::comment_view::reply_view::BoxedQuery<'a, Pg>,
  for_user_id: i32,
  sort: &'a SortType,
  unread_only: bool,
  page: Option<i64>,
  limit: Option<i64>,
}

impl<'a> ReplyQueryBuilder<'a> {
  pub fn create(conn: &'a dgraph_tonic::Client, for_user_id: i32) -> Self {
    use super::comment_view::reply_view::dsl::*;

    let query = reply_view.into_boxed();

    ReplyQueryBuilder {
      conn,
      query,
      for_user_id,
      sort: &SortType::New,
      unread_only: false,
      page: None,
      limit: None,
    }
  }

  pub fn sort(mut self, sort: &'a SortType) -> Self {
    self.sort = sort;
    self
  }

  pub fn unread_only(mut self, unread_only: bool) -> Self {
    self.unread_only = unread_only;
    self
  }

  pub fn page<T: MaybeOptional<i64>>(mut self, page: T) -> Self {
    self.page = page.get_optional();
    self
  }

  pub fn limit<T: MaybeOptional<i64>>(mut self, limit: T) -> Self {
    self.limit = limit.get_optional();
    self
  }

  pub fn list(self) -> Result<Vec<ReplyView>, Error> {
    use super::comment_view::reply_view::dsl::*;

    let mut query = self.query;

    query = query
      .filter(user_id.eq(self.for_user_id))
      .filter(recipient_id.eq(self.for_user_id))
      .filter(deleted.eq(false))
      .filter(removed.eq(false));

    if self.unread_only {
      query = query.filter(read.eq(false));
    }

    query = match self.sort {
      // SortType::Hot => query.order_by(hot_rank.desc()),
      SortType::New => query.order_by(published.desc()),
      SortType::TopAll => query.order_by(score.desc()),
      SortType::TopYear => query
        .filter(published.gt(now - 1.years()))
        .order_by(score.desc()),
      SortType::TopMonth => query
        .filter(published.gt(now - 1.months()))
        .order_by(score.desc()),
      SortType::TopWeek => query
        .filter(published.gt(now - 1.weeks()))
        .order_by(score.desc()),
      SortType::TopDay => query
        .filter(published.gt(now - 1.days()))
        .order_by(score.desc()),
      _ => query.order_by(published.desc()),
    };

    let (limit, offset) = limit_and_offset(self.page, self.limit);
    query
      .limit(limit)
      .offset(offset)
      .load::<ReplyView>(self.conn)
  }
}

/*
#[cfg(test)]
mod tests {
  use super::super::comment::*;
  use super::super::community::*;
  use super::super::post::*;
  use super::super::user::*;
  use crate::db::*;
  #[test]
  fn test_crud() {
    let conn = establish_unpooled_connection();

    let new_user = UserForm {
      name: "timmy".into(),
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
      name: "test community 5".to_string(),
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
      name: "A test post 2".into(),
      creator_id: inserted_user.id,
      url: None,
      body: None,
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
      content: "A test comment 32".into(),
      creator_id: inserted_user.id,
      post_id: inserted_post.id,
      parent_id: None,
      removed: None,
      deleted: None,
      read: None,
      updated: None,
    };

    let inserted_comment = Comment::create(&conn, &comment_form).unwrap();

    let comment_like_form = CommentLikeForm {
      comment_id: inserted_comment.id,
      post_id: inserted_post.id,
      user_id: inserted_user.id,
      score: 1,
    };

    let _inserted_comment_like = CommentLike::like(&conn, &comment_like_form).unwrap();

    let expected_comment_view_no_user = CommentView {
      id: inserted_comment.id,
      content: "A test comment 32".into(),
      creator_id: inserted_user.id,
      post_id: inserted_post.id,
      community_id: inserted_community.id,
      community_name: inserted_community.name.to_owned(),
      parent_id: None,
      removed: false,
      deleted: false,
      read: false,
      banned: false,
      banned_from_community: false,
      published: inserted_comment.published,
      updated: None,
      creator_name: inserted_user.name.to_owned(),
      creator_avatar: None,
      score: 1,
      downvotes: 0,
      hot_rank: 0,
      upvotes: 1,
      user_id: None,
      my_vote: None,
      subscribed: None,
      saved: None,
    };

    let expected_comment_view_with_user = CommentView {
      id: inserted_comment.id,
      content: "A test comment 32".into(),
      creator_id: inserted_user.id,
      post_id: inserted_post.id,
      community_id: inserted_community.id,
      community_name: inserted_community.name.to_owned(),
      parent_id: None,
      removed: false,
      deleted: false,
      read: false,
      banned: false,
      banned_from_community: false,
      published: inserted_comment.published,
      updated: None,
      creator_name: inserted_user.name.to_owned(),
      creator_avatar: None,
      score: 1,
      downvotes: 0,
      hot_rank: 0,
      upvotes: 1,
      user_id: Some(inserted_user.id),
      my_vote: Some(1),
      subscribed: None,
      saved: None,
    };

    let mut read_comment_views_no_user = CommentQueryBuilder::create(&conn)
      .for_post_id(inserted_post.id)
      .list()
      .unwrap();
    read_comment_views_no_user[0].hot_rank = 0;

    let mut read_comment_views_with_user = CommentQueryBuilder::create(&conn)
      .for_post_id(inserted_post.id)
      .my_user_id(inserted_user.id)
      .list()
      .unwrap();
    read_comment_views_with_user[0].hot_rank = 0;

    let like_removed = CommentLike::remove(&conn, &comment_like_form).unwrap();
    let num_deleted = Comment::delete(&conn, inserted_comment.id).unwrap();
    Post::delete(&conn, inserted_post.id).unwrap();
    Community::delete(&conn, inserted_community.id).unwrap();
    User_::delete(&conn, inserted_user.id).unwrap();

    assert_eq!(expected_comment_view_no_user, read_comment_views_no_user[0]);
    assert_eq!(
      expected_comment_view_with_user,
      read_comment_views_with_user[0]
    );
    assert_eq!(1, num_deleted);
    assert_eq!(1, like_removed);
  }
}

*/