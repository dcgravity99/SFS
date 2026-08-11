/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

export class FeatureFlagManager {
  private static flags: Map<string, boolean> = new Map();

  public static setFlag(flag: string, enabled: boolean): void {
    FeatureFlagManager.flags.set(flag, enabled);
  }

  public static isEnabled(flag: string, defaultValue = false): boolean {
    return FeatureFlagManager.flags.has(flag)
      ? (FeatureFlagManager.flags.get(flag) as boolean)
      : defaultValue;
  }
}
