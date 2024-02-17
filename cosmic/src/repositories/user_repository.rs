use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl,
    SaveChangesDsl,
};
use uuid::Uuid;

use crate::helpers::db::{DatabaseConnectionHelper, OptionalResult};
use crate::helpers::db_pagination::{PageData, Paginate};
use crate::helpers::form::get_nullable_uuid;
use crate::helpers::http::QueryParams;
use crate::helpers::string::password_hash;
use crate::helpers::time::current_timestamp;
use crate::helpers::DBPool;
use crate::models::user::{
    TempPasswordStatus, User, UserCacheable, UserFullName, UserMinimalData, UserRegisterForm,
    UserStatus, UserUpdateForm,
};
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;
use crate::schema::{user_roles, users};

pub struct UserRepository;

impl UserRepository {
    pub fn all(&mut self, pool: &DBPool) -> AppResult<Vec<UserMinimalData>> {
        users::table
            .select((
                users::user_id,
                users::username,
                users::first_name,
                users::last_name,
                users::email,
            ))
            .filter(users::deleted_at.is_null())
            .get_results::<UserMinimalData>(&mut pool.conn())
            .into_app_result()
    }

    pub fn list(&mut self, pool: &DBPool, query_params: QueryParams) -> AppResult<PageData<User>> {
        let search_format = format!("%{}%", query_params.get_search_query());
        users::table
            .filter(
                users::first_name
                    .ilike(search_format.clone())
                    .or(users::last_name.ilike(search_format.clone()))
                    .or(users::email.ilike(search_format.clone()))
                    .or(users::email.ilike(search_format)),
            )
            .filter(users::deleted_at.is_null())
            .order_by(users::created_at.desc())
            .paginate(query_params.get_page())
            .per_page(query_params.get_per_page())
            .load_and_count_pages::<User>(&mut pool.conn())
            .into_app_result()
    }

    pub fn list_by_role(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        query_params: QueryParams,
    ) -> AppResult<PageData<User>> {
        let sq_role_users = user_roles::table
            .select(user_roles::user_id)
            .filter(user_roles::role_id.eq(id))
            .filter(user_roles::deleted_at.is_null());

        let search_format = format!("%{}%", query_params.get_search_query());
        users::table
            .filter(
                users::first_name
                    .ilike(search_format.clone())
                    .or(users::last_name.ilike(search_format.clone()))
                    .or(users::email.ilike(search_format.clone()))
                    .or(users::email.ilike(search_format)),
            )
            .filter(users::user_id.eq_any(sq_role_users))
            .filter(users::deleted_at.is_null())
            .order_by(users::created_at.desc())
            .paginate(query_params.get_page())
            .per_page(query_params.get_per_page())
            .load_and_count_pages::<User>(&mut pool.conn())
            .into_app_result()
    }

    pub fn create(
        &mut self,
        pool: &DBPool,
        data: UserRegisterForm,
        verification_code: String,
        verification_token: String,
        status: Option<UserStatus>,
    ) -> AppResult<User> {
        let model = User {
            user_id: Uuid::new_v4(),
            created_by: get_nullable_uuid(data.created_by),
            username: data.username,
            first_name: None,
            last_name: None,
            email: data.email,
            profile_picture: None,
            verification_code: Some(verification_code),
            verification_token: Some(verification_token),
            verified_at: None,
            is_verified: false,
            is_password_locked: false,
            has_started_password_reset: false,
            temp_password_status: TempPasswordStatus::UnUsed.to_string(),
            status: status.unwrap_or(UserStatus::Pending).to_string(),
            password: password_hash(data.password),
            created_at: current_timestamp(),
            updated_at: current_timestamp(),
            deleted_at: None,
        };

        let user = diesel::insert_into(users::dsl::users)
            .values(model)
            .get_result::<User>(&mut pool.conn())
            .unwrap();

        Ok(user)
    }

    pub fn update(&mut self, pool: &DBPool, id: Uuid, form: UserUpdateForm) -> AppResult<User> {
        let mut user = self.find_by_id(pool, id)?;
        user.first_name = form.first_name;
        user.last_name = form.last_name;
        user.email = form.email;
        user.save_changes::<User>(&mut pool.conn())
            .into_app_result()
    }

    pub fn get_full_name(&mut self, pool: &DBPool, id: Uuid) -> AppResult<UserFullName> {
        users::table
            .select((users::user_id, users::first_name, users::last_name))
            .filter(users::user_id.eq(id))
            .filter(users::deleted_at.is_null())
            .first::<UserFullName>(&mut pool.conn())
            .required("user")
    }

    pub fn get_basic_info(&mut self, pool: &DBPool, id: Uuid) -> AppResult<UserMinimalData> {
        users::table
            .select((
                users::user_id,
                users::username,
                users::first_name,
                users::last_name,
                users::email,
            ))
            .filter(users::user_id.eq(id))
            .filter(users::deleted_at.is_null())
            .first::<UserMinimalData>(&mut pool.conn())
            .required("user")
    }

    pub fn fetch_email(&mut self, pool: &DBPool, id: Uuid) -> AppResult<String> {
        users::table
            .select(users::email)
            .filter(users::user_id.eq(id))
            .filter(users::deleted_at.is_null())
            .first::<String>(&mut pool.conn())
            .required("user")
    }

    pub fn fetch_cacheable(&mut self, pool: &DBPool, id: Uuid) -> AppResult<UserCacheable> {
        users::table
            .select((users::user_id, users::username, users::email))
            .filter(users::user_id.eq(id))
            .filter(users::deleted_at.is_null())
            .first::<UserCacheable>(&mut pool.conn())
            .required("user")
    }

    pub fn find_by_id(&mut self, pool: &DBPool, id: Uuid) -> AppResult<User> {
        users::table
            .filter(users::user_id.eq(id))
            .filter(users::deleted_at.is_null())
            .first::<User>(&mut pool.conn())
            .required("user")
    }

    pub fn find_by_token(&mut self, pool: &DBPool, token: String) -> AppResult<User> {
        users::table
            .filter(users::verification_token.eq(token))
            .filter(users::deleted_at.is_null())
            .first::<User>(&mut pool.conn())
            .required("user")
    }

    pub fn exists_by_email(
        &mut self,
        pool: &DBPool,
        email_addr: String,
    ) -> AppResult<Option<String>> {
        users::table
            .select(users::email)
            .filter(users::email.eq(email_addr))
            .filter(users::deleted_at.is_null())
            .first::<String>(&mut pool.conn())
            .optional()
    }

    pub fn username_exists(&mut self, pool: &DBPool, username: String) -> AppResult<String> {
        users::table
            .select(users::username)
            .filter(users::username.eq(username))
            .filter(users::deleted_at.is_null())
            .first::<String>(&mut pool.conn())
            .into_app_result()
    }

    pub fn find_by_email(&mut self, pool: &DBPool, email_addr: String) -> AppResult<User> {
        users::table
            .filter(users::email.eq(email_addr))
            .filter(users::deleted_at.is_null())
            .first::<User>(&mut pool.conn())
            .required("user")
    }
}
