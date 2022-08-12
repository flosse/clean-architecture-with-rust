use infrastructure::{logger, web};

pub fn main() {
    logger::init_default_logger();
    web::run();
}
