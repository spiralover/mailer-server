use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

use actix_web::web::{Data, ServiceConfig};
use actix_web::{get, HttpRequest, HttpResponse};
use rand::Rng;
use serde::Deserialize;
use strum::VariantNames;
use uuid::Uuid;

use core::app_state::AppState;
use core::helpers::responder::json_success_message;
use core::models::role::RoleCreateForm;
use core::models::user::{UserRegisterForm, UserStatus};
use core::models::user_ui_menu_item::MenuItemCreateDto;
use core::models::DBPool;
use core::permissions::Permissions;
use core::repositories::permission_repository::PermissionRepository;
use core::repositories::role_repository::RoleRepository;
use core::repositories::ui_menu_item_repository::UiMenuItemRepository;
use core::results::AppResult;
use core::roles::Roles;
use core::services::permission_service::PermissionService;
use core::services::role_service::RoleService;
use core::services::user_service::UserService;
use core::services::user_ui_menu_item_service::UserUiMenuItemService;

pub fn system_controller(cfg: &mut ServiceConfig) {
    cfg.service(database_seed);
    cfg.service(docker_test);
}

#[get("docker-health-check")]
async fn docker_test() -> HttpResponse {
    json_success_message("received")
}

#[get("database-seed")]
async fn database_seed(req: HttpRequest) -> HttpResponse {
    let super_admin_user_id = Uuid::from_str("be6ee736-ed4d-43c9-9c91-bfd0318b875e").unwrap();
    let admin_user_id = Uuid::from_str("3b9fcf79-188c-489c-97e9-d9b57b29109b").unwrap();
    let ahmard_user_id = Uuid::from_str("430167fd-0b57-46e0-a184-6fe92b9658ea").unwrap();
    let ahmardiy_user_id = Uuid::from_str("23d10910-5bd2-4cec-b979-9bd7f21cc6d1").unwrap();

    let app = req.app_data::<Data<AppState>>().unwrap().get_ref();
    let db_pool = app.get_db_pool();

    // Create Roles
    for role_name in Roles::VARIANTS {
        let _role = RoleService
            .create(
                db_pool,
                super_admin_user_id,
                RoleCreateForm {
                    name: role_name.to_string(),
                    guard: String::from("api"),
                },
            )
            .unwrap();
    }

    // Seed Permission
    log::info!("seeding permissions...");
    let mut permissions = vec![];
    for permission in Permissions::VARIANTS {
        let perm = PermissionService
            .create(
                db_pool,
                super_admin_user_id,
                permission.to_string(),
                String::from("api"),
            )
            .unwrap();

        permissions.push(perm);
    }

    // SEED USERS FROM users.json
    log::info!("seeding users...");
    seed_users(app).await;

    // USER UI MENU ITEM
    log::info!("assigning menu item to users...");
    assign_menu_items_to_user(
        db_pool,
        vec![super_admin_user_id, admin_user_id, ahmard_user_id],
    );

    // USER ROLE
    let super_admin_role = RoleRepository
        .find_by_name(db_pool, Roles::SuperAdmin.to_string())
        .unwrap();

    let admin_role = RoleRepository
        .find_by_name(db_pool, Roles::Admin.to_string())
        .unwrap();

    let user_role = RoleRepository
        .find_by_name(db_pool, Roles::User.to_string())
        .unwrap();

    // ASSIGN BASIC PERMISSIONS TO ROLE
    let all_roles = vec![super_admin_role.role_id, user_role.role_id];

    let _ = give_basic_permissions_to_roles(db_pool, super_admin_user_id, all_roles);

    // Assign Permissions To Super Admin Role
    log::info!("binding roles with permissions...");
    for permission in permissions {
        RoleService
            .add_permission(
                db_pool,
                super_admin_user_id,
                super_admin_role.role_id,
                permission.permission_id,
            )
            .expect("Failed to add permission to role");
    }

    // Assign roles to users
    log::info!("assigning roles to users...");
    RoleService
        .assign_role_to_user(
            db_pool,
            super_admin_user_id,
            super_admin_role.role_id,
            super_admin_user_id,
        )
        .expect("Failed to add role");

    RoleService
        .assign_role_to_user(db_pool, admin_user_id, admin_role.role_id, admin_user_id)
        .expect("Failed to add role");

    RoleService
        .assign_role_to_user(
            db_pool,
            super_admin_user_id,
            user_role.role_id,
            ahmard_user_id,
        )
        .expect("Failed to add role");

    RoleService
        .assign_role_to_user(
            db_pool,
            super_admin_user_id,
            user_role.role_id,
            ahmardiy_user_id,
        )
        .expect("Failed to add role");

    json_success_message("database seeded")
}

#[allow(dead_code)]
fn get_random_uuid(ids: &Vec<Uuid>) -> &Uuid {
    let length = ids.len() as i64;
    let upper_bound = (length - 1) as usize;
    ids.get(rand::thread_rng().gen_range(0..upper_bound))
        .unwrap()
}

async fn seed_users(app: &AppState) {
    let filename = "users.json";
    if !Path::new(filename).exists() {
        return; // skip since file does not exits
    }

    let db_pool = app.get_db_pool();
    let mut file = File::open(filename).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let temp_users = serde_json::from_str::<Vec<TempUser>>(data.as_str()).unwrap();
    let mut users = vec![];

    let default_role_id = RoleRepository.get_default_role_id(db_pool);

    for user in temp_users {
        let email = user.email.clone();
        let split_email: Vec<&str> = email.split('@').collect();

        users.push(
            UserService
                .create(
                    app,
                    default_role_id,
                    UserRegisterForm {
                        email: user.email,
                        username: user.username,
                        first_name: user.first_name,
                        last_name: user.last_name,
                        created_by: None,
                        password: format!("#{}.{}", split_email.first().unwrap(), 576),
                    },
                    Some(UserStatus::Active),
                )
                .await,
        )
    }
}

fn give_basic_permissions_to_roles(
    db_pool: &DBPool,
    created_by: Uuid,
    roles: Vec<Uuid>,
) -> AppResult<()> {
    let default_permission_names = vec![
        Permissions::UserMyProfileUpdate,
        Permissions::UserMyProfileUploadPassport,
        Permissions::UserMyProfileListAuthAttempt,
    ];

    let permissions = PermissionRepository
        .get_by_names(db_pool, default_permission_names)
        .map(|perms| {
            let perms: Vec<Uuid> = perms.iter().map(|p| p.permission_id).collect();
            perms
        })?;

    for role in &roles {
        for permission in &permissions {
            let _x = RoleService.add_permission(db_pool, created_by, *role, *permission);
        }
    }

    Ok(())
}

fn assign_menu_items_to_user(db_pool: &DBPool, ids: Vec<Uuid>) {
    let menu_items = UiMenuItemRepository.list(db_pool).unwrap();
    for id in ids {
        for menu_item in &menu_items {
            let _ = UserUiMenuItemService.create(
                db_pool,
                id,
                MenuItemCreateDto {
                    user_id: id,
                    menu_item_id: menu_item.ui_menu_item_id,
                },
            );
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct TempUser {
    username: String,
    first_name: Option<String>,
    last_name: Option<String>,
    email: String,
}
