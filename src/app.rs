use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[cfg(feature = "ssr")]
use leptos_axum::redirect;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/page2" view=|cx| view! { cx, <Page2/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let go_page2 = create_server_action::<GoPage2>(cx);

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <ActionForm action=go_page2>
            <input type="submit" value="Go to Page2"/>
        </ActionForm>
    }
}

/// Renders page2 of your application.
#[component]
fn Page2(cx: Scope) -> impl IntoView {
    let go_home = create_server_action::<GoHome>(cx);

    view! { cx,
        <h1>"Welcome to Page2"</h1>
        <ActionForm action=go_home>
            <input type="submit" value="Go to /"/>
        </ActionForm>
    }
}

#[server(GoHome, "/api")]
pub async fn go_home(cx: Scope) -> Result<(), ServerFnError> {
    redirect(cx, "/");
    Ok(())
}

#[server(GoPage2, "/api")]
pub async fn go_page2(cx: Scope) -> Result<(), ServerFnError> {
    redirect(cx, "/page2");
    Ok(())
}
