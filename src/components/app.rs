use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <h1>"Leptos SPA Demo"</h1>
        <p>"Welcome! This is a Rust + Leptos SPA running in WASM."</p>

    }
}

#[component]
pub fn Sorting() -> impl IntoView {
    view! {
        <h1>"Sorting Home Page"</h1>
        <a href="sorting/merge-sort">merge-sort</a>
    }
}

#[component]
pub fn PathFinding() -> impl IntoView {
    view! {
        <h1>"Path Finding Home Page"</h1>

    }
}


#[component]
pub fn About() -> impl IntoView {
    view! {
        <h1>"About"</h1>
        <p>"This is a multi-page Leptos web app, with routing and client-side state, built entirely in Rust!"</p>
    }
}
