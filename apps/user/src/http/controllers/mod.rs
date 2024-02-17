use cosmic::http::controllers::application_controller::application_controller;
use cosmic::http::controllers::auth_controller::auth_controller;
use cosmic::http::controllers::main_controller_guest::main_controller_guest;
use cosmic::http::controllers::misc_controller::misc_controller;
use cosmic::http::controllers::profile_controller::profile_controller;
use cosmic::http::controllers::setting_controller::setting_controller;
use cosmic::http::kernel::{Controller, Route};
use cosmic::http::middlewares::auth_middleware::AuthMiddleware;

use crate::http::controllers::announcement_controller::announcement_controller;
use crate::http::controllers::notification_controller::notification_controller;
use crate::http::controllers::permission_controller::permission_controller;
use crate::http::controllers::role_controller::role_controller;
use crate::http::controllers::system_controller::system_controller;
use crate::http::controllers::ui_menu_controller::ui_menu_controller;
use crate::http::controllers::ui_menu_item_controller::ui_menu_item_controller;
use crate::http::controllers::user_controller::user_controller;

mod announcement_controller;
mod notification_controller;
mod permission_controller;
mod role_controller;
mod system_controller;
mod ui_menu_controller;
mod ui_menu_item_controller;
mod user_controller;

pub fn routes() -> Vec<Route<AuthMiddleware>> {
    let routes = vec![
        Route {
            auth: None,
            prefix: String::from(""),
            controllers: vec![Controller {
                path: String::from(""),
                handler: main_controller_guest,
            }],
        },
        Route {
            auth: None,
            prefix: String::from("/system"),
            controllers: vec![Controller {
                path: String::from(""),
                handler: system_controller,
            }],
        },
        Route {
            auth: None,
            prefix: String::from("/api/v1"),
            controllers: vec![Controller {
                path: String::from("/auth"),
                handler: auth_controller,
            }],
        },
        Route {
            auth: Some(AuthMiddleware::new(vec![])),
            prefix: String::from("/api/v1"),
            controllers: vec![
                Controller {
                    path: String::from("/profile"),
                    handler: profile_controller,
                },
                Controller {
                    path: String::from("/settings"),
                    handler: setting_controller,
                },
                Controller {
                    path: String::from("/roles"),
                    handler: role_controller,
                },
                Controller {
                    path: String::from("/permissions"),
                    handler: permission_controller,
                },
                Controller {
                    path: String::from("/misc"),
                    handler: misc_controller,
                },
                Controller {
                    path: String::from("/announcements"),
                    handler: announcement_controller,
                },
                Controller {
                    path: String::from("/ui-menus"),
                    handler: ui_menu_controller,
                },
                Controller {
                    path: String::from("/ui-menu-items"),
                    handler: ui_menu_item_controller,
                },
                Controller {
                    path: String::from("/users"),
                    handler: user_controller,
                },
                Controller {
                    path: String::from("/applications"),
                    handler: application_controller,
                },
                Controller {
                    path: String::from("/notifications"),
                    handler: notification_controller,
                },
            ],
        },
    ];

    routes
}
