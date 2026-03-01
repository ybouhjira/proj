import type {ReactNode} from 'react';
import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

type FeatureItem = {
  title: string;
  icon: string;
  description: ReactNode;
};

const FeatureList: FeatureItem[] = [
  {
    title: 'Fast Navigation',
    icon: '⚡',
    description: (
      <>
        Jump between projects instantly with shell integration.
        No more typing long paths or searching through directories.
      </>
    ),
  },
  {
    title: 'Multi-Repository Management',
    icon: '📦',
    description: (
      <>
        Manage all your projects from one place. List, add, remove,
        and organize projects with simple commands.
      </>
    ),
  },
  {
    title: 'Shell Integration',
    icon: '🐚',
    description: (
      <>
        Native integration with bash, zsh, and fish shells.
        Use <code>proj cd</code> to change directories seamlessly.
      </>
    ),
  },
  {
    title: 'Code Quality Checks',
    icon: '✅',
    description: (
      <>
        Built-in code quality checks powered by Claude.
        Get instant feedback on your code before committing.
        (Coming soon)
      </>
    ),
  },
  {
    title: 'Fuzzy Search',
    icon: '🔍',
    description: (
      <>
        Find projects quickly with fuzzy search.
        Type part of the name and let proj find it for you.
      </>
    ),
  },
  {
    title: 'Lightweight & Fast',
    icon: '🚀',
    description: (
      <>
        Written in Rust for maximum performance.
        Zero dependencies, instant startup, minimal memory footprint.
      </>
    ),
  },
];

function Feature({title, icon, description}: FeatureItem) {
  return (
    <div className={clsx('col col--4')}>
      <div className={styles.featureCard}>
        <div className={styles.featureIcon}>{icon}</div>
        <Heading as="h3" className={styles.featureTitle}>{title}</Heading>
        <p className={styles.featureDescription}>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): ReactNode {
  return (
    <section className={styles.features}>
      <div className="container">
        <Heading as="h2" className={styles.featuresHeading}>
          Why proj?
        </Heading>
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
