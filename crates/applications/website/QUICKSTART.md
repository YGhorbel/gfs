# GFS Website - Quick Start

## What's Been Created

A complete Leptos-based static website for GFS documentation with:

✅ **Design**: Dark theme matching the qwery.ai aesthetic with yellow (#FFCB51) accents
✅ **Home Page**: Hero section, installation tabs, features grid, workflow steps, FAQ
✅ **Documentation**: Sidebar navigation, getting started, installation, quick start guides
✅ **Components**: Reusable Header, Footer, Hero, FeatureCard, CodeTabs, FAQ
✅ **Responsive**: Mobile-friendly design
✅ **Ready to Deploy**: Configurations for Netlify, Vercel, Cloudflare Pages, GitHub Pages

## Getting Started

### 1. Install Prerequisites

```bash
# Install Trunk (Leptos build tool)
cargo install trunk

# Install wasm target
rustup target add wasm32-unknown-unknown
```

### 2. Run Locally

```bash
cd crates/applications/website
trunk serve
```

Visit http://localhost:8080 to see the site!

### 3. Build for Production

```bash
cd crates/applications/website
./build.sh
# or
trunk build --release
```

Static files will be in `dist/`

## Deploy to https://gfs.guepard.run/

### Option 1: Cloudflare Pages Dashboard (Recommended)

1. Go to [Cloudflare Pages](https://dash.cloudflare.com/?to=/:account/pages)
2. Connect your GitHub repository
3. Configure build:
   - Build command: `cd crates/applications/website && ./build.sh`
   - Build output: `crates/applications/website/dist`
4. Deploy! Every push to `main` will auto-deploy

### Option 2: Wrangler CLI

```bash
cd crates/applications/website
./build.sh
wrangler pages deploy dist --project-name=gfs
```

## Project Structure

```
crates/applications/website/
├── src/
│   ├── lib.rs              # Main app and routing
│   ├── components/         # Reusable UI components
│   │   ├── header.rs
│   │   ├── footer.rs
│   │   ├── hero.rs
│   │   ├── feature_card.rs
│   │   ├── code_tabs.rs
│   │   └── faq.rs
│   └── pages/              # Page components
│       ├── home.rs         # Home page
│       ├── docs.rs         # Documentation
│       └── not_found.rs    # 404 page
├── style/
│   └── main.css           # All styles (dark theme)
├── public/
│   └── favicon.svg        # Site icon
├── index.html             # HTML template
├── Trunk.toml            # Build configuration
├── build.sh              # Build script
├── netlify.toml          # Netlify config
├── vercel.json           # Vercel config
└── DEPLOYMENT.md         # Detailed deployment guide

```

## Customization

### Update Content

- **Home page**: Edit `src/pages/home.rs`
- **Docs**: Edit `src/pages/docs.rs`
- **FAQ**: Edit `src/components/faq.rs`

### Change Colors

Edit `style/main.css`:

```css
:root {
    --accent: #ffcb51;     /* Yellow accent color */
    --bg-primary: #000000;  /* Black background */
    /* ... */
}
```

### Add New Pages

1. Create component in `src/pages/`
2. Add route in `src/lib.rs`

## Troubleshooting

### "trunk: command not found"

```bash
cargo install trunk
```

### "wasm32-unknown-unknown target not found"

```bash
rustup target add wasm32-unknown-unknown
```

### CSS not loading

Check that `index.html` has:
```html
<link data-trunk rel="css" href="style/main.css">
```

## Next Steps

1. **Test Locally**: `trunk serve` and visit http://localhost:8080
2. **Review Content**: Check all pages and update as needed
3. **Deploy**: Push to your repo and let CI/CD deploy, or build and upload manually
4. **Configure DNS**: Point gfs.guepard.run to your hosting provider

## Features Implemented

- ✅ Hero section with yellow highlights
- ✅ Installation tabs (curl, Homebrew, Cargo)
- ✅ Features grid (6 key features)
- ✅ Workflow steps (4-step guide)
- ✅ FAQ with expandable items
- ✅ Documentation with sidebar navigation
- ✅ Responsive mobile design
- ✅ Fast loading with WASM
- ✅ SEO optimized with meta tags
- ✅ Security headers configured

## Live Development

Changes to `.rs` files will trigger rebuilds automatically when using `trunk serve`.
Changes to CSS require a manual refresh.

Enjoy your new GFS website! 🚀
