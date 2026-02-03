import { Layout } from './components/Layout';
import { Resources } from './components/Resources';
import './App.css';

function App() {
  return (
    <Layout>
      <section className="docs-hero">
        <div className="hero-content">
          <p className="hero-eyebrow">Stellar Game Studio</p>
          <h1>Build two-player Soroban games that ship fast.</h1>
          <p className="hero-lede">
            A developer-first toolkit for Stellar games: deterministic testnet flows, instant player
            switching, and a production build that plugs in CreitTech&apos;s wallet kit v2.
          </p>
          <div className="hero-actions">
            <a
              className="button primary"
              href="https://github.com/jamesbachini/Stellar-Game-Studio"
              target="_blank"
              rel="noreferrer"
            >
              Fork the repo
            </a>
            <a className="button ghost" href="#quickstart">
              Read the quickstart
            </a>
          </div>
          <div className="hero-tags">
            <span>Testnet dev wallets</span>
            <span>start_game + end_game enforced</span>
            <span>Wallet kit v2 production build</span>
          </div>
        </div>

        <div className="hero-panel">
          <div className="hero-panel-header">Dev-to-Publish Pipeline</div>
          <ol className="hero-steps">
            <li>Fork and clone the repo</li>
            <li>Deploy contracts to testnet</li>
            <li>Build the standalone game frontend</li>
            <li>Publish with a production wallet flow</li>
          </ol>
          <div className="hero-code">
            <pre>
              <code>{`bun run setup
bun run create my-game
bun run dev:game my-game
bun run publish my-game --build`}</code>
            </pre>
          </div>
        </div>
      </section>

      <Resources />
    </Layout>
  );
}

export default App;
