use leptos::*;
use leptos_router::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="header">
            <div class="container">
                <nav class="nav">
                    <A href="/" class="logo">
                        <img src="/public/logo.svg" alt="GFS Logo" class="logo-img"/>
                    </A>
                    <div class="nav-links">
                        <a href="https://github.com/Guepard-Corp/gfs" target="_blank" class="nav-link">
                            "GitHub"
                        </a>
                        <A href="/docs" class="nav-link">"Docs"</A>
                        <a href="https://discord.gg/SEdZuJbc5V" target="_blank" class="nav-link-button">
                            "Community"
                        </a>
                    </div>
                </nav>
            </div>
        </header>
    }
}
