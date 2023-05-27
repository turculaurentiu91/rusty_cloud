use leptos::*;

fn main() {
    mount_to_body(|cx| view! { cx,  <App />})
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let async_data = create_resource(cx, count, |_| async move {
        reqwest_wasm::get("http://localhost:3000/api")
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    });

    view! { cx,
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me: "
            {move || count.get()}
        </button>
        <p>
            {move || match async_data.read(cx) {
                None => view! { cx, "Loading..." }.into_view(cx),
                Some(data) => view! { cx, <span>{data}</span> }.into_view(cx),
            }}
        </p>
    }
}
