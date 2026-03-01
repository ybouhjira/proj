import type {ReactNode} from 'react';
import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import HomepageFeatures from '@site/src/components/HomepageFeatures';
import Heading from '@theme/Heading';

import styles from './index.module.css';

function HomepageHeader() {
  const {siteConfig} = useDocusaurusContext();
  return (
    <header className={clsx('hero', styles.heroBanner)}>
      <div className="container">
        <Heading as="h1" className={styles.heroTitle}>
          <span className={styles.terminalPrompt}>$</span> proj
        </Heading>
        <p className={styles.heroSubtitle}>{siteConfig.tagline}</p>
        <div className={styles.buttons}>
          <Link
            className="button button--primary button--lg"
            to="/docs/getting-started">
            Get Started
          </Link>
          <Link
            className="button button--secondary button--lg"
            to="https://github.com/ybouhjira/proj">
            View on GitHub
          </Link>
        </div>
        <div className={styles.terminalDemo}>
          <div className={styles.terminalHeader}>
            <span className={styles.terminalButton}></span>
            <span className={styles.terminalButton}></span>
            <span className={styles.terminalButton}></span>
          </div>
          <pre className={styles.terminalContent}>
            <code>
              <span className={styles.prompt}>$</span> proj add ./my-project{'\n'}
              <span className={styles.success}>✓</span> Added project: my-project{'\n'}
              {'\n'}
              <span className={styles.prompt}>$</span> proj list{'\n'}
              my-project    ~/code/my-project{'\n'}
              {'\n'}
              <span className={styles.prompt}>$</span> proj cd my-project{'\n'}
              <span className={styles.comment}># Now in ~/code/my-project</span>
            </code>
          </pre>
        </div>
      </div>
    </header>
  );
}

function ComparisonTable() {
  return (
    <section className={styles.comparison}>
      <div className="container">
        <Heading as="h2">How does it compare?</Heading>
        <table className={styles.comparisonTable}>
          <thead>
            <tr>
              <th>Feature</th>
              <th>proj</th>
              <th>ghq</th>
              <th>gita</th>
              <th>mani</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>Multi-repo management</td>
              <td>✅</td>
              <td>✅</td>
              <td>✅</td>
              <td>✅</td>
            </tr>
            <tr>
              <td>Shell integration</td>
              <td>✅</td>
              <td>✅</td>
              <td>❌</td>
              <td>❌</td>
            </tr>
            <tr>
              <td>Code quality checks</td>
              <td>✅ (coming soon)</td>
              <td>❌</td>
              <td>❌</td>
              <td>❌</td>
            </tr>
            <tr>
              <td>Written in</td>
              <td>Rust</td>
              <td>Go</td>
              <td>Python</td>
              <td>Go</td>
            </tr>
            <tr>
              <td>Configuration</td>
              <td>JSON/TOML</td>
              <td>Git config</td>
              <td>.gita</td>
              <td>YAML</td>
            </tr>
          </tbody>
        </table>
      </div>
    </section>
  );
}

function InstallSection() {
  return (
    <section className={styles.install}>
      <div className="container">
        <Heading as="h2">Installation</Heading>
        <div className={styles.installTabs}>
          <div className={styles.installTab}>
            <h3>Cargo</h3>
            <pre><code>cargo install proj-cli</code></pre>
          </div>
          <div className={styles.installTab}>
            <h3>Homebrew</h3>
            <pre><code>brew install ybouhjira/tap/proj</code></pre>
          </div>
          <div className={styles.installTab}>
            <h3>Binary</h3>
            <pre><code>
              # Download from releases{'\n'}
              curl -L https://github.com/ybouhjira/proj/releases/latest/download/proj-linux-x64 -o proj{'\n'}
              chmod +x proj{'\n'}
              sudo mv proj /usr/local/bin/
            </code></pre>
          </div>
        </div>
        <div className={styles.shellSetup}>
          <p>After installation, add shell integration:</p>
          <pre><code>
            # Bash{'\n'}
            echo 'eval "$(proj init bash)"' &gt;&gt; ~/.bashrc{'\n'}
            {'\n'}
            # Zsh{'\n'}
            echo 'eval "$(proj init zsh)"' &gt;&gt; ~/.zshrc{'\n'}
            {'\n'}
            # Fish{'\n'}
            proj init fish | source
          </code></pre>
        </div>
      </div>
    </section>
  );
}

export default function Home(): ReactNode {
  const {siteConfig} = useDocusaurusContext();
  return (
    <Layout
      title="Home"
      description="Manage all your projects from the terminal">
      <HomepageHeader />
      <main>
        <HomepageFeatures />
        <ComparisonTable />
        <InstallSection />
      </main>
    </Layout>
  );
}
