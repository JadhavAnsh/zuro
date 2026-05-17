export type AppVisibilityMode = "visible" | "tray-only" | "pet-visible";

export interface PetWindowState {
  visible: boolean;
}

export interface RestoreMainWindowResult {
  restored: boolean;
}
