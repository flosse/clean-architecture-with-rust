use cawr_infrastructure::{cli, logger};

pub fn main() {
    logger::init_default_logger();
    cli::run();
}
