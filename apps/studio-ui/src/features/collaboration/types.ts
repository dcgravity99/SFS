/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

export interface LocalizedTextMap {
  "ta-IN": string;
  "en-US": string;
}

export type ArtistRole = 'Director' | 'Cinematographer' | 'AudioEngineer' | 'Animator' | 'Editor';

export interface TeamMemberView {
  artist_id: string; // Machine-readable UUIDv7
  display_name: string;
  role: ArtistRole;
  avatar_asset_id?: string;
  is_online: boolean;
}

export interface ReviewCommentView {
  comment_id: string;
  artist_id: string;
  artist_name: string;
  timecode_frame: number;
  content: LocalizedTextMap;
  created_at: string;
}

export type ApprovalStatus = 'Approved' | 'RevisionsRequested' | 'PendingReview';

export interface ShotApprovalStateView {
  shot_id: string;
  approval_status: ApprovalStatus;
  approved_by_artist_id?: string;
  notes?: LocalizedTextMap;
}
