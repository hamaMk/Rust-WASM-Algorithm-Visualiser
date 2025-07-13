use leptos::prelude::*;
use leptos::children::Children;
use leptos_router::hooks::use_location;
use leptos_router::components::{A};

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