use colored::Colorize;

use crate::execution_state::{ExecutionState, ExecutionStatus};
use crate::steps::types::StepState;

pub fn execution_state(execution_state: &ExecutionState) {
    let status = execution_state.status();
    let status_text = format!("{:?}", status);
    let colored_status = match status {
        ExecutionStatus::New => status_text.white(),
        ExecutionStatus::Running => status_text.cyan(),
        ExecutionStatus::Finished => status_text.green(),
        ExecutionStatus::Error => status_text.yellow(),
        ExecutionStatus::Failed => status_text.red(),
    };

    println!("{} {}", "Execution Status:".bold(), colored_status.bold());
    println!("{}", "─".repeat(40));

    if execution_state.step_states.is_empty() {
        println!("  {}", "(no steps)".dimmed());
        return;
    }

    for (i, step) in execution_state.step_states.iter().enumerate() {
        let (icon, status_label) = match step {
            StepState::Ready(_) => ("○".white(), "Ready".white()),
            StepState::Running(_) => ("●".cyan(), "Running".cyan()),
            StepState::Completed { .. } => ("✔".green(), "Completed".green()),
            StepState::Error { .. } => ("⚠".yellow(), "Error".yellow()),
            StepState::Failed { .. } => ("✘".red(), "Failed".red()),
        };
        let kind_str = format!("{:?}", step.kind());
        println!("  {} Step {} [{}] ({}): {}", icon, i + 1, step.id().dimmed(), kind_str.dimmed(), status_label);
    }
}
