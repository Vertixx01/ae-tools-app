import type { AfterEffectsInstall, ScanSnapshot } from "./types";

export function getHealthScore(snapshot: ScanSnapshot | null): number {
  if (!snapshot) return 0;

  let score = 100;
  if (snapshot.system.ramGb < 32) score -= 16;
  score -= Math.min(snapshot.startupItems.filter((item) => item.score >= 65).length * 7, 28);
  if (!snapshot.installs.length) score -= 18;
  if (snapshot.warnings.length) score -= 8;

  return Math.max(score, 20);
}

export function getAllCachePaths(installs: AfterEffectsInstall[]): string[] {
  return [...new Set(installs.flatMap((install) => install.cachePaths))];
}
