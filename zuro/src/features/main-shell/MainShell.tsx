export function MainShell(): React.JSX.Element {
  return (
    <main className="main-shell">
      <section className="main-shell__hero">
        <p className="main-shell__eyebrow">Zuro Desktop Shell</p>
        <h1>Browser stays in the tray. Pet restores the app.</h1>
        <p className="main-shell__body">
          Minimize or close this window to send Zuro to the background. The floating pet remains
          available on the desktop and the tray icon can restore the browser at any time.
        </p>
      </section>
      <section className="main-shell__panel">
        <div className="main-shell__metric">
          <span>Behavior</span>
          <strong>Close to tray</strong>
        </div>
        <div className="main-shell__metric">
          <span>Desktop helper</span>
          <strong>Floating pet</strong>
        </div>
        <div className="main-shell__metric">
          <span>Restore</span>
          <strong>Pet or tray click</strong>
        </div>
      </section>
    </main>
  );
}
