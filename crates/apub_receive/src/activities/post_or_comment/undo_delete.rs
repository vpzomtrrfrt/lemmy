use crate::activities::{
  comment::send_websocket_message as send_comment_message,
  post::send_websocket_message as send_post_message,
  post_or_comment::delete::DeletePostOrComment,
  verify_activity,
  verify_person_in_community,
};
use activitystreams::activity::kind::UndoType;
use lemmy_api_common::blocking;
use lemmy_apub::{fetcher::objects::get_or_fetch_and_insert_post_or_comment, PostOrComment};
use lemmy_apub_lib::{verify_urls_match, ActivityCommonFields, ActivityHandler, PublicUrl};
use lemmy_db_queries::source::{comment::Comment_, post::Post_};
use lemmy_db_schema::source::{comment::Comment, post::Post};
use lemmy_utils::LemmyError;
use lemmy_websocket::{LemmyContext, UserOperationCrud};
use url::Url;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UndoDeletePostOrComment {
  to: PublicUrl,
  object: DeletePostOrComment,
  cc: [Url; 1],
  #[serde(rename = "type")]
  kind: UndoType,
  #[serde(flatten)]
  common: ActivityCommonFields,
}

#[async_trait::async_trait(?Send)]
impl ActivityHandler for UndoDeletePostOrComment {
  async fn verify(
    &self,
    context: &LemmyContext,
    request_counter: &mut i32,
  ) -> Result<(), LemmyError> {
    verify_activity(self.common())?;
    verify_person_in_community(&self.common().actor, &self.cc, context, request_counter).await?;
    verify_urls_match(&self.common.actor, &self.object.common().actor)?;
    self.object.verify(context, request_counter).await?;
    Ok(())
  }

  async fn receive(
    &self,
    context: &LemmyContext,
    request_counter: &mut i32,
  ) -> Result<(), LemmyError> {
    match get_or_fetch_and_insert_post_or_comment(&self.object.object, context, request_counter)
      .await?
    {
      PostOrComment::Post(post) => {
        let deleted_post = blocking(context.pool(), move |conn| {
          Post::update_deleted(conn, post.id, false)
        })
        .await??;
        send_post_message(deleted_post.id, UserOperationCrud::EditPost, context).await
      }
      PostOrComment::Comment(comment) => {
        let deleted_comment = blocking(context.pool(), move |conn| {
          Comment::update_deleted(conn, comment.id, false)
        })
        .await??;
        send_comment_message(
          deleted_comment.id,
          vec![],
          UserOperationCrud::EditComment,
          context,
        )
        .await
      }
    }
  }

  fn common(&self) -> &ActivityCommonFields {
    &self.common
  }
}