use crate::http::{
  comment::get_apub_comment,
  community::{
    community_inbox,
    get_apub_community_followers,
    get_apub_community_http,
    get_apub_community_moderators,
    get_apub_community_outbox,
  },
  get_activity,
  person::{get_apub_person_http, get_apub_person_outbox, person_inbox},
  post::get_apub_post,
  shared_inbox,
};
use actix_web::{dev::RequestHead, guard::Guard, http::Method, *};
use http_signature_normalization_actix::digest::middleware::VerifyDigest;
use lemmy_utils::settings::structs::Settings;
use sha2::{Digest, Sha256};

pub fn config(cfg: &mut web::ServiceConfig, settings: &Settings) {
  if settings.federation.enabled {
    println!("federation enabled, host is {}", settings.hostname);

    cfg
      .route(
        "/c/{community_name}",
        web::get().to(get_apub_community_http),
      )
      .route(
        "/c/{community_name}/followers",
        web::get().to(get_apub_community_followers),
      )
      .route(
        "/c/{community_name}/outbox",
        web::get().to(get_apub_community_outbox),
      )
      .route(
        "/c/{community_name}/moderators",
        web::get().to(get_apub_community_moderators),
      )
      .route("/u/{user_name}", web::get().to(get_apub_person_http))
      .route(
        "/u/{user_name}/outbox",
        web::get().to(get_apub_person_outbox),
      )
      .route("/post/{post_id}", web::get().to(get_apub_post))
      .route("/comment/{comment_id}", web::get().to(get_apub_comment))
      .route("/activities/{type_}/{id}", web::get().to(get_activity));

    cfg.service(
      web::scope("")
        .wrap(VerifyDigest::new(Sha256::new()))
        .guard(InboxRequestGuard)
        .route("/c/{community_name}/inbox", web::post().to(community_inbox))
        .route("/u/{user_name}/inbox", web::post().to(person_inbox))
        .route("/inbox", web::post().to(shared_inbox)),
    );
  }
}

/// Without this, things like webfinger or RSS feeds stop working, as all requests seem to get
/// routed into the inbox service (because it covers the root path). So we filter out anything that
/// definitely can't be an inbox request (based on Accept header and request method).
struct InboxRequestGuard;

impl Guard for InboxRequestGuard {
  fn check(&self, request: &RequestHead) -> bool {
    if request.method != Method::POST {
      return false;
    }
    if let Some(val) = request.headers.get("Content-Type") {
      return val
        .to_str()
        .expect("Content-Type header contains non-ascii chars.")
        .starts_with("application/");
    }
    false
  }
}
