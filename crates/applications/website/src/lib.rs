use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
mod pages;

use components::{Footer, Header};
use pages::{Docs, Home, NotFound};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Header/>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/docs" view=Docs/>
                    <Route path="/docs/*page" view=Docs/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
