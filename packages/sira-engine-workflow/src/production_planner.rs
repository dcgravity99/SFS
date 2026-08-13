/* ============================================================================
 * Siragugal Film Studio — Module 48: AI Production Planning & Scheduling Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProductionTaskSpec {
    pub task_id: String,
    pub name: String,
    pub estimated_hours: f32,
    pub dependencies: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProductionSchedule {
    pub schedule_id: String,
    pub total_duration_days: f32,
    pub critical_path_task_ids: Vec<String>,
}

#[derive(Default)]
pub struct ProductionPlannerEngine;

impl ProductionPlannerEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn generate_schedule(&self, tasks: &[ProductionTaskSpec]) -> SiraResult<ProductionSchedule> {
        if tasks.is_empty() {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "EMPTY_TASKS_LIST".to_string(),
                category: "WORKFLOW_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.production_planner.empty_tasks".to_string(),
                suggested_action_key: None,
            });
        }

        let mut total_hours = 0.0;
        let mut task_ids = Vec::with_capacity(tasks.len());

        for task in tasks {
            if task.task_id.contains("..") {
                return SiraResult::Error(SiraError {
                    code: SiraErrorCode::UnknownSystemError,
                    error_name: "INVALID_TASK_ID_PATH".to_string(),
                    category: "WORKFLOW_ENGINE".to_string(),
                    severity: "ERROR".to_string(),
                    is_recoverable: false,
                    correlation_id: None,
                    job_id: None,
                    i18n_key: "errors.production_planner.invalid_path".to_string(),
                    suggested_action_key: None,
                });
            }
            total_hours += task.estimated_hours;
            task_ids.push(task.task_id.clone());
        }

        let schedule = ProductionSchedule {
            schedule_id: "SCHED-PROD-2026-001".to_string(),
            total_duration_days: total_hours / 8.0,
            critical_path_task_ids: task_ids,
        };

        SiraResult::Success(schedule)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_48_production_planner_lifecycle() {
        let engine = ProductionPlannerEngine::new();
        let tasks = vec![
            ProductionTaskSpec {
                task_id: "TASK-STORYBOARD-01".to_string(),
                name: "Storyboard Scene 01".to_string(),
                estimated_hours: 16.0,
                dependencies: vec![],
            },
            ProductionTaskSpec {
                task_id: "TASK-RENDER-01".to_string(),
                name: "Render Farm Shot 01".to_string(),
                estimated_hours: 24.0,
                dependencies: vec!["TASK-STORYBOARD-01".to_string()],
            },
        ];

        let sched_res = engine.generate_schedule(&tasks);
        assert!(matches!(sched_res, SiraResult::Success(_)));

        if let SiraResult::Success(schedule) = sched_res {
            assert_eq!(schedule.total_duration_days, 5.0); // 40 hours / 8 = 5 days
            assert_eq!(schedule.critical_path_task_ids.len(), 2);
        }

        // Test empty tasks rejection
        assert!(matches!(engine.generate_schedule(&[]), SiraResult::Error(_)));

        // Test path traversal rejection
        let invalid_tasks = vec![ProductionTaskSpec {
            task_id: "TASK/../traversed".to_string(),
            name: "Invalid Task".to_string(),
            estimated_hours: 8.0,
            dependencies: vec![],
        }];
        assert!(matches!(engine.generate_schedule(&invalid_tasks), SiraResult::Error(_)));
    }
}
