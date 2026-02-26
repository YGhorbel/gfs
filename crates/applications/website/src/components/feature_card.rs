use leptos::*;

#[component]
pub fn FeatureCard(
    #[prop(into)] icon: String,
    #[prop(into)] title: String,
    #[prop(into)] description: String,
) -> impl IntoView {
    view! {
        <div class="feature-card">
            <div class="feature-icon">{icon}</div>
            <h3 class="feature-title">{title}</h3>
            <p class="feature-description">{description}</p>
        </div>
    }
}
