use leptos::{ev::SubmitEvent, *};

mod auth;
fn main() {
    mount_to_body(|cx| view! { cx,  <App />})
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (email, set_email) = create_signal(cx, String::new());
    let (password, set_password) = create_signal(cx, String::new());

    let login_action = create_action(cx, |input: &(String, String)| {
        let (email, password) = input;
        auth::login(email.clone(), password.clone())
    });

    let on_submit = move |e: SubmitEvent| {
        e.prevent_default();

        let email = email.get();
        let password = password.get();

        login_action.dispatch((email, password));
    };

    let token = login_action.value();

    view! { cx,
        <div>
            <form on:submit=on_submit>
            <input type="email" value={email} on:input=move |ev| set_email(event_target_value(&ev))/>
                <input type="password" value={password} on:input=move |ev| {set_password(event_target_value(&ev))} />
                <button type="submit">"Login"</button>
            </form>
            <p>
            {move || {match token.get() {
                Some(Ok(token)) => format!("Token: {}", token),
                Some(Err(e)) => format!("Error: {}", e),
                None => "Not logged in".to_string(),
            }}}
            </p>
        </div>
    }
}
