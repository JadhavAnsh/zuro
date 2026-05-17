import { PetAvatar } from "./PetAvatar";

interface PetButtonProps {
  isAwake: boolean;
  isRestoring: boolean;
  onRestore: () => void;
}

export function PetButton({
  isAwake,
  isRestoring,
  onRestore,
}: PetButtonProps): React.JSX.Element {
  return (
    <button
      className="pet-button"
      type="button"
      onClick={onRestore}
      aria-label="Restore Zuro Browser"
      disabled={isRestoring}
    >
      <PetAvatar isAwake={isAwake} />
      <span className="pet-button__label">
        {isRestoring ? "Restoring..." : "Open Zuro"}
      </span>
    </button>
  );
}
