import { invoke } from "@tauri-apps/api/core";
import type { AppVisibilityMode, PetWindowState, RestoreMainWindowResult } from "../types";

export function restoreMainWindow(): Promise<RestoreMainWindowResult> {
  return invoke<RestoreMainWindowResult>("restore_main_window");
}

export function showPet(): Promise<PetWindowState> {
  return invoke<PetWindowState>("show_pet");
}

export function hidePet(): Promise<PetWindowState> {
  return invoke<PetWindowState>("hide_pet");
}

export function quitFromTray(): Promise<void> {
  return invoke("quit_from_tray");
}

export function getVisibilityMode(): Promise<AppVisibilityMode> {
  return invoke<AppVisibilityMode>("get_visibility_mode");
}
