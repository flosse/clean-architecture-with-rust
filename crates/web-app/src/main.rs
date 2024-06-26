use cawr_web_app_seed::start;

// ------ ------
//     Start
// ------ ------

fn main() {
    _ = console_log::init_with_level(log::Level::Debug); // TODO: use 'Info' in release mode
    console_error_panic_hook::set_once();
    let mount = gloo_utils::document()
        .get_element_by_id("app")
        .expect("#app node");
    log::info!("Start web application");
    start(mount);
}
