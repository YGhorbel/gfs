use leptos::*;
use crate::components::{CodeTabs, Faq, FeatureCard, Hero, VideoShowcase};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="home">
            <Hero/>
            <CodeTabs/>
            <VideoShowcase/>

            <section class="features-section">
                <div class="container">
                    <h2 class="section-title">"Why GFS?"</h2>
                    <div class="features-grid">
                        <FeatureCard
                            icon="💾"
                            title="Commit Database States"
                            description="Save snapshots of your database with meaningful messages, just like Git commits."
                        />
                        <FeatureCard
                            icon="🌿"
                            title="Branch & Merge"
                            description="Create isolated branches for experiments and features, merge them back when ready."
                        />
                        <FeatureCard
                            icon="⏰"
                            title="Time Travel"
                            description="Checkout any previous state of your database instantly. No more manual backups."
                        />
                        <FeatureCard
                            icon="🤝"
                            title="Team Collaboration"
                            description="Work on database changes with confidence, knowing you can always rollback."
                        />
                        <FeatureCard
                            icon="🐳"
                            title="Docker Powered"
                            description="Isolated database environments managed automatically with Docker containers."
                        />
                        <FeatureCard
                            icon="🔄"
                            title="Instant Rollback"
                            description="Made a mistake? Rollback to any previous commit in seconds, not hours."
                        />
                    </div>
                </div>
            </section>

            <section class="workflow-section">
                <div class="container">
                    <h2 class="section-title">"Simple Workflow"</h2>
                    <div class="workflow-steps">
                        <div class="workflow-step">
                            <div class="step-number">"1"</div>
                            <h3>"Initialize"</h3>
                            <pre><code>"gfs init --database-provider postgres --database-version 17"</code></pre>
                        </div>
                        <div class="workflow-step">
                            <div class="step-number">"2"</div>
                            <h3>"Make Changes"</h3>
                            <pre><code>"psql -h localhost -p 5432 -U postgres\n# Make your database changes..."</code></pre>
                        </div>
                        <div class="workflow-step">
                            <div class="step-number">"3"</div>
                            <h3>"Commit"</h3>
                            <pre><code>"gfs commit -m \"Add user table\""</code></pre>
                        </div>
                        <div class="workflow-step">
                            <div class="step-number">"4"</div>
                            <h3>"Time Travel"</h3>
                            <pre><code>"gfs log\ngfs checkout <commit_hash>"</code></pre>
                        </div>
                    </div>
                </div>
            </section>

            <section class="cta-section">
                <div class="container">
                    <div class="cta-box">
                        <h2>"Ready to get started?"</h2>
                        <p>"Join the community and help shape the future of database version control."</p>
                        <div class="cta-buttons">
                            <a href="/docs" class="btn-primary">"Read the Docs"</a>
                            <a href="https://github.com/Guepard-Corp/gfs" target="_blank" class="btn-secondary">"View on GitHub"</a>
                        </div>
                    </div>
                </div>
            </section>

            <Faq/>
        </div>
    }
}
