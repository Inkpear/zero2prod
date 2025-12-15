use actix_web::{HttpResponse, web};
use actix_web_flash_messages::FlashMessage;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

use crate::{
    authentication::{AuthError, Credentials, UserId, validate_credentials},
    routes::admin::dashboard::get_username,
    utils::{e500, see_other},
};

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

pub async fn change_password(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();

    if form.new_password.expose_secret() != form.new_password_check.expose_secret() {
        FlashMessage::error("You entered tow different passwords - the field values must match.")
            .send();
        return Ok(see_other("/admin/password"));
    }
    let username = get_username(*user_id, &pool).await.map_err(e500)?;
    let credentials = Credentials {
        username,
        password: form.0.current_password,
    };
    if let Err(e) = validate_credentials(credentials, &pool).await {
        return match e {
            AuthError::InvalidCredentials(_) => {
                FlashMessage::error("The current password is incorrect.").send();
                Ok(see_other("/admin/password"))
            }
            AuthError::UnexpectedError(e) => Err(e500(e).into()),
        };
    }
    if let Err(msg) = check_password_is_valid(&form.0.new_password) {
        FlashMessage::error(msg).send();
        return Ok(see_other("/admin/password"));
    }

    crate::authentication::change_password(*user_id, form.0.new_password, &pool)
        .await
        .map_err(e500)?;
    FlashMessage::error("Your password has been changed.").send();

    Ok(see_other("/admin/password"))
}

fn check_password_is_valid(password: &Secret<String>) -> Result<(), String> {
    if password.expose_secret().len() <= 12 {
        Err("the length of password is too shorter.".to_string())
    } else if password.expose_secret().len() > 128 {
        Err("the length of password is too longer.".to_string())
    } else {
        Ok(())
    }
}
