use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::*;
use crate::components::app::{About, Home, PathFinding, Sorting};
use crate::components::common::{NavLink, NotFound, Footer};
use crate::components::path_finding::dijkstra::Dijkstra;
use crate::components::sorting::merge_sort::MergeSort;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="d-flex flex-column min-vh-100">
                <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
                    <div class="container-fluid">
                        <a class="navbar-brand" href="/">
                            "Algorithm-Visualizer"
                        </a>
                        <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav" aria-controls="navbarNav" aria-expanded="false" aria-label="Toggle navigation">
                        <span class="navbar-toggler-icon"></span>
                        </button>
                        <div class="collapse navbar-collapse" id="navbarNav">
                            <ul class="navbar-nav navbar-nav me-auto mb-2 mb-lg-0">
                                <li class="nav-item">
                                    <NavLink href="/".to_string()>"Home"</NavLink>
                                </li>
                                <li class="nav-item">
                                    <NavLink href="/about".to_string()>"About"</NavLink>
                                </li>
                            </ul>
                        </div>
                    </div>
                </nav>
                <main class="flex-grow-1 mt-5">
                    <div class="container">
                        <Routes fallback=|| view! { <NotFound /> }>
                            <Route path=path!("/") view=Home />
                            <Route path=path!("/sorting") view=Sorting />
                            <Route path=path!("/sorting/merge-sort") view=MergeSort />
                            <Route path=path!("/path-finding") view=PathFinding />
                            <Route path=path!("/path-finding/dijkstra") view=Dijkstra />
                            <Route path=path!("/about") view=About />
                        </Routes>
                    </div>
                </main>
                <Footer />
            </div>
        </Router>
    }
}
