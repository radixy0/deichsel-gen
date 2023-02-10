use rand::Rng;
use yew::prelude::*;
use yew::{function_component, html, Callback, Html, Properties};

fn test() -> String {
    let list = [
        "machen",
        "verfahren",
        "wirken",
        "handeln",
        "tun",
        "vorgehen",
        "veranstalten",
        "operieren",
        "anfangen",
        "walken",
        "aufführen",
        "schmecken",
        "herausschneiden",
        "zuschlagen",
        "zusammennähen",
        "eine Schwierigkeit überwinden",
        "Arbeit leisten",
        "tätig sein",
        "sich verhalten",
        "zur Tat schreiten",
        "deichseln",
        "tun",
        "machen",
        "erledigen",
        "machen",
        "verfahren",
        "spielen",
        "auftreten",
        "wirken",
        "tun",
        "schaffen",
        "darstellen",
        "betreiben",
        "vorgehen",
        "herstellen",
        "treiben",
        "vorstellen",
        "agieren",
        "erledigen",
        "operieren",
        "verkörpern",
        "basteln",
        "mimen",
        "arbeiten",
        "sich befassen",
        "sich beschäftigen",
        "sich betätigen",
        "sich regen",
        "sich rühren",
        "sich widmen",
        "sich verhalten",
        "etwas unternehmen",
        "figurieren",
        "tätig sein",
        "bilden",
        "machen",
        "tun",
        "durchführen",
        "realisieren",
        "verwirklichen",
        "begehen",
        "agieren",
        "bereiten",
        "verrichten",
        "absolvieren",
        "erledigen",
        "vollziehen",
        "vollenden",
        "durchziehen",
        "verüben",
        "bewerkstelligen",
        "anrichten",
        "besorgen",
        "vollbringen",
        "abwickeln",
        "verschulden",
        "anstellen",
        "zubereiten",
        "vollführen",
        "unternehmen",
        "gute Arbeit leisten",
        "bereitmachen",
        "ausfressen",
        "abmachen",
    ]
    .to_owned();
    let rand = rand::thread_rng().gen_range(0..list.len());
    return String::from(list[rand]);
}

#[function_component(App)]
pub fn app() -> Html {
    let a = use_state(String::default);
    let b = (*a).clone();
    let onclick = Callback::from(move |_| {
        a.set(String::from(test()));
    });

    html! {
        <main>
            <div class="flexCol">
            <button {onclick} class="button123">{"Click me!"}</button>
            <span class="subtitle">{b}</span>
            </div>
        </main>
    }
}
