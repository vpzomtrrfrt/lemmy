use crate::{
  naive_now,
  newtypes::LocalUserId,
  schema::local_user::dsl::*,
  source::local_user::{LocalUser, LocalUserForm},
  traits::Crud,
};
use bcrypt::{hash, DEFAULT_COST};
use diesel::{dsl::*, result::Error, *};

mod safe_settings_type {
  use crate::{
    schema::local_user::columns::*,
    source::local_user::LocalUser,
    traits::ToSafeSettings,
  };

  type Columns = (
    id,
    person_id,
    email,
    show_nsfw,
    theme,
    default_sort_type,
    default_listing_type,
    lang,
    show_avatars,
    send_notifications_to_email,
    validator_time,
    show_bot_accounts,
    show_scores,
    show_read_posts,
    show_new_post_notifs,
  );

  impl ToSafeSettings for LocalUser {
    type SafeSettingsColumns = Columns;

    /// Includes everything but the hashed password
    fn safe_settings_columns_tuple() -> Self::SafeSettingsColumns {
      (
        id,
        person_id,
        email,
        show_nsfw,
        theme,
        default_sort_type,
        default_listing_type,
        lang,
        show_avatars,
        send_notifications_to_email,
        validator_time,
        show_bot_accounts,
        show_scores,
        show_read_posts,
        show_new_post_notifs,
      )
    }
  }
}

impl LocalUser {
  pub fn register(conn: &PgConnection, form: &LocalUserForm) -> Result<Self, Error> {
    let mut edited_user = form.clone();
    let password_hash =
      hash(&form.password_encrypted, DEFAULT_COST).expect("Couldn't hash password");
    edited_user.password_encrypted = password_hash;

    Self::create(conn, &edited_user)
  }

  pub fn update_password(
    conn: &PgConnection,
    local_user_id: LocalUserId,
    new_password: &str,
  ) -> Result<Self, Error> {
    let password_hash = hash(new_password, DEFAULT_COST).expect("Couldn't hash password");

    diesel::update(local_user.find(local_user_id))
      .set((
        password_encrypted.eq(password_hash),
        validator_time.eq(naive_now()),
      ))
      .get_result::<Self>(conn)
  }
}

impl Crud for LocalUser {
  type Form = LocalUserForm;
  type IdType = LocalUserId;
  fn read(conn: &PgConnection, local_user_id: LocalUserId) -> Result<Self, Error> {
    local_user.find(local_user_id).first::<Self>(conn)
  }
  fn delete(conn: &PgConnection, local_user_id: LocalUserId) -> Result<usize, Error> {
    diesel::delete(local_user.find(local_user_id)).execute(conn)
  }
  fn create(conn: &PgConnection, form: &LocalUserForm) -> Result<Self, Error> {
    insert_into(local_user)
      .values(form)
      .get_result::<Self>(conn)
  }
  fn update(
    conn: &PgConnection,
    local_user_id: LocalUserId,
    form: &LocalUserForm,
  ) -> Result<Self, Error> {
    diesel::update(local_user.find(local_user_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}
