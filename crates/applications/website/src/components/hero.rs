use leptos::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="hero">
            <div class="container">
                <div class="hero-content">
                    <h1 class="hero-title">
                        <span class="highlight">"Git-like version control"</span>
                        " for your databases"
                    </h1>
                    <p class="hero-subtitle">
                        "Commit, branch, merge, and time travel through your database history with confidence."
                    </p>
                    <div class="hero-badges">
                        <span class="badge">"🚧 Under Active Development"</span>
                        <span class="badge">"✅ PostgreSQL 13-18"</span>
                        <span class="badge">"✅ MySQL 8.0-8.1"</span>
                    </div>
                </div>
            </div>
        </section>
    }
}
