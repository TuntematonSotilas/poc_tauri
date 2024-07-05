
use yew::prelude::*;
use yew_router::prelude::*;
use yew_i18n::use_translation;

use crate::enums::route::Route;

#[function_component(SettingComp)]
pub fn setting() -> Html {

    let navigator = use_navigator().unwrap();
    let menuclick: Callback<MouseEvent> = Callback::from(move |_| navigator.push(&Route::Menu));

    //let i18n = use_translation();

    //let onclick: Callback<String> = Callback::from(move |_| {
    //    i18n.set_translation_language("FR");
    //});

    html! {
        <div class="container">
            <div class="row">
                <h1>{"Settings"}</h1>
            </div>
            <div class="row">
                <button onclick={menuclick}>
                    {"Exit"}    
                </button>
            </div>
            <div class="row">
                <button>{"FR"}</button>
            </div>
            
        </div>
    }
}