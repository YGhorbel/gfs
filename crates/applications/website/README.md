# GFS Website

Documentation website for GFS (Git For database Systems).

## Development

### Prerequisites

- Rust (1.93+)
- [Trunk](https://trunkrs.dev/) - Install with: `cargo install trunk`
- wasm32-unknown-unknown target: `rustup target add wasm32-unknown-unknown`

### Running Locally

```bash
cd crates/applications/website
trunk serve
```

The site will be available at http://localhost:8080

### Building for Production

```bash
trunk build --release
```

The static files will be generated in the `dist/` directory.

## Deployment

The site is deployed to https://gfs.guepard.run/ using Cloudflare Pages.

### Deploy to Cloudflare Pages:

1. Connect your GitHub repository to Cloudflare Pages
2. Configure build settings:
   - Build command: `cd crates/applications/website && ./build.sh`
   - Build output directory: `crates/applications/website/dist`
3. Cloudflare will automatically build and deploy on every push to main

### Manual Deployment:

```bash
# Build the site
cd crates/applications/website
./build.sh

# Deploy using Wrangler CLI
wrangler pages deploy dist --project-name=gfs
```

See [DEPLOYMENT.md](DEPLOYMENT.md) for detailed instructions.

## Structure

- `src/lib.rs` - Main app component and routing
- `src/components/` - Reusable UI components
- `src/pages/` - Page components
- `style/main.css` - Global styles
- `index.html` - HTML template
- `public/` - Static assets
- `cloudflare-pages.toml` - Cloudflare Pages configuration
- `build.sh` - Build script

## Features

- ✅ Responsive design
- ✅ Dark theme matching qwery design
- ✅ Documentation with sidebar navigation
- ✅ FAQ section with expandable items
- ✅ Code tabs for installation methods
- ✅ SEO optimized with meta tags
- ✅ Fast loading with Leptos CSR
- ✅ Static site generation ready
