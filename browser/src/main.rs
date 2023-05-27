use crate::app::App;
use leptos::*;

mod app;
mod auth;
mod routes;

fn main() {
    mount_to_body(|cx| view! { cx,  <App />})
}
