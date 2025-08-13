use input_rs::yew::Input;
use yew::prelude::*;

use simulator::components::time_signal::registry::{register_time_signal, YewTimeSignal};
use simulator::components::time_signal::BoxedTimeSignalDialogProps;
use super::random::RandomSignal;
use control_box::signal::{DynTimeSignal, TimeSignal};

use log::info;

pub struct YewRandom {
    signal: RandomSignal<f64>,
}

impl YewTimeSignal for YewRandom {
    fn dialog(
        &self,
        signal: Box<dyn DynTimeSignal<f64>>,
        on_update: Callback<Box<dyn DynTimeSignal<f64>>>,
    ) -> Html {
        if self.signal().short_type_name() == signal.short_type_name() {
            html! { <RandomSignalDialog time_signal={signal} on_update={ on_update }/> }
        } else {
            html! {}
        }
    }

    fn name(&self) -> &'static str {
        self.signal.short_type_name()
    }

    fn render(&self) -> Html {
        html! { <> { self.signal.short_type_name() } </> }
    }

    fn signal(&self) -> Box<dyn control_box::signal::DynTimeSignal<f64> + Send + Sync> {
        Box::new(self.signal.clone())
    }
}
fn yew_random_factory() -> Box<dyn YewTimeSignal + Sync> {
    Box::new(YewRandom {
        signal: RandomSignal::<f64>::default(),
    })
}

pub fn register() {
    info!("Registering RandomSignal Yew component");
    register_time_signal(yew_random_factory);
}

#[function_component(RandomSignalDialog)]
pub fn random_signal_dialog(props: &BoxedTimeSignalDialogProps) -> Html {
    // Runtime reflection (downcasting to concrete type)
    // Variable assignment must be done outside the html! macro
    let updated = if let Some(step) = props
        .time_signal
        .clone()
        .as_any()
        .downcast_ref::<RandomSignal<f64>>()
    {
        step.clone()
    } else {
        RandomSignal::<f64>::default()
    };

    fn always_valid(_s: String) -> bool {
        true
    }

    let minimum_ref = use_node_ref();
    let minimum_handle = use_state(|| updated.minimum.to_string());
    let minimum_valid_handle = use_state(|| true);

    let maximum_ref = use_node_ref();
    let maximum_handle = use_state(|| updated.maximum.to_string());
    let maximum_valid_handle = use_state(|| true);


    let updated = RandomSignal::<f64> {
        minimum: (*minimum_handle).parse::<f64>().unwrap_or_default(),
        maximum: (*maximum_handle).parse::<f64>().unwrap_or_default(),
    };

    props.on_update.emit(Box::new(updated));

    html! {
        <div>
       <form  class="flex flex-row">
            <div class="flex flex-col w-64">
                <label class="block text-sm mb-2 form-field w-64 text-gray-300 dark:text-gray-700
                " for="impulse_function_label"> { "Signal Type" } </label>
                <div id="impulse_function_label" class=" text-lg font-bold w-64"> { "Impulse Function"} </div>
            </div>
            <Input
                r#type="number"
                name="minimum"
                r#ref={minimum_ref}
                handle={minimum_handle}
                valid_handle={minimum_valid_handle}
                validate_function={always_valid}

                label="Minimumn"
                required={true}
                error_message="Must be a number"
                class="form-field w-64"
                label_class="block text-sm mb-2 text-gray-300 dark:text-gray-700"
                input_class="w-full p-2 border border-gray-400 dark:border-gray-600 rounded "
                error_class="text-red-800"
            />
            <Input
                r#type="number"
                name="maximum"
                r#ref={maximum_ref}
                handle={maximum_handle}
                valid_handle={maximum_valid_handle}
                validate_function={always_valid}

                label="Maximum"
                required={true}
                error_message="Must be a number"
                class="form-field w-64"
                label_class="block text-sm mb-2 text-gray-300 dark:text-gray-700"
                input_class="w-full p-2 border border-gray-400 dark:border-gray-600 rounded "
                error_class="error-text"
            />

        </form>
        </div>
    }
}
