use leptos::*;
use leptos_router::*;
use crate::components::CodeBlock;

#[component]
pub fn Docs() -> impl IntoView {
    let params = use_params_map();
    let page = move || params.with(|p| p.get("page").cloned().unwrap_or_else(|| "getting-started".to_string()));

    view! {
        <div class="docs-page">
            <div class="container">
                <div class="docs-layout">
                    <aside class="docs-sidebar">
                        <nav class="docs-nav">
                            <div class="nav-section">
                                <h3>"Getting Started"</h3>
                                <ul>
                                    <li><A href="/docs" class="nav-item">"Introduction"</A></li>
                                    <li><A href="/docs/installation" class="nav-item">"Installation"</A></li>
                                    <li><A href="/docs/quick-start" class="nav-item">"Quick Start"</A></li>
                                </ul>
                            </div>
                            <div class="nav-section">
                                <h3>"Commands"</h3>
                                <ul>
                                    <li><A href="/docs/commands/init" class="nav-item">"gfs init"</A></li>
                                    <li><A href="/docs/commands/status" class="nav-item">"gfs status"</A></li>
                                    <li><A href="/docs/commands/commit" class="nav-item">"gfs commit"</A></li>
                                    <li><A href="/docs/commands/log" class="nav-item">"gfs log"</A></li>
                                    <li><A href="/docs/commands/checkout" class="nav-item">"gfs checkout"</A></li>
                                    <li><A href="/docs/commands/providers" class="nav-item">"gfs providers"</A></li>
                                </ul>
                            </div>
                            <div class="nav-section">
                                <h3>"MCP Server"</h3>
                                <ul>
                                    <li><A href="/docs/mcp/overview" class="nav-item">"Overview"</A></li>
                                    <li><A href="/docs/mcp/claude-desktop" class="nav-item">"Claude Desktop"</A></li>
                                    <li><A href="/docs/mcp/http-mode" class="nav-item">"HTTP Mode"</A></li>
                                </ul>
                            </div>
                            <div class="nav-section">
                                <h3>"Advanced"</h3>
                                <ul>
                                    <li><A href="/docs/configuration" class="nav-item">"Configuration"</A></li>
                                    <li><A href="/docs/troubleshooting" class="nav-item">"Troubleshooting"</A></li>
                                    <li><A href="/docs/development" class="nav-item">"Development"</A></li>
                                </ul>
                            </div>
                        </nav>
                    </aside>
                    <article class="docs-content">
                        {move || match page().as_str() {
                            "" | "getting-started" => view! { <GettingStarted/> }.into_view(),
                            "installation" => view! { <Installation/> }.into_view(),
                            "quick-start" => view! { <QuickStart/> }.into_view(),
                            "commands/init" => view! { <CommandInit/> }.into_view(),
                            "commands/status" => view! { <CommandStatus/> }.into_view(),
                            "commands/commit" => view! { <CommandCommit/> }.into_view(),
                            "commands/log" => view! { <CommandLog/> }.into_view(),
                            "commands/checkout" => view! { <CommandCheckout/> }.into_view(),
                            "commands/providers" => view! { <CommandProviders/> }.into_view(),
                            _ => view! { <ComingSoon page=page()/> }.into_view(),
                        }}
                    </article>
                </div>
            </div>
        </div>
    }
}

#[component]
fn GettingStarted() -> impl IntoView {
    view! {
        <div>
            <h1>"Getting Started with GFS"</h1>
            <p class="lead">"GFS (Git For database Systems) brings Git-like version control to your databases."</p>

            <h2>"What is GFS?"</h2>
            <p>"GFS enables you to:"</p>
            <ul>
                <li><strong>"Commit"</strong>" database states with meaningful messages"</li>
                <li><strong>"Branch"</strong>" and "<strong>"merge"</strong>" database schemas and data"</li>
                <li><strong>"Time travel"</strong>" through your database history"</li>
                <li><strong>"Collaborate"</strong>" on database changes with confidence"</li>
                <li><strong>"Rollback"</strong>" to any previous state instantly"</li>
            </ul>

            <div class="alert warning">
                <strong>"⚠️ Important Notice"</strong>
                <p>"This project is under active development and not yet suitable for production use. Expect breaking changes, incomplete features, and evolving APIs."</p>
            </div>

            <h2>"Supported Databases"</h2>
            <ul>
                <li>"PostgreSQL (versions 13-18)"</li>
                <li>"MySQL (versions 8.0-8.1)"</li>
                <li>"More database providers coming soon"</li>
            </ul>

            <h2>"Requirements"</h2>
            <ul>
                <li>"Docker (latest version recommended)"</li>
                <li>"Bash/Zsh shell"</li>
                <li><code>"curl"</code>" for installation"</li>
                <li><code>"tar"</code>" for extracting releases"</li>
            </ul>

            <h2>"Next Steps"</h2>
            <p>"Continue to "<a href="/docs/installation">"Installation"</a>" to set up GFS on your system."</p>
        </div>
    }
}

#[component]
fn Installation() -> impl IntoView {
    view! {
        <div>
            <h1>"Installation"</h1>

            <h2>"Quick Install"</h2>
            <p>"The easiest way to install GFS is using our installation script:"</p>
            <CodeBlock code="curl -fsSL https://gfs.guepard.run/install | bash"/>

            <h2>"Build from Source"</h2>
            <p>"If you prefer to build from source:"</p>
            <CodeBlock code="git clone https://github.com/Guepard-Corp/gfs.git
cd gfs
cargo build --release"/>
            <p>"The binary will be available at "<code>"target/release/gfs"</code>"."</p>

            <h2>"Verify Installation"</h2>
            <p>"After installation, verify that GFS is working:"</p>
            <CodeBlock code="gfs --version"/>

            <h2>"Docker Setup"</h2>
            <p>"GFS requires Docker to be installed and running. Make sure Docker is available before using GFS:"</p>
            <ul>
                <li>"macOS/Windows: Install "<a href="https://www.docker.com/products/docker-desktop/" target="_blank">"Docker Desktop"</a></li>
                <li>"Linux: Install Docker Engine using your distribution's package manager"</li>
            </ul>

            <h2>"Next Steps"</h2>
            <p>"Continue to "<a href="/docs/quick-start">"Quick Start"</a>" to create your first GFS repository."</p>
        </div>
    }
}

#[component]
fn QuickStart() -> impl IntoView {
    view! {
        <div>
            <h1>"Quick Start"</h1>

            <h2>"1. Check Available Providers"</h2>
            <p>"First, see what database providers are available:"</p>
            <CodeBlock code="gfs providers"/>

            <h2>"2. Create a New Project"</h2>
            <CodeBlock code="mkdir my_project
cd my_project"/>

            <h2>"3. Initialize the Repository"</h2>
            <CodeBlock code="gfs init --database-provider postgres --database-version 17"/>
            <p>"This creates a "<code>".gfs"</code>" directory and starts a PostgreSQL database in a Docker container."</p>

            <h2>"4. Check Status"</h2>
            <CodeBlock code="gfs status"/>

            <h2>"5. Connect to Your Database"</h2>
            <CodeBlock code="psql -h localhost -p 5432 -U postgres -d postgres"/>

            <h2>"6. Make Changes and Commit"</h2>
            <p>"After modifying your database schema or data:"</p>
            <CodeBlock code="gfs commit -m \"my first commit\""/>

            <h2>"7. View Commit History"</h2>
            <CodeBlock code="gfs log"/>

            <h2>"8. Time Travel"</h2>
            <p>"Checkout a previous commit:"</p>
            <CodeBlock code="gfs checkout <commit_hash>"/>

            <h2>"9. Work with Branches"</h2>
            <p>"Create a new branch:"</p>
            <CodeBlock code="gfs checkout -b release"/>
            <p>"Switch back to main:"</p>
            <CodeBlock code="gfs checkout main"/>

            <h2>"Next Steps"</h2>
            <p>"Explore the "<a href="/docs/commands/init">"Commands"</a>" section to learn more about what you can do with GFS."</p>
        </div>
    }
}

#[component]
fn CommandProviders() -> impl IntoView {
    view! {
        <div>
            <h1>"gfs providers"</h1>
            <p class="lead">"List available database providers and their supported versions."</p>

            <h2>"Usage"</h2>
            <CodeBlock code="gfs providers [PROVIDER]"/>

            <h2>"Description"</h2>
            <p>"The "<code>"providers"</code>" command displays all supported database providers along with their available versions and features."</p>

            <h2>"Examples"</h2>
            <h3>"List all providers"</h3>
            <CodeBlock code="gfs providers"/>
            <p>"Output:"</p>
            <pre><code>"  database_provider    | version                        | features                                          \n  ---------------------+--------------------------------+---------------------------------------------------\n  postgres             | 13, 14, 15, 16, 17, 18         | tls, schema, masking, auto-scaling, performance...\n  mysql                | 8.0, 8.1                       | tls, schema, masking, backup, import              \n\n  Images are pulled from Docker Hub by default."</code></pre>

            <h3>"Show details for a specific provider"</h3>
            <CodeBlock code="gfs providers postgres"/>

            <h2>"Supported Providers"</h2>
            <h3>"PostgreSQL"</h3>
            <ul>
                <li><strong>"Versions:"</strong>" 13, 14, 15, 16, 17, 18"</li>
                <li><strong>"Features:"</strong>" TLS, schema management, data masking, auto-scaling, performance monitoring"</li>
            </ul>

            <h3>"MySQL"</h3>
            <ul>
                <li><strong>"Versions:"</strong>" 8.0, 8.1"</li>
                <li><strong>"Features:"</strong>" TLS, schema management, data masking, backup, import"</li>
            </ul>

            <h2>"Use Case"</h2>
            <p>"Run this command before initializing a repository to see what database providers and versions are available."</p>

            <h2>"See Also"</h2>
            <ul>
                <li><a href="/docs/commands/init">"gfs init"</a>" - Initialize a repository with a specific provider"</li>
            </ul>
        </div>
    }
}

#[component]
fn CommandInit() -> impl IntoView {
    view! {
        <div>
            <h1>"gfs init"</h1>
            <p class="lead">"Initialize a new GFS repository."</p>

            <h2>"Usage"</h2>
            <CodeBlock code="gfs init --database-provider <PROVIDER> --database-version <VERSION>"/>

            <h2>"Options"</h2>
            <ul>
                <li><code>"--database-provider"</code>" (required) - Database type (e.g., "<code>"postgres"</code>", "<code>"mysql"</code>")"</li>
                <li><code>"--database-version"</code>" (required) - Database version (e.g., "<code>"17"</code>" for PostgreSQL, "<code>"8.0"</code>" for MySQL)"</li>
            </ul>

            <h2>"Description"</h2>
            <p>"The "<code>"init"</code>" command creates a new GFS repository in the current directory. It:"</p>
            <ul>
                <li>"Creates a "<code>".gfs"</code>" directory to store repository metadata"</li>
                <li>"Starts a Docker container with the specified database"</li>
                <li>"Initializes the database for version control"</li>
                <li>"Creates an initial commit (root commit)"</li>
            </ul>

            <h2>"Examples"</h2>
            <h3>"Initialize with PostgreSQL 17"</h3>
            <CodeBlock code="gfs init --database-provider postgres --database-version 17"/>

            <h3>"Initialize with MySQL 8.0"</h3>
            <CodeBlock code="gfs init --database-provider mysql --database-version 8.0"/>

            <h2>"What Happens"</h2>
            <ol>
                <li>"A "<code>".gfs"</code>" directory is created in your current directory"</li>
                <li>"Docker pulls the specified database image if not already available"</li>
                <li>"A Docker container starts with the database"</li>
                <li>"The database is configured for GFS version control"</li>
                <li>"Connection information is displayed"</li>
            </ol>

            <h2>"Connection Information"</h2>
            <p>"After initialization, you can connect to your database using the displayed connection details:"</p>
            <pre><code>"# For PostgreSQL\npsql -h localhost -p 5432 -U postgres -d postgres\n\n# For MySQL\nmysql -h localhost -P 3306 -u root"</code></pre>

            <h2>"Requirements"</h2>
            <ul>
                <li>"Docker must be installed and running"</li>
                <li>"The current directory should be empty or not already a GFS repository"</li>
                <li>"Sufficient disk space for the database container"</li>
            </ul>

            <h2>"See Also"</h2>
            <ul>
                <li><a href="/docs/commands/providers">"gfs providers"</a>" - List available providers and versions"</li>
                <li><a href="/docs/commands/status">"gfs status"</a>" - Check repository status"</li>
            </ul>
        </div>
    }
}

#[component]
fn CommandStatus() -> impl IntoView {
    view! {
        <div>
            <h1>"gfs status"</h1>
            <p class="lead">"Show the current state of storage and compute resources."</p>

            <h2>"Usage"</h2>
            <CodeBlock code="gfs status"/>

            <h2>"Description"</h2>
            <p>"The "<code>"status"</code>" command displays information about your GFS repository, including:"</p>
            <ul>
                <li>"Current branch"</li>
                <li>"Database connection information"</li>
                <li>"Docker container status"</li>
                <li>"Storage backend information"</li>
                <li>"Compute resource status"</li>
            </ul>

            <h2>"Example Output"</h2>
            <pre><code>"  Repository\n  ────────────────────────────────────────\n  Branch               main\n  Active workspace     .gfs/workspaces/main/0/data\n\n  Compute\n  ────────────────────────────────────────\n  Provider             postgres\n  Version              17\n  Status               ● running\n  Container ID         360bc81dd287…\n  Container data dir   .gfs/workspaces/main/0/data\n  Connection           postgresql://postgres:postgres@localhost:55024/postgres"</code></pre>

            <h2>"Use Cases"</h2>
            <ul>
                <li>"Check if the database container is running"</li>
                <li>"Get connection details for your database"</li>
                <li>"Verify which branch you're currently on"</li>
                <li>"Troubleshoot connection issues"</li>
            </ul>

            <h2>"See Also"</h2>
            <ul>
                <li><a href="/docs/commands/init">"gfs init"</a>" - Initialize a repository"</li>
                <li><a href="/docs/commands/log">"gfs log"</a>" - View commit history"</li>
            </ul>
        </div>
    }
}

#[component]
fn CommandCommit() -> impl IntoView {
    view! {
        <div>
            <h1>"gfs commit"</h1>
            <p class="lead">"Commit the current database state."</p>

            <h2>"Usage"</h2>
            <CodeBlock code="gfs commit -m <MESSAGE>"/>

            <h2>"Options"</h2>
            <ul>
                <li><code>"-m, --message"</code>" (required) - Commit message describing the changes"</li>
            </ul>

            <h2>"Description"</h2>
            <p>"The "<code>"commit"</code>" command creates a snapshot of your current database state, including:"</p>
            <ul>
                <li>"Schema changes (tables, columns, indexes, constraints)"</li>
                <li>"Data changes (inserts, updates, deletes)"</li>
                <li>"Database configuration"</li>
            </ul>

            <h2>"Examples"</h2>
            <h3>"Commit with a message"</h3>
            <CodeBlock code="gfs commit -m \"Add users table\""/>
            <p>"Output:"</p>
            <pre><code>"[main] 23bff20  Add users table"</code></pre>

            <h3>"Commit schema changes"</h3>
            <CodeBlock code="gfs commit -m \"Add email column to users table\""/>

            <h3>"Commit data changes"</h3>
            <CodeBlock code="gfs commit -m \"Import initial user data\""/>

            <h2>"How It Works"</h2>
            <ol>
                <li>"GFS captures a complete snapshot of your database"</li>
                <li>"The snapshot is stored efficiently using deduplication"</li>
                <li>"A commit hash is generated"</li>
                <li>"The commit is added to the current branch's history"</li>
            </ol>

            <h2>"Best Practices"</h2>
            <ul>
                <li>"Write clear, descriptive commit messages"</li>
                <li>"Commit logical units of change"</li>
                <li>"Test your changes before committing"</li>
                <li>"Commit frequently to maintain detailed history"</li>
            </ul>

            <h2>"Commit Message Guidelines"</h2>
            <ul>
                <li>"Use imperative mood: \"Add column\" not \"Added column\""</li>
                <li>"Be specific: \"Add email index to users table\" not \"Update database\""</li>
                <li>"Keep it concise but informative"</li>
            </ul>

            <h2>"See Also"</h2>
            <ul>
                <li><a href="/docs/commands/log">"gfs log"</a>" - View commit history"</li>
                <li><a href="/docs/commands/checkout">"gfs checkout"</a>" - Switch to a different commit"</li>
                <li><a href="/docs/commands/status">"gfs status"</a>" - Check repository status"</li>
            </ul>
        </div>
    }
}

#[component]
fn CommandLog() -> impl IntoView {
    view! {
        <div>
            <h1>"gfs log"</h1>
            <p class="lead">"Show the commit history."</p>

            <h2>"Usage"</h2>
            <CodeBlock code="gfs log"/>

            <h2>"Description"</h2>
            <p>"The "<code>"log"</code>" command displays the commit history of the current branch, showing:"</p>
            <ul>
                <li>"Commit hash (short form)"</li>
                <li>"Commit message"</li>
                <li>"Author information"</li>
                <li>"Timestamp"</li>
                <li>"Branch information"</li>
            </ul>

            <h2>"Example Output"</h2>
            <pre><code>"commit 23bff20e3f52d7a197d0de438194b5d950e0d07e617293f364b6007feee7983b (HEAD -> main, main)\nAuthor: user\nDate:   Thu Feb 26 05:25:04 2026 +0000\n\n    Test commit after schema changes"</code></pre>

            <h2>"Understanding the Output"</h2>
            <ul>
                <li><strong>"commit hash"</strong>" - Unique identifier for the commit"</li>
                <li><strong>"HEAD"</strong>" - Current commit you're on"</li>
                <li><strong>"branch name"</strong>" - Branch this commit belongs to"</li>
                <li><strong>"Author"</strong>" - Who made the commit"</li>
                <li><strong>"Date"</strong>" - When the commit was made"</li>
                <li><strong>"message"</strong>" - Description of the changes"</li>
            </ul>

            <h2>"Use Cases"</h2>
            <ul>
                <li>"Review what changes were made to the database"</li>
                <li>"Find a specific commit to checkout"</li>
                <li>"Understand the evolution of your database schema"</li>
                <li>"Debug when a change was introduced"</li>
            </ul>

            <h2>"See Also"</h2>
            <ul>
                <li><a href="/docs/commands/commit">"gfs commit"</a>" - Create a new commit"</li>
                <li><a href="/docs/commands/checkout">"gfs checkout"</a>" - Switch to a different commit"</li>
            </ul>
        </div>
    }
}

#[component]
fn CommandCheckout() -> impl IntoView {
    view! {
        <div>
            <h1>"gfs checkout"</h1>
            <p class="lead">"Switch to a different commit or branch."</p>

            <h2>"Usage"</h2>
            <CodeBlock code="# Checkout a specific commit
gfs checkout <COMMIT_HASH>

# Create and checkout a new branch
gfs checkout -b <BRANCH_NAME>

# Checkout an existing branch
gfs checkout <BRANCH_NAME>"/>

            <h2>"Options"</h2>
            <ul>
                <li><code>"-b"</code>" - Create a new branch and switch to it"</li>
            </ul>

            <h2>"Description"</h2>
            <p>"The "<code>"checkout"</code>" command allows you to:"</p>
            <ul>
                <li>"Travel back to any previous database state"</li>
                <li>"Switch between branches"</li>
                <li>"Create new branches from the current state"</li>
            </ul>
            <p>"When you checkout a commit or branch, GFS restores your database to that exact state."</p>

            <h2>"Examples"</h2>
            <h3>"Checkout a specific commit"</h3>
            <CodeBlock code="gfs checkout 23bff20e3f52d7a197d0de438194b5d950e0d07e617293f364b6007feee7983b"/>
            <p>"Your database will be restored to the state at that commit."</p>

            <h3>"Create a new branch"</h3>
            <CodeBlock code="gfs checkout -b feature-test"/>
            <p>"Output:"</p>
            <pre><code>"Switched to new branch 'feature-test' (23bff20)"</code></pre>

            <h3>"Switch to an existing branch"</h3>
            <CodeBlock code="gfs checkout main"/>
            <p>"Switches back to the "<code>"main"</code>" branch."</p>

            <h2>"How It Works"</h2>
            <ol>
                <li>"GFS stops the current database container"</li>
                <li>"The database storage is restored to the target commit"</li>
                <li>"A new container starts with the restored state"</li>
                <li>"You can now work with the database at that point in history"</li>
            </ol>

            <h2>"Time Travel Example"</h2>
            <CodeBlock code="# View your commits
gfs log

# Go back to a previous commit
gfs checkout d4e5f6g

# Check your database - it's restored to that point!
psql -h localhost -p 5432 -U postgres

# Return to the latest state
gfs checkout main"/>

            <h2>"Working with Branches"</h2>
            <CodeBlock code="# Create a branch for experimental changes
gfs checkout -b experiment

# Make changes to your database
# ...

# Commit your changes
gfs commit -m \"Experimental schema changes\"

# Switch back to main
gfs checkout main

# Your database is back to main's state
# The experimental changes are preserved in the experiment branch"/>

            <h2>"Important Notes"</h2>
            <ul>
                <li>"Any uncommitted changes will be lost when checking out"</li>
                <li>"The database container is recreated during checkout"</li>
                <li>"Active connections to the database will be closed"</li>
            </ul>

            <h2>"See Also"</h2>
            <ul>
                <li><a href="/docs/commands/log">"gfs log"</a>" - View commit history to find commit hashes"</li>
                <li><a href="/docs/commands/commit">"gfs commit"</a>" - Save changes before checking out"</li>
                <li><a href="/docs/commands/status">"gfs status"</a>" - Check current branch"</li>
            </ul>
        </div>
    }
}

#[component]
fn ComingSoon(#[prop(into)] page: String) -> impl IntoView {
    view! {
        <div>
            <h1>"Coming Soon"</h1>
            <p>"Documentation for "<code>{page}</code>" is coming soon."</p>
            <p>"In the meantime, check out the "<a href="https://github.com/Guepard-Corp/gfs" target="_blank">"GitHub repository"</a>" for more information."</p>
        </div>
    }
}
