use std::time::Duration;

use actix_web::rt::{spawn, time};
use log::{error, info};
use redis::Commands;

use core::app_state::AppState;
use core::models::mail::{
    MailFailureResponse, MailQueueablePayload, MailSaved, MailSuccessResponse,
};
use core::services::mail_service::MailService;

use crate::redis_error_handler::handle_redis_error;

pub(crate) fn handle_awaiting_queue(app: &AppState, thread_name: String) {
    let app = app.clone();
    spawn(async move {
        let mut interval = time::interval(Duration::from_millis(200));
        loop {
            let mut redis = app.redis.clone();
            let popped = redis.rpop::<&str, String>(&*app.redis_queues.awaiting, None);
            match popped {
                Ok(item) => {
                    let payload_res = serde_json::from_str::<MailQueueablePayload>(item.as_str());
                    match payload_res {
                        Ok(mut payload) => {
                            let subject = payload.subject.clone();

                            info!("[{}] handling awaiting: {}", thread_name.clone(), subject);

                            payload.from =
                                Option::from(payload.from.unwrap_or_else(|| app.mail_from.clone()));

                            match MailService.create(app.database(), payload) {
                                Ok(mail) => {
                                    let _ = MailService.push_to_processing_queue(&app, mail);
                                }
                                Err(err) => {
                                    error!(
                                        "[{}][handle_awaiting_queue] db error: {:?}",
                                        thread_name.clone(),
                                        err
                                    );
                                }
                            };
                        }
                        Err(err) => {
                            error!(
                                "[{}] error decoding awaiting: {:?}",
                                thread_name.clone(),
                                err
                            );
                        }
                    };
                }
                Err(err) => {
                    handle_redis_error(err, thread_name.clone(), "handle_awaiting_queue");
                    interval.tick().await;
                }
            };
        }
    });
}

pub(crate) fn handle_processing_queue(app: &AppState, thread_name: String) {
    let app = app.clone();
    spawn(async move {
        let mut interval = time::interval(Duration::from_millis(200));
        loop {
            let mut redis = app.redis.clone();
            let popped = redis.rpop::<&str, String>(&*app.redis_queues.processing, None);
            match popped {
                Ok(item) => {
                    let payload_res = serde_json::from_str::<MailSaved>(item.as_str());
                    match payload_res {
                        Ok(saved) => {
                            let subject = saved.mail.subject.clone();
                            info!("[{}] processing: {}", thread_name.clone(), subject);
                            MailService.send(&app, thread_name.clone(), saved).await;
                        }
                        Err(err) => {
                            error!(
                                "[{}] error decoding awaiting: {:?}",
                                thread_name.clone(),
                                err
                            );
                        }
                    };
                }
                Err(err) => {
                    handle_redis_error(err, thread_name.clone(), "handle_processing_queue");
                    interval.tick().await;
                }
            };
        }
    });
}

pub(crate) fn handle_success_queue(app: &AppState, thread_name: String) {
    let app = app.clone();
    spawn(async move {
        let mut interval = time::interval(Duration::from_millis(200));
        loop {
            let mut redis = app.redis.clone();
            let popped = redis.rpop::<&str, String>(&*app.redis_queues.success, None);
            match popped {
                Ok(item) => {
                    let payload_res = serde_json::from_str::<MailSuccessResponse>(item.as_str());
                    match payload_res {
                        Ok(response) => {
                            info!(
                                "[{}] marking as success: {}",
                                thread_name.clone(),
                                response.saved_mail.mail.subject.clone()
                            );

                            let _ = MailService.mark_as_success(app.database(), response);
                        }
                        Err(err) => {
                            error!(
                                "[{}] error decoding success: {:?}",
                                thread_name.clone(),
                                err
                            );
                        }
                    };
                }
                Err(err) => {
                    handle_redis_error(err, thread_name.clone(), "handle_success_queue");
                    interval.tick().await;
                }
            };
        }
    });
}

pub(crate) fn handle_failure_queue(app: &AppState, thread_name: String) {
    let app = app.clone();
    spawn(async move {
        let mut interval = time::interval(Duration::from_millis(200));
        loop {
            let mut redis = app.redis.clone();
            let popped = redis.rpop::<&str, String>(&*app.redis_queues.failure, None);
            match popped {
                Ok(item) => {
                    let payload_res = serde_json::from_str::<MailFailureResponse>(item.as_str());
                    match payload_res {
                        Ok(response) => {
                            let mut saved = response.saved_mail.clone();
                            info!(
                                "[{}] marking as failure: {}",
                                thread_name.clone(),
                                saved.mail.subject.clone()
                            );

                            let trials = saved.mail.trials + 1;
                            match trials < app.max_retrials {
                                true => {
                                    info!("retrying: {}", trials);

                                    let mail =
                                        MailService.mark_as_retrying(app.database(), response);

                                    if let Ok(mail) = mail {
                                        saved.mail = mail;
                                        let _ = MailService.push_to_processing_queue(&app, saved);
                                    }
                                }
                                false => {
                                    let _ = MailService.mark_as_failure(app.database(), response);
                                }
                            };
                        }
                        Err(err) => {
                            error!(
                                "[{}] error decoding failure: {:?}",
                                thread_name.clone(),
                                err
                            );
                        }
                    };
                }
                Err(err) => {
                    handle_redis_error(err, thread_name.clone(), "handle_failure_queue");
                    interval.tick().await;
                }
            };
        }
    });
}
