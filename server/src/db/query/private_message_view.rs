use crate::db::*;


// The faked schema since diesel doesn't do views




#[derive(
  PartialEq, Debug, Serialize, Deserialize, Clone,
)]
pub struct PrivateMessageView {
  pub id: i32,
  pub creator_id: i32,
  pub recipient_id: i32,
  pub content: String,
  pub deleted: bool,
  pub read: bool,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
  pub creator_name: String,
  pub creator_avatar: Option<String>,
  pub recipient_name: String,
  pub recipient_avatar: Option<String>,
}

pub struct PrivateMessageQueryBuilder<'a> {
  conn: &'a dgraph_tonic::Client,
  query: super::private_message_view::private_message_mview::BoxedQuery<'a, Pg>,
  for_recipient_id: i32,
  unread_only: bool,
  page: Option<i64>,
  limit: Option<i64>,
}

impl<'a> PrivateMessageQueryBuilder<'a> {
  pub fn create(conn: &'a dgraph_tonic::Client, for_recipient_id: i32) -> Self {
    use super::private_message_view::private_message_mview::dsl::*;

    let query = private_message_mview.into_boxed();

    PrivateMessageQueryBuilder {
      conn,
      query,
      for_recipient_id,
      unread_only: false,
      page: None,
      limit: None,
    }
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

  pub fn list(self) -> Result<Vec<PrivateMessageView>, Error> {
    use super::private_message_view::private_message_mview::dsl::*;

    let mut query = self.query.filter(deleted.eq(false));

    // If its unread, I only want the ones to me
    if self.unread_only {
      query = query
        .filter(read.eq(false))
        .filter(recipient_id.eq(self.for_recipient_id));
    }
    // Otherwise, I want the ALL view to show both sent and received
    else {
      query = query.filter(
        recipient_id
          .eq(self.for_recipient_id)
          .or(creator_id.eq(self.for_recipient_id)),
      )
    }

    let (limit, offset) = limit_and_offset(self.page, self.limit);

    query
      .limit(limit)
      .offset(offset)
      .order_by(published.desc())
      .load::<PrivateMessageView>(self.conn)
  }
}

impl PrivateMessageView {
  pub fn read(conn: &dgraph_tonic::Client, from_private_message_id: i32) -> Result<Self, Error> {
    use super::private_message_view::private_message_view::dsl::*;

    let mut query = private_message_view.into_boxed();

    query = query
      .filter(id.eq(from_private_message_id))
      .order_by(published.desc());

    query.first::<Self>(conn)
  }
}
