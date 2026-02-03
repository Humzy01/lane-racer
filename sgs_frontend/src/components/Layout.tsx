import './Layout.css';

interface LayoutProps {
  children: React.ReactNode;
}

export function Layout({ children }: LayoutProps) {
  return (
    <div className="studio docs">
      <div className="studio-background" aria-hidden="true">
        <div className="studio-orb orb-1" />
        <div className="studio-orb orb-2" />
        <div className="studio-orb orb-3" />
        <div className="studio-grid" />
      </div>

      <header className="studio-header">
        <div className="brand">
          <div className="brand-title">Stellar Game Studio</div>
          <p className="brand-subtitle">
            Build deterministic two-player games on Stellar with a testnet sandbox and a production-ready wallet flow.
          </p>
        </div>
        <div className="header-actions">
          <a className="header-link" href="#quickstart">Quickstart</a>
          <a
            className="button primary small"
            href="https://github.com/jamesbachini/Stellar-Game-Studio"
            target="_blank"
            rel="noreferrer"
          >
            Fork on GitHub
          </a>
        </div>
      </header>

      <main className="studio-main">{children}</main>

      <footer className="studio-footer">
        <span>Stellar Game Studio is an open-source starter kit for Soroban game developers.</span>
        <span className="footer-meta">Host-ready docs at jamesbachini.github.io/Stellar-Game-Studio</span>
      </footer>
    </div>
  );
}
