use leptos::prelude::*;
use leptos::children::Children;
use leptos_router::hooks::use_location;
use leptos_router::components::A;
use chrono::{Utc, Datelike};

#[component]
pub fn NavLink(href: String, children: Children) -> impl IntoView {
    let location = use_location();
    let href_owned = href.clone();
    let is_active = move || location.pathname.get() == href_owned;

    let class = move || {
        if is_active() {
            "nav-link active"
        } else {
            "nav-link"
        }
    };

    view! {
        <A href=href attr:class=class>
            {children()}
        </A>
    }
}

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="text-center">
            <h1 class="display-4">404</h1>
            <p class="lead">Page not found.</p>
            <a class="btn btn-primary" href="/">"Go Home"</a>
        </div>
    }
}

#[component]
pub fn Footer() -> impl IntoView {
    let year = Utc::now().year();

    view! {
        <div class="bg-dark p-5 text-center text-light d-flex flex-column">
            <span>"Made with ❤️🦀 Rust + WASM"</span>
            <span>{format!("Hamandishe Mkandabvute © {}", year)}</span>
        </div>
    }
}

#[component]
pub fn AlgorithmCard(href: String, title: String) -> impl IntoView {
    view! {
        <A href=href attr:class="text-decoration-none">
            <div class="card w-50">
                <div class="card-body">
                    <h5 class="card-title">{title}</h5>
                    <p class="card-text">
                        <span>""</span>
                    </p>
                    <span class="stretched-link"></span>
                </div>
            </div>
        </A>
    }
}

