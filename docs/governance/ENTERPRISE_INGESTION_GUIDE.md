# ENTERPRISE UNIVERSAL MEDIA ASSET INGESTION & TRANSCODING GUIDE
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: APPROVED & PUBLISHED  
**Author**: AG (Chief Software Architect)  

---

## 1. Overview

This document defines camera RAW ingestion pipelines, automated 1080p editing proxy generation, SMPTE timecode extraction, and ACEScg color space conversion procedures for **Siragugal Film Studio**.

---

## 2. Universal Camera Format Support

- **Supported Inputs**: ARRIRAW, REDCODE RAW, Blackmagic RAW, Sony X-OCN, EXR 16-bit sequences, Apple ProRes 4444 XQ, MOV, MP4, WAV multi-track stems.
- **Automated Proxy Preset**: 1080p ProRes Proxy (`1920x1080`) for real-time timeline editing on low-end hardware.

---

## 3. ACES Color Transformation

- **Input Transform (IDT)**: Converts camera native color gamut (Arri Wide Gamut, S-Gamut3) to ACEScg (`AP1` primaries).
- **Working Space**: ACEScg linear floating-point color space for render engine compatibility.
