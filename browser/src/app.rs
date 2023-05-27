use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let me = create_resource(cx, || (), |_| async move { crate::auth::me().await });

    view! { cx,
        <div>
        { move || match me.read(cx) {
            None => view! { cx, <p>"Loading..."</p> }.into_view(cx),
            Some(Some(data)) => view! { cx,
                <div>
                    <p>{ data.email }</p>
                    <p>{ data.name }</p>
                </div>
            }.into_view(cx),
            Some(None) => view! { cx, <p>"Not logged in"</p> }.into_view(cx),
        }}
        </div>
    }
}
