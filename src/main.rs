use simulator::components::time_signal::register_build_in_time_signals;
use simulator::app;
use yew::Renderer;
use log::info;

use  my_control_box::time_signal::register::register_local_time_signals;

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    info!("Register *Plugins* for Yew app");
    register_build_in_time_signals();
    register_local_time_signals();

    info!("Start Yew app");
    Renderer::<app::App>::new().render();
}