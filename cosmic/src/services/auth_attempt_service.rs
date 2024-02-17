use diesel::SaveChangesDsl;

use crate::enums::app_message::AppMessage;
use crate::helpers::db::DatabaseConnectionHelper;
use crate::helpers::time::current_timestamp;
use crate::helpers::DBPool;
use crate::models::auth_attempt::{AuthAttempt, AuthAttemptStatus, CreateDto};
use crate::repositories::auth_attempt_repository::AuthAttemptRepository;
use crate::repositories::user_repository::UserRepository;
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;

pub struct AuthAttemptService;

impl AuthAttemptService {
    pub fn create(
        &mut self,
        pool: &DBPool,
        status: AuthAttemptStatus,
        data: CreateDto,
    ) -> AppResult<AuthAttempt> {
        AuthAttemptRepository.deactivate_all_active_for_user(pool, data.email.clone())?;
        AuthAttemptRepository.create(pool, status, data)
    }

    pub fn verify_code(&mut self, pool: &DBPool, code: String) -> AppResult<AuthAttempt> {
        let auth_attempt =
            AuthAttemptRepository.find_pending_verification_by_code(pool, code.to_owned())?;

        let invalid_code_error = Err(AppMessage::WarningMessageStr(
            "Invalid verification code, please try login again",
        ));

        if auth_attempt.user_id.is_none() {
            return invalid_code_error;
        }

        UserRepository.find_by_id(pool, auth_attempt.user_id.unwrap())?;

        let verification_code = auth_attempt.verification_code.clone().unwrap();

        // Check attempt trials
        if auth_attempt.verification_code_trials == 3 {
            let result = AuthAttemptService.change_code_status(
                pool,
                code,
                AuthAttemptStatus::InvalidatedToken,
            );

            if result.is_err() {
                return Err(AppMessage::WarningMessageStr(
                    "Something went wrong, try again",
                ));
            }

            return Err(AppMessage::WarningMessageStr(
                "Your verification code has been used up, try login again",
            ));
        }

        if verification_code != code {
            let _ =
                AuthAttemptService.increment_verification_code_trial(pool, verification_code)?;

            return Err(AppMessage::WarningMessageStr("Invalid verification code"));
        }

        let _x = AuthAttemptService.change_code_status(pool, code, AuthAttemptStatus::LoggedIn);

        Ok(auth_attempt)
    }

    pub fn increment_verification_code_trial(
        &mut self,
        pool: &DBPool,
        code: String,
    ) -> AppResult<AuthAttempt> {
        let mut auth_attempt = AuthAttemptRepository.find_by_code(pool, code)?;
        auth_attempt.verification_code_trials += 1;
        auth_attempt.updated_at = current_timestamp();
        auth_attempt
            .save_changes::<AuthAttempt>(&mut pool.conn())
            .into_app_result()
    }

    pub fn change_code_status(
        &mut self,
        pool: &DBPool,
        code: String,
        status: AuthAttemptStatus,
    ) -> AppResult<AuthAttempt> {
        let mut auth_attempt = AuthAttemptRepository.find_by_code(pool, code)?;
        auth_attempt.status = status.to_string();
        auth_attempt.updated_at = current_timestamp();
        auth_attempt
            .save_changes::<AuthAttempt>(&mut pool.conn())
            .into_app_result()
    }
}
