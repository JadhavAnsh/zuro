export type AppWindowSurface = "main" | "pet";

const PET_ROUTE = "#/pet";

export function detectWindowSurface(): AppWindowSurface {
  return window.location.hash === PET_ROUTE ? "pet" : "main";
}
