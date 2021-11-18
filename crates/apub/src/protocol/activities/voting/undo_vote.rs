use crate::{objects::person::ApubPerson, protocol::activities::voting::vote::Vote};
use activitystreams::{activity::kind::UndoType, primitives::OneOrMany, unparsed::Unparsed};
use lemmy_apub_lib::object_id::ObjectId;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UndoVote {
  pub(crate) actor: ObjectId<ApubPerson>,
  pub(crate) to: Option<OneOrMany<Url>>,
  pub(crate) object: Vote,
  pub(crate) cc: Option<OneOrMany<Url>>,
  #[serde(rename = "type")]
  pub(crate) kind: UndoType,
  pub(crate) id: Url,
  #[serde(flatten)]
  pub(crate) unparsed: Unparsed,
}
