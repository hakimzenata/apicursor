use log::{error, info};

pub fn init_logger() {
    env_logger::init();
}

pub fn log_info(message: &str) {
    info!("{}", message);
}

pub fn log_error(message: &str) {
    error!("{}", message);
}
