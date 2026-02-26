use leptos::*;

#[component]
pub fn VideoShowcase() -> impl IntoView {
    view! {
        <section class="video-section">
            <div class="container">
                <h2 class="section-title">"Git for databases in Action"</h2>
                <div class="video-container">
                    <video
                        autoplay=true
                        muted=true
                        loop=true
                        playsinline=true
                        controls=true
                        class="showcase-video"
                    >
                        <source src="/public/assets/gfs-showcase.mp4" type="video/mp4"/>
                        "Your browser does not support the video tag."
                    </video>
                </div>
            </div>
        </section>
    }
}
