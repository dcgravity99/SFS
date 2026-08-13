/* ============================================================================
 * Siragugal Film Studio — Module 27: Subtitle & Closed Caption Generator
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};
use crate::fountain::DialogueBlock;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubtitleTrackSpec {
    pub track_id: String,
    pub language_code: String, // e.g. "ta-IN", "en-US", "hi-IN"
    pub max_characters_per_line: usize,
    pub max_lines_per_caption: usize,
    pub max_cps: f32,
}

impl Default for SubtitleTrackSpec {
    fn default() -> Self {
        Self {
            track_id: "SUB_TRACK_01".to_string(),
            language_code: "ta-IN".to_string(),
            max_characters_per_line: 37,
            max_lines_per_caption: 2,
            max_cps: 17.0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubtitleSegment {
    pub sequence_number: usize,
    pub start_timecode_ms: u64,
    pub end_timecode_ms: u64,
    pub speaker_name: Option<String>,
    pub text: String,
    pub language_code: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubtitleTrack {
    pub track_id: String,
    pub language_code: String,
    pub segments: Vec<SubtitleSegment>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubtitleValidationReport {
    pub is_compliant: bool,
    pub total_captions: usize,
    pub total_duration_ms: u64,
    pub max_observed_cps: f32,
    pub violations: Vec<String>,
}

#[derive(Default)]
pub struct SubtitleGeneratorEngine;

impl SubtitleGeneratorEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_subtitle_track(
        &self,
        spec: &SubtitleTrackSpec,
        dialogues: &[DialogueBlock],
    ) -> SiraResult<SubtitleTrack> {
        let mut segments = Vec::new();
        let mut current_time_ms: u64 = 1000; // Start at 1.0s

        for (idx, block) in dialogues.iter().enumerate() {
            let word_count = block.speech_text.split_whitespace().count();
            let duration_ms = ((word_count as u64) * 350).max(1500); // ~350ms per word, min 1.5s
            let start_ms = current_time_ms;
            let end_ms = start_ms + duration_ms;

            segments.push(SubtitleSegment {
                sequence_number: idx + 1,
                start_timecode_ms: start_ms,
                end_timecode_ms: end_ms,
                speaker_name: Some(block.character_name.clone()),
                text: block.speech_text.clone(),
                language_code: spec.language_code.clone(),
            });

            current_time_ms = end_ms + 200; // 200ms gap between captions
        }

        SiraResult::Success(SubtitleTrack {
            track_id: spec.track_id.clone(),
            language_code: spec.language_code.clone(),
            segments,
        })
    }

    pub fn export_to_srt(&self, track: &SubtitleTrack) -> SiraResult<String> {
        let mut srt_out = String::new();

        for seg in &track.segments {
            let start_tc = format_srt_timecode(seg.start_timecode_ms);
            let end_tc = format_srt_timecode(seg.end_timecode_ms);
            let speaker_prefix = if let Some(speaker) = &seg.speaker_name {
                format!("{}: ", speaker)
            } else {
                "".to_string()
            };

            srt_out.push_str(&format!(
                "{}\n{} --> {}\n{}{}\n\n",
                seg.sequence_number, start_tc, end_tc, speaker_prefix, seg.text
            ));
        }

        SiraResult::Success(srt_out)
    }

    pub fn export_to_webvtt(&self, track: &SubtitleTrack) -> SiraResult<String> {
        let mut vtt_out = String::from("WEBVTT\n\n");

        for seg in &track.segments {
            let start_tc = format_webvtt_timecode(seg.start_timecode_ms);
            let end_tc = format_webvtt_timecode(seg.end_timecode_ms);
            let speaker_prefix = if let Some(speaker) = &seg.speaker_name {
                format!("<v {}>", speaker)
            } else {
                "".to_string()
            };

            vtt_out.push_str(&format!(
                "{}\n{} --> {}\n{}{}\n\n",
                seg.sequence_number, start_tc, end_tc, speaker_prefix, seg.text
            ));
        }

        SiraResult::Success(vtt_out)
    }

    pub fn validate_compliance(
        &self,
        track: &SubtitleTrack,
        spec: &SubtitleTrackSpec,
    ) -> SiraResult<SubtitleValidationReport> {
        let mut violations = Vec::new();
        let mut max_observed_cps = 0.0f32;
        let mut total_duration_ms = 0u64;

        for seg in &track.segments {
            let duration_sec = ((seg.end_timecode_ms - seg.start_timecode_ms) as f32) / 1000.0;
            total_duration_ms += (seg.end_timecode_ms - seg.start_timecode_ms) as u64;

            if duration_sec > 0.0 {
                let char_count = seg.text.chars().count();
                let cps = char_count as f32 / duration_sec;
                if cps > max_observed_cps {
                    max_observed_cps = cps;
                }

                if cps > spec.max_cps {
                    violations.push(format!(
                        "Caption #{} exceeds max CPS: {:.1} > {:.1}",
                        seg.sequence_number, cps, spec.max_cps
                    ));
                }
            }

            let lines: Vec<&str> = seg.text.lines().collect();
            if lines.len() > spec.max_lines_per_caption {
                violations.push(format!(
                    "Caption #{} exceeds max line count: {} > {}",
                    seg.sequence_number,
                    lines.len(),
                    spec.max_lines_per_caption
                ));
            }
        }

        SiraResult::Success(SubtitleValidationReport {
            is_compliant: violations.is_empty(),
            total_captions: track.segments.len(),
            total_duration_ms,
            max_observed_cps,
            violations,
        })
    }
}

fn format_srt_timecode(ms: u64) -> String {
    let hours = ms / 3600000;
    let minutes = (ms % 3600000) / 60000;
    let seconds = (ms % 60000) / 1000;
    let millis = ms % 1000;
    format!("{:02}:{:02}:{:02},{:03}", hours, minutes, seconds, millis)
}

fn format_webvtt_timecode(ms: u64) -> String {
    let hours = ms / 3600000;
    let minutes = (ms % 3600000) / 60000;
    let seconds = (ms % 60000) / 1000;
    let millis = ms % 1000;
    format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_27_subtitles_lifecycle_and_tamil() {
        let engine = SubtitleGeneratorEngine::new();
        let spec = SubtitleTrackSpec::default(); // ta-IN

        let dialogues = vec![
            DialogueBlock {
                character_name: "வேலன்".to_string(),
                speech_text: "சிறகுகள் பிலிம் ஸ்டுடியோவிற்கு உங்களை வரவேற்கிறோம்".to_string(),
                parenthetical: None,
            },
            DialogueBlock {
                character_name: "கயல்".to_string(),
                speech_text: "நன்றி, இந்த திரைப்படம் சிறப்பாக அமையும்".to_string(),
                parenthetical: None,
            },
        ];

        let track_res = engine.create_subtitle_track(&spec, &dialogues);
        assert!(matches!(track_res, SiraResult::Success(_)));

        if let SiraResult::Success(track) = track_res {
            assert_eq!(track.segments.len(), 2);
            assert_eq!(track.segments[0].speaker_name.as_deref(), Some("வேலன்"));
            assert_eq!(track.segments[0].language_code, "ta-IN");

            let srt_res = engine.export_to_srt(&track);
            if let SiraResult::Success(srt) = srt_res {
                assert!(srt.contains("1\n00:00:01,000 -->"));
                assert!(srt.contains("வேலன்: சிறகுகள் பிலிம் ஸ்டுடியோவிற்கு"));
            } else {
                panic!("export_to_srt failed");
            }

            let vtt_res = engine.export_to_webvtt(&track);
            if let SiraResult::Success(vtt) = vtt_res {
                assert!(vtt.starts_with("WEBVTT\n\n"));
                assert!(vtt.contains("00:00:01.000 -->"));
            } else {
                panic!("export_to_webvtt failed");
            }

            let report_res = engine.validate_compliance(&track, &spec);
            if let SiraResult::Success(report) = report_res {
                assert_eq!(report.total_captions, 2);
                assert!(report.is_compliant);
            } else {
                panic!("validate_compliance failed");
            }
        } else {
            panic!("create_subtitle_track failed");
        }
    }
}
