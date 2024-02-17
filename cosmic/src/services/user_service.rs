use std::env;
use std::sync::Arc;

use diesel::SaveChangesDsl;
use nanoid::nanoid;
use tera::Context;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::enums::app_message::AppMessage;
use crate::enums::app_message::AppMessage::WarningMessageStr;
use crate::helpers::db::DatabaseConnectionHelper;
use crate::helpers::id_generator::number_generator;
use crate::helpers::string::{password_hash, string};
use crate::helpers::time::current_timestamp;
use crate::helpers::DBPool;
use crate::models::mail::MailBox;
use crate::models::user::{FullName, User, UserRegisterForm, UserSharableData, UserStatus, UserUpdateForm, UsernameAvailability, UserCacheable, UserCacheData};
use crate::repositories::user_repository::UserRepository;
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;
use crate::services::mailer_service::MailerService;
use crate::services::role_service::RoleService;
use crate::services::user_ui_menu_item_service::UserUiMenuItemService;

pub struct UserService;

impl UserService {
    pub fn update(&mut self, pool: &DBPool, id: Uuid, form: UserUpdateForm) -> AppResult<User> {
        UserRepository.update(pool, id, form)
    }

    pub fn create(
        &mut self,
        app: Arc<AppState>,
        role_id: Uuid,
        data: UserRegisterForm,
        status: Option<UserStatus>,
    ) -> AppResult<User> {
        let db_pool = app.database();
        let (code, token) = self.make_verification_codes();

        let email_exists = UserRepository.exists_by_email(db_pool, data.email.clone())?;
        if email_exists.is_some() {
            return Err(AppMessage::WarningMessageStr(
                "User with such email already exists",
            ));
        }

        // save user to db
        let user =
            UserRepository.create(db_pool, data, code.clone(), token.clone(), status.clone())?;

        // assign role
        RoleService.assign_role_to_user(db_pool, user.user_id, role_id, user.user_id)?;

        // give basic sidebar menu items
        let _ = UserUiMenuItemService.give_user_basic_items(db_pool, user.user_id);

        if status.is_none() || status.unwrap() == UserStatus::Pending {
            self.send_email_confirmation(app, user.clone(), code, token);
        }

        Ok(user)
    }

    pub fn username_availability(
        &mut self,
        pool: &DBPool,
        username: String,
    ) -> AppResult<UsernameAvailability> {
        match UserRepository.username_exists(pool, username.clone()) {
            Ok(uname) => Ok(UsernameAvailability {
                username: Some(uname),
                is_available: false,
                message: string("username is unavailable"),
            }),
            Err(AppMessage::DatabaseEntityNotFound) => Ok(UsernameAvailability {
                username: Some(username),
                is_available: true,
                message: string("username is available"),
            }),
            Err(e) => Err(AppMessage::DatabaseErrorMessage(e.to_string())),
        }
    }

    pub fn activate(&mut self, pool: &DBPool, user_id: Uuid) -> AppResult<User> {
        self.change_status(pool, user_id, UserStatus::Active)
    }

    pub fn deactivate(&mut self, pool: &DBPool, user_id: Uuid) -> AppResult<User> {
        self.change_status(pool, user_id, UserStatus::Inactive)
    }

    fn change_status(
        &mut self,
        pool: &DBPool,
        user_id: Uuid,
        status: UserStatus,
    ) -> AppResult<User> {
        let mut user = UserRepository.find_by_id(pool, user_id)?;
        user.status = status.to_string();
        user.save_changes(&mut pool.conn()).into_app_result()
    }

    pub fn change_password(
        &mut self,
        pool: &DBPool,
        user_id: Uuid,
        password: String,
    ) -> AppResult<User> {
        let mut user = UserRepository.find_by_id(pool, user_id)?;
        user.password = password_hash(password);
        user.save_changes(&mut pool.conn()).into_app_result()
    }

    pub fn get_profile(&mut self, pool: &DBPool, id: Uuid) -> AppResult<UserSharableData> {
        let user = UserRepository.find_by_id(pool, id)?.into_sharable();
        Ok(user)
    }

    pub fn verify_email(&mut self, pool: &DBPool, token: String) -> AppResult<User> {
        let mut user = UserRepository.find_by_token(pool, token)?;

        if user.is_verified {
            return Err(WarningMessageStr("Your account has been verified already"));
        }

        user.is_verified = true;
        user.verified_at = Some(current_timestamp());
        user.save_changes(&mut pool.conn()).into_app_result()
    }

    pub fn resend_email_confirmation(
        &mut self,
        app: Arc<AppState>,
        email: String,
    ) -> AppResult<User> {
        let mut user = UserRepository.find_by_email(app.database(), email)?;

        if user.is_verified {
            return Err(WarningMessageStr("Your account has been verified already"));
        }

        // Check verification code (create one if empty)
        if user.verification_token.clone().is_none() {
            let (code, token) = self.make_verification_codes();
            user.verification_code = Some(code);
            user.verification_token = Some(token);
            user.save_changes::<User>(&mut app.database().conn())
                .into_app_result()?;
        }

        self.send_email_confirmation(
            app,
            user.clone(),
            user.verification_code.clone().unwrap(),
            user.verification_token.clone().unwrap(),
        );

        Ok(user)
    }

    pub fn mark_user_started_password_reset(
        &mut self,
        pool: &DBPool,
        mut user: User,
    ) -> AppResult<User> {
        user.has_started_password_reset = true;
        user.save_changes(&mut pool.conn()).into_app_result()
    }

    pub fn mark_user_finished_password_reset(
        &mut self,
        pool: &DBPool,
        mut user: User,
    ) -> AppResult<User> {
        user.has_started_password_reset = false;
        user.save_changes(&mut pool.conn()).into_app_result()
    }

    pub fn change_profile_picture(
        &mut self,
        pool: &DBPool,
        mut user: User,
        file_path: String,
    ) -> AppResult<User> {
        user.profile_picture = Some(file_path);
        user.save_changes(&mut pool.conn()).into_app_result()
    }

    fn make_verification_codes(&mut self) -> (String, String) {
        let code = number_generator(10);
        let token = nanoid!();
        (code, token)
    }

    fn send_email_confirmation(
        &mut self,
        app: Arc<AppState>,
        user: User,
        code: String,
        token: String,
    ) {
        let link = format!(
            "{}/email-verification?token={}",
            env::var("MAILER_FRONTEND_ADDRESS").unwrap(),
            token
        );

        let mut context = Context::new();
        context.insert("full_name", &user.full_name());
        context.insert("code", &code);
        context.insert("link", &link);

        MailerService::new(app.clone())
            .subject(app.title("Email Verification"))
            .receivers(vec![MailBox::new(&user.full_name(), user.email.as_str())])
            .view("account-confirmation", context)
            .send_silently();
    }

    pub fn make_cache_date(user: UserCacheable) -> AppResult<UserCacheData> {
        Ok(user.into_cache_data())
    }
}
