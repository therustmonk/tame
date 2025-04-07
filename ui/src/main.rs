use crb::agent::Standalone;
use tame_web::WebApp;

fn main() {
    console_error_panic_hook::set_once();
    let config = wasm_logger::Config::new(log::Level::Info);
    wasm_logger::init(config);
    WebApp::new().spawn();
}
