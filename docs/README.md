# proj Documentation

This directory contains the documentation website for `proj`, built with [Docusaurus](https://docusaurus.io/).

## Development

```bash
# Install dependencies
npm install

# Start development server
npm start

# Build for production
npm run build

# Serve production build locally
npm run serve
```

The site will be available at `http://localhost:3000/proj/`

## Deployment

The site is configured to deploy to GitHub Pages at `https://ybouhjira.github.io/proj/`

### GitHub Actions Deployment

To deploy automatically on push, add this workflow to `.github/workflows/docs.yml`:

```yaml
name: Deploy Documentation

on:
  push:
    branches: [main]
    paths:
      - 'docs/**'
  workflow_dispatch:

permissions:
  contents: read
  pages: write
  id-token: write

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 18
          cache: npm
          cache-dependency-path: docs/package-lock.json

      - name: Install dependencies
        working-directory: docs
        run: npm ci

      - name: Build website
        working-directory: docs
        run: npm run build

      - name: Upload artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: docs/build

  deploy:
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    runs-on: ubuntu-latest
    needs: build
    steps:
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

### Manual Deployment

```bash
# Build the site
npm run build

# Deploy to GitHub Pages (requires gh CLI)
cd build
git init
git add .
git commit -m "Deploy docs"
git remote add origin https://github.com/ybouhjira/proj.git
git push -f origin HEAD:gh-pages
```

## Structure

```
docs/
├── docs/               # Documentation pages
│   ├── getting-started.md
│   ├── commands.md
│   ├── configuration.md
│   ├── shell-integration.md
│   └── ai-checks.md
├── src/
│   ├── components/     # React components
│   ├── css/            # Global styles
│   └── pages/          # Custom pages (landing page)
├── static/             # Static assets
└── docusaurus.config.ts
```

## Customization

### Theme

The site uses a custom dark theme with terminal aesthetics. See:
- `src/css/custom.css` - Global theme variables
- `src/pages/index.module.css` - Landing page styles
- `src/components/HomepageFeatures/styles.module.css` - Feature grid styles

### Content

- Landing page: `src/pages/index.tsx`
- Documentation: `docs/*.md`
- Configuration: `docusaurus.config.ts`

## License

Same as the main project.
