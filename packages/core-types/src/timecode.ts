/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

export type RationalFrameRate = {
  numerator: number;
  denominator: number;
};

export const FRAME_RATES = {
  FILM_23_976: { numerator: 24000, denominator: 1001 } as RationalFrameRate,
  FILM_24: { numerator: 24, denominator: 1 } as RationalFrameRate,
  PAL_25: { numerator: 25, denominator: 1 } as RationalFrameRate,
  NTSC_29_97: { numerator: 30000, denominator: 1001 } as RationalFrameRate,
  HFR_59_94: { numerator: 60000, denominator: 1001 } as RationalFrameRate,
};

export class SiraTimecode {
  public hours: number;
  public minutes: number;
  public seconds: number;
  public frames: number;
  public isDropFrame: boolean;
  public frameRate: RationalFrameRate;

  constructor(
    hours = 0,
    minutes = 0,
    seconds = 0,
    frames = 0,
    isDropFrame = false,
    frameRate = FRAME_RATES.FILM_24
  ) {
    this.hours = hours;
    this.minutes = minutes;
    this.seconds = seconds;
    this.frames = frames;
    this.isDropFrame = isDropFrame;
    this.frameRate = frameRate;
  }

  public toString(): string {
    const hh = String(this.hours).padStart(2, '0');
    const mm = String(this.minutes).padStart(2, '0');
    const ss = String(this.seconds).padStart(2, '0');
    const ff = String(this.frames).padStart(2, '0');
    const sep = this.isDropFrame ? ';' : ':';
    return `${hh}:${mm}:${ss}${sep}${ff}`;
  }
}
