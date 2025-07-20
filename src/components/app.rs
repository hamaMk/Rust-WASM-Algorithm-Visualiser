use leptos::prelude::*;
use crate::components::common::AlgorithmCard;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <h1>"Algorithm Visualiser"</h1>
        <p class="text-muted">"Welcome! Checkout some Algorithm visualisations."</p>

        <div class="container mt-5">
            <div class="row">
                <div class="col">
                    <AlgorithmCard 
                        href="/sorting".to_string()
                        title="Sorting".to_string()
                    />
                </div>
                
                <div class="col">
                    <AlgorithmCard 
                        href="/path-finding".to_string()
                        title="Path Finding".to_string()
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Sorting() -> impl IntoView {
    view! {
        <h1>"Sorting Home Page"</h1>
        <a href="sorting/merge-sort">Merge-Sort</a>
    }
}

#[component]
pub fn PathFinding() -> impl IntoView {
    view! {
        <h1>"Path Finding Home Page"</h1>
        <a href="path-finding/dijkstra">Dijkstra</a>
    }
}


#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="container">
            <h1 class="display-4 mb-4">"About Algorithm Visualiser"</h1>
            
            <section class="mb-5">
                <h2 class="h3 border-bottom pb-2">"Project Overview"</h2>
                <p class="lead">
                    "A web-based algorithm visualiser built with "
                    <a href="https://leptos.dev/" target="_blank">"Leptos"</a>
                    " and Rust. This project animates popular algorithms step-by-step to help users understand how they work internally."
                </p>
                <p>
                    "This educational tool is designed to make complex algorithms more accessible through visual representation. 
                    By seeing algorithms in action, users can better understand the underlying principles and mechanics."
                </p>
            </section>
            
            <section class="mb-5">
                <h2 class="h3 border-bottom pb-2">"Features"</h2>
                <ul class="list-group list-group-flush">
                    <li class="list-group-item">"✅ Visual step-by-step animation of algorithms"</li>
                    <li class="list-group-item">"⚡ Smooth transitions powered by reactive signals"</li>
                    <li class="list-group-item">"🧠 Built using modern Rust + WASM (WebAssembly)"</li>
                    <li class="list-group-item">"🎨 CSS styling via Bootstrap"</li>
                    <li class="list-group-item">"🔍 Logs internal algorithm steps and UI updates"</li>
                </ul>
                <p class="mt-3">
                    <strong>"Currently supports: "</strong>
                    "Merge Sort, with more algorithms like Bubble Sort, Dijkstra's, and others coming soon!"
                </p>
            </section>
            
            <section class="mb-5">
                <h2 class="h3 border-bottom pb-2">"Technical Implementation"</h2>
                
                <div class="card mb-3">
                    <div class="card-header bg-light">
                        <h3 class="h5 mb-0">"🦀 Why Rust?"</h3>
                    </div>
                    <div class="card-body">
                        <p>"Rust is renowned for its blazing-fast performance, memory safety, and zero-cost abstractions. In this project, Rust ensures:"</p>
                        <ul>
                            <li>"Fast computation of algorithm steps"</li>
                            <li>"Safe concurrency with no data races"</li>
                            <li>"Minimal runtime overhead"</li>
                        </ul>
                    </div>
                </div>
                
                <div class="card">
                    <div class="card-header bg-light">
                        <h3 class="h5 mb-0">"🌐 Why WebAssembly?"</h3>
                    </div>
                    <div class="card-body">
                        <p>"By compiling to WebAssembly, this visualiser:"</p>
                        <ul>
                            <li>"🚀 Runs at near-native speed directly in the browser"</li>
                            <li>"🧩 Offers better performance than equivalent JavaScript visualisers"</li>
                            <li>"🔒 Operates in a secure, sandboxed environment"</li>
                        </ul>
                        <p>"Combined, Rust + WASM make this tool lightweight, efficient, and perfectly suited for performance-intensive visualisations."</p>
                    </div>
                </div>
            </section>
            
            <section class="mb-5">
                <h2 class="h3 border-bottom pb-2">"Author"</h2>
                <p class="lead">
                    "Hamandishe Mkandabvute"
                </p>
                <p>
                    <a href="https://hamandishe.com" target="_blank" class="btn btn-outline-primary">
                        "Visit Author's Website"
                    </a>
                </p>
            </section>
            
            <section class="mb-5">
                <h2 class="h3 border-bottom pb-2">"Get Involved"</h2>
                <p>
                    "This project is open-source and contributions are welcome! Whether you want to add new algorithms, 
                    improve visualizations, or enhance the user interface, your input is valuable."
                </p>
                <p>
                    <a href="https://github.com/hamaMk/Rust-WASM-Algorithm-Visualiser" target="_blank" class="btn btn-outline-dark">
                        <i class="bi bi-github me-2"></i>"View on GitHub"
                    </a>
                </p>
            </section>
        </div>
    }
}
