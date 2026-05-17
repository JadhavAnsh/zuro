import { useEffect, useState } from "react";
import { getVisibilityMode, restoreMainWindow } from "./api/pet-shell";
import { PetButton } from "./components/PetButton";
import type { AppVisibilityMode } from "./types";

export function PetWindow(): React.JSX.Element {
  const [isRestoring, setIsRestoring] = useState(false);
  const [visibilityMode, setVisibilityMode] = useState<AppVisibilityMode>("pet-visible");

  useEffect(() => {
    let isMounted = true;

    void getVisibilityMode()
      .then((mode) => {
        if (isMounted) {
          setVisibilityMode(mode);
        }
      })
      .catch(() => {
        // The pet surface remains functional even if visibility sync fails.
      });

    return () => {
      isMounted = false;
    };
  }, []);

  function handleRestore(): void {
    setIsRestoring(true);

    void restoreMainWindow()
      .then(() => {
        setVisibilityMode("visible");
      })
      .finally(() => {
        setIsRestoring(false);
      });
  }

  return (
    <main className="pet-window">
      <section className="pet-window__shell" data-mode={visibilityMode}>
        <span className="pet-window__status">Background mode active</span>
        <PetButton
          isAwake={visibilityMode !== "visible"}
          isRestoring={isRestoring}
          onRestore={handleRestore}
        />
        <p className="pet-window__hint">Click the mascot or the tray icon to bring Zuro back.</p>
      </section>
    </main>
  );
}
