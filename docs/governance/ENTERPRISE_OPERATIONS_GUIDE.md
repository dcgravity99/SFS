# ENTERPRISE OPERATIONS & MONITORING GUIDE
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: APPROVED & PUBLISHED  
**Author**: AG (Chief Software Architect)  

---

## 1. Overview

This document defines runtime operations, health telemetry monitoring, structured log ingestion, and alert dispatch procedures for **Siragugal Film Studio**.

---

## 2. Runtime Telemetry Metrics

Sub-engines emit telemetry metrics through `sira-observability-engine`:

- **CPU Utilization**: Percentage load across core threads.
- **GPU Compute & VRAM Allocation**: Tracked in real-time (`RTX 4090`, limit `20.4 GB`).
- **Render Latency**: Frame-by-frame path tracing rendering latency in ms.

---

## 3. Structured JSON Logging Schema

All system logs are structured in JSON format:

```json
{
  "timestamp": "2026-08-04T09:00:00Z",
  "level": "INFO",
  "service": "sira_render_engine",
  "message": "Render Job job-01 frame 142/360 completed"
}
```

---

## 4. Alert Thresholds & Escalation

- **Critical Alert**: VRAM allocation `> 95%` or IPC connection drop.
- **Warning Alert**: Render frame latency `> 50 ms`.
