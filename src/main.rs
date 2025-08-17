use cb_simulator_yew::register_build_in;
use cb_simulator_yew::app;
use yew::Renderer;
use log::info;

use  my_control_box::time_signal::register::register_local_time_signals;

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    info!("Register *Plugins* for Yew app");
    register_build_in();
    register_local_time_signals();

    info!("Start Yew app");
    Renderer::<app::App>::new().render();
}