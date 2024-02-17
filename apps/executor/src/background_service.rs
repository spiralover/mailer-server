use log::info;

use crate::queue_handler::{
    handle_awaiting_queue, handle_failure_queue, handle_processing_queue, handle_success_queue,
};
use cosmic::app_state::AppState;

pub(crate) fn create_background_service(app: &AppState, name: String, features_per_worker: i8) {
    info!("preparing thread: {}", name.clone());

    handle_awaiting_queue(app, name.clone());
    handle_success_queue(app, name.clone());
    handle_failure_queue(app, name.clone());

    for _loop_index in 0..features_per_worker {
        handle_processing_queue(&app.clone(), name.clone());
    }
}
