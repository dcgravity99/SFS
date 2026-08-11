#!/usr/bin/env bash
# ==============================================================================
# Siragugal Film Studio — Batch 2 (Modules 19–24) Validation Orchestrator
# Copyright (C) 2026 Siragugal Film Studio Contributors
# Licensed under Apache-2.0 or MIT
# ==============================================================================

set -euo pipefail

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_DIR"

REPORT_MD="$REPO_DIR/docs/MODULE_19_24_VALIDATION_REPORT.md"
REPORT_JSON="$REPO_DIR/docs/MODULE_19_24_VALIDATION_REPORT.json"

echo "==============================================================================="
echo " SIRAGUGAL FILM STUDIO — BATCH 2 (MODULES 19–24) VALIDATION ORCHESTRATOR"
echo " Repository: $REPO_DIR"
echo "==============================================================================="

PASSED_COUNT=0
FAILED_COUNT=0
BLOCKED_COUNT=0

mkdir -p "$REPO_DIR/docs"

cat <<EOF > "$REPORT_MD"
# SIRAGUGAL FILM STUDIO — MODULES 19–24 VALIDATION REPORT

**Repository**: \`$REPO_DIR\`  
**Timestamp**: $(date -u +"%Y-%m-%dT%H:%M:%SZ")  
**Target Batch**: \`Batch 2 (Modules 19–24)\`  

---

## Module Validation Results

| Module | Name | Primary Crate | Status | Verification Detail |
| :--- | :--- | :--- | :---: | :--- |
EOF

echo "{" > "$REPORT_JSON"
echo "  \"timestamp\": \"$(date -u +"%Y-%m-%dT%H:%M:%SZ")\"," >> "$REPORT_JSON"
echo "  \"modules\": [" >> "$REPORT_JSON"

validate_module() {
    local mod_num="$1"
    local mod_name="$2"
    local crate_name="$3"
    local file_check="$4"

    local status="NOT_IMPLEMENTED"
    local detail=""

    if [[ -f "$file_check" || -d "$file_check" ]]; then
        if cargo check -p "$crate_name" --locked 2>/dev/null; then
            status="PASS"
            detail="Crate $crate_name checked cleanly"
            PASSED_COUNT=$((PASSED_COUNT + 1))
        else
            status="FAIL"
            detail="Crate $crate_name check failed"
            FAILED_COUNT=$((FAILED_COUNT + 1))
        fi
    else
        status="BLOCKED"
        detail="Missing expected source file: $file_check"
        BLOCKED_COUNT=$((BLOCKED_COUNT + 1))
    fi

    echo "| Module $mod_num | $mod_name | $crate_name | $status | $detail |" >> "$REPORT_MD"
    echo "    {\"module\": $mod_num, \"name\": \"$mod_name\", \"crate\": \"$crate_name\", \"status\": \"$status\", \"detail\": \"$detail\"}," >> "$REPORT_JSON"
    echo "Module $mod_num ($mod_name): $status - $detail"
}

# Batch 2 Validation (Modules 19–24)
validate_module 19 "3D Scene Composition Engine" "sira_engine_scene" "packages/sira-engine-scene/src/scene_compositor.rs"
validate_module 20 "Timeline NLE Engine" "sira_engine_timeline" "packages/sira-engine-timeline/src/nle_timeline.rs"
validate_module 21 "Multi-Track Audio Engine" "sira_engine_audio" "packages/sira-engine-audio/src/multitrack_mixer.rs"
validate_module 22 "Render Compositor Engine" "sira_engine_render" "packages/sira-engine-render/src/layer_compositor.rs"
validate_module 23 "VFX Engine" "sira_engine_render" "packages/sira-engine-render/src/vfx_engine.rs"
validate_module 24 "Color Grading & ACES Suite" "sira_engine_render" "packages/sira-engine-render/src/color_suite.rs"

cat <<EOF >> "$REPORT_MD"

---

## Validation Summary

- **Passed**: $PASSED_COUNT
- **Failed**: $FAILED_COUNT
- **Blocked**: $BLOCKED_COUNT
EOF

# Remove trailing comma in JSON
sed -i '$ s/,$//' "$REPORT_JSON" 2>/dev/null || true
echo "  ]" >> "$REPORT_JSON"
echo "}" >> "$REPORT_JSON"

echo "==============================================================================="
echo " Batch 2 Validation Complete!"
echo " Report (Markdown): $REPORT_MD"
echo " Report (JSON):     $REPORT_JSON"
echo "==============================================================================="
