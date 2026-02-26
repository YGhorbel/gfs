# Deployment Guide

This guide covers deploying the GFS website to Cloudflare Pages at https://gfs.guepard.run/

### Wrangler CLI

For manual deployments using the command line:

```bash
# Install Wrangler (if not already installed)
npm install -g wrangler

# Login to Cloudflare
wrangler login

# Build the site
cd crates/applications/website
./build.sh

# Deploy to Cloudflare Pages
wrangler pages deploy dist --project-name=gfs
```

## Performance Optimization

The build is already optimized with:
- ✅ Release mode compilation
- ✅ LTO (Link Time Optimization)
- ✅ WASM binary optimization
- ✅ Code splitting

For additional optimization:

```bash
# Install wasm-opt
cargo install wasm-opt

# Optimize WASM after build
wasm-opt -Oz -o dist/optimized.wasm dist/*.wasm
```

## Rollback

Or use Wrangler CLI:

```bash
wrangler pages deployment list --project-name=gfs
wrangler pages deployment rollback <DEPLOYMENT_ID> --project-name=gfs
```
