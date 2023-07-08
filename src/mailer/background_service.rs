use std::time::{Duration};

use actix_web::rt::{spawn, time};
use lettre::SmtpTransport;
use log::info;
use redis::{Client, Commands};
use crate::core::mail_service::MailService;

use crate::mailer::thread_namer::name_thread;

pub(crate) fn create_background_service(redis: Client, smtp: &SmtpTransport, sub_threads: i8) {
    let name = name_thread().unwrap();
    info!("preparing thread: {}", name.clone());

    for _i in 0..sub_threads {
        let name = name.clone();
        let mut redis = redis.clone();
        let smtp = smtp.clone();
        spawn(async move {
            let mut interval = time::interval(Duration::from_secs(2));
            loop {
                interval.tick().await;
                let data_result = redis.rpop::<&str, String>("queue:mailer:mails", None);
                if data_result.is_ok() {
                    let data = data_result.unwrap();
                    let mail_service_result = serde_json::from_str::<MailService>(data.as_str());
                    info!("[{}] Queue: {}", name, data.clone());

                    if mail_service_result.is_ok() {
                        let smtp = smtp.clone();
                        spawn(async move {
                            mail_service_result.unwrap().send(&smtp);
                        });
                    }
                }
            }
        });
    }
}
