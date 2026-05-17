import { MainShell } from "../features/main-shell/MainShell";
import { PetWindow } from "../features/pet/PetWindow";
import { detectWindowSurface } from "./window-context";

export function AppRoot(): React.JSX.Element {
  const surface = detectWindowSurface();

  if (surface === "pet") {
    return <PetWindow />;
  }

  return <MainShell />;
}
