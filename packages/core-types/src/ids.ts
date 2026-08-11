/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

export type ProjectId = string & { readonly __brand: unique symbol };
export type SceneId = string & { readonly __brand: unique symbol };
export type AssetId = string & { readonly __brand: unique symbol };
export type CharacterId = string & { readonly __brand: unique symbol };
export type WorkflowId = string & { readonly __brand: unique symbol };
export type RenderJobId = string & { readonly __brand: unique symbol };

export function createProjectId(id: string): ProjectId { return id as ProjectId; }
export function createSceneId(id: string): SceneId { return id as SceneId; }
export function createAssetId(id: string): AssetId { return id as AssetId; }
export function createCharacterId(id: string): CharacterId { return id as CharacterId; }
export function createWorkflowId(id: string): WorkflowId { return id as WorkflowId; }
export function createRenderJobId(id: string): RenderJobId { return id as RenderJobId; }
