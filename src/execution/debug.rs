// Interactive debugging support for BDD test execution
use crate::execution::result::ScenarioResult;
use std::collections::HashMap;
use std::io::{self, Write};

/// Debugger state and configuration
#[derive(Debug, Clone)]
pub struct Debugger {
    pub enabled: bool,
    pub breakpoints: HashMap<String, bool>, // scenario_name -> is_enabled
    pub paused: bool,
    pub step_breakpoints: HashMap<String, bool>, // step_text -> is_enabled
    pub auto_step: bool,                         // Step through each step automatically
    pub current_scenario: String,
    pub current_step_index: usize,
}

impl Debugger {
    pub fn new() -> Self {
        Debugger {
            enabled: false,
            breakpoints: HashMap::new(),
            paused: false,
            step_breakpoints: HashMap::new(),
            auto_step: false,
            current_scenario: String::new(),
            current_step_index: 0,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        self.paused = false;
    }

    pub fn set_scenario_breakpoint(&mut self, scenario_name: &str, enabled: bool) {
        self.breakpoints.insert(scenario_name.to_string(), enabled);
    }

    pub fn set_step_breakpoint(&mut self, step_text: &str, enabled: bool) {
        self.step_breakpoints.insert(step_text.to_string(), enabled);
    }

    pub fn clear_breakpoints(&mut self) {
        self.breakpoints.clear();
        self.step_breakpoints.clear();
    }

    /// Check if we should pause at current location
    pub fn should_pause(&self, scenario_name: &str, step_text: &str) -> bool {
        if !self.enabled {
            return false;
        }

        // Check scenario breakpoint
        if let Some(enabled) = self.breakpoints.get(scenario_name) {
            if *enabled {
                return true;
            }
        }

        // Check step breakpoint
        if let Some(enabled) = self.step_breakpoints.get(step_text) {
            if *enabled {
                return true;
            }
        }

        false
    }

    /// Interactive debugger REPL
    pub fn repl(&mut self, scenario: &ScenarioResult, step_index: usize) -> DebugCommand {
        self.current_scenario = scenario.name.clone();
        self.current_step_index = step_index;
        self.paused = true;

        loop {
            println!("\n{}", "=".repeat(60));
            println!(
                "📍 Debugger - Scenario: '{}' - Step {}",
                scenario.name,
                step_index + 1
            );
            println!("{}", "=".repeat(60));

            // Show current step
            if step_index < scenario.steps.len() {
                let step = &scenario.steps[step_index];
                println!("Current Step: [{}] {}", step.keyword, step.text);
                println!("Status: {}", step.status);
            }

            // Show available commands
            self.print_help();

            // Get user input
            print!("\n(debugger) > ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                println!("Failed to read input");
                continue;
            }

            let input = input.trim();
            if input.is_empty() {
                continue;
            }

            match self.parse_command(input) {
                DebugCommand::Continue => {
                    self.paused = false;
                    return DebugCommand::Continue;
                }
                DebugCommand::Step => {
                    return DebugCommand::Step;
                }
                DebugCommand::Repeat => {
                    return DebugCommand::Repeat;
                }
                DebugCommand::Skip => {
                    return DebugCommand::Skip;
                }
                DebugCommand::Help => {
                    self.print_help();
                }
                DebugCommand::Info => {
                    self.print_info(scenario, step_index);
                }
                DebugCommand::Breakpoints => {
                    self.print_breakpoints();
                }
                DebugCommand::SetBreakpoint(name) => {
                    self.set_scenario_breakpoint(&name, true);
                    println!("✓ Breakpoint set for scenario: {}", name);
                }
                DebugCommand::ClearBreakpoint(name) => {
                    self.set_scenario_breakpoint(&name, false);
                    println!("✓ Breakpoint cleared for scenario: {}", name);
                }
                DebugCommand::Quit => {
                    self.paused = false;
                    return DebugCommand::Quit;
                }
                DebugCommand::Unknown => {
                    println!("Unknown command. Type 'help' for available commands.");
                }
            }
        }
    }

    fn parse_command(&self, input: &str) -> DebugCommand {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return DebugCommand::Unknown;
        }

        match parts[0] {
            "c" | "continue" => DebugCommand::Continue,
            "n" | "next" | "step" => DebugCommand::Step,
            "r" | "repeat" => DebugCommand::Repeat,
            "s" | "skip" => DebugCommand::Skip,
            "h" | "help" => DebugCommand::Help,
            "i" | "info" => DebugCommand::Info,
            "b" | "breakpoints" => DebugCommand::Breakpoints,
            "break" => {
                if parts.len() > 1 {
                    DebugCommand::SetBreakpoint(parts[1..].join(" "))
                } else {
                    DebugCommand::Unknown
                }
            }
            "clear" => {
                if parts.len() > 1 {
                    DebugCommand::ClearBreakpoint(parts[1..].join(" "))
                } else {
                    DebugCommand::Unknown
                }
            }
            "q" | "quit" => DebugCommand::Quit,
            _ => DebugCommand::Unknown,
        }
    }

    fn print_help(&self) {
        println!("\nDebugger Commands:");
        println!("  c, continue    - Continue execution until next breakpoint");
        println!("  n, next, step  - Execute current step and pause");
        println!("  r, repeat      - Repeat current step");
        println!("  s, skip        - Skip current step");
        println!("  i, info        - Show current step information");
        println!("  b, breakpoints - List all breakpoints");
        println!("  break <name>   - Set breakpoint for scenario");
        println!("  clear <name>   - Clear breakpoint for scenario");
        println!("  h, help        - Show this help message");
        println!("  q, quit        - Quit debugger and stop execution");
    }

    fn print_info(&self, scenario: &ScenarioResult, step_index: usize) {
        println!("\n--- Step Information ---");
        if step_index < scenario.steps.len() {
            let step = &scenario.steps[step_index];
            println!("Keyword: {}", step.keyword);
            println!("Text: {}", step.text);
            println!("Status: {}", step.status);
            println!("Duration: {}ms", step.duration_ms);
            if let Some(output) = &step.output {
                println!("Output: {}", output);
            }
            if let Some(error) = &step.error {
                println!("Error: {} - {}", error.code, error.message);
                if !error.suggestions.is_empty() {
                    println!("Suggestions:");
                    for suggestion in &error.suggestions {
                        println!("  - {}", suggestion);
                    }
                }
            }
        }
        println!(
            "--- Scenario: {} ({}/{} steps)",
            scenario.name,
            step_index + 1,
            scenario.steps.len()
        );
    }

    fn print_breakpoints(&self) {
        println!("\n--- Breakpoints ---");
        if self.breakpoints.is_empty() && self.step_breakpoints.is_empty() {
            println!("No breakpoints set");
        } else {
            println!("Scenario Breakpoints:");
            for (name, enabled) in &self.breakpoints {
                println!("  {} - {}", if *enabled { "✓" } else { "✗" }, name);
            }
            println!("Step Breakpoints:");
            for (text, enabled) in &self.step_breakpoints {
                println!("  {} - {}", if *enabled { "✓" } else { "✗" }, text);
            }
        }
    }
}

/// Debugger commands
#[derive(Debug, Clone)]
pub enum DebugCommand {
    Continue,
    Step,
    Repeat,
    Skip,
    Help,
    Info,
    Breakpoints,
    SetBreakpoint(String),
    ClearBreakpoint(String),
    Quit,
    Unknown,
}

/// Execution context snapshot
#[derive(Debug, Clone)]
pub struct ExecutionSnapshot {
    pub scenario_name: String,
    pub step_index: usize,
    pub step_text: String,
    pub step_status: String,
    pub step_output: Option<String>,
    pub timestamp: String,
}

impl ExecutionSnapshot {
    pub fn from_scenario_step(scenario: &ScenarioResult, step_index: usize) -> Option<Self> {
        if step_index >= scenario.steps.len() {
            return None;
        }

        let step = &scenario.steps[step_index];
        Some(ExecutionSnapshot {
            scenario_name: scenario.name.clone(),
            step_index,
            step_text: step.text.clone(),
            step_status: step.status.clone(),
            step_output: step.output.clone(),
            timestamp: chrono::Local::now().to_rfc3339(),
        })
    }
}

/// Execution state tracker
#[derive(Debug, Clone)]
pub struct ExecutionState {
    pub snapshots: Vec<ExecutionSnapshot>,
    pub variables: HashMap<String, String>,
    pub current_snapshot: Option<ExecutionSnapshot>,
}

impl ExecutionState {
    pub fn new() -> Self {
        ExecutionState {
            snapshots: Vec::new(),
            variables: HashMap::new(),
            current_snapshot: None,
        }
    }

    pub fn add_snapshot(&mut self, snapshot: ExecutionSnapshot) {
        self.snapshots.push(snapshot.clone());
        self.current_snapshot = Some(snapshot);
    }

    pub fn set_variable(&mut self, name: &str, value: &str) {
        self.variables.insert(name.to_string(), value.to_string());
    }

    pub fn get_variable(&self, name: &str) -> Option<&str> {
        self.variables.get(name).map(|v| v.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debugger_creation() {
        let debugger = Debugger::new();
        assert!(!debugger.enabled);
        assert!(!debugger.paused);
        assert!(debugger.breakpoints.is_empty());
    }

    #[test]
    fn test_debugger_enable_disable() {
        let mut debugger = Debugger::new();
        assert!(!debugger.enabled);

        debugger.enable();
        assert!(debugger.enabled);

        debugger.disable();
        assert!(!debugger.enabled);
    }

    #[test]
    fn test_set_breakpoint() {
        let mut debugger = Debugger::new();
        debugger.set_scenario_breakpoint("Test Scenario", true);

        assert!(debugger.breakpoints.contains_key("Test Scenario"));
        assert_eq!(debugger.breakpoints.get("Test Scenario"), Some(&true));
    }

    #[test]
    fn test_should_pause_at_breakpoint() {
        let mut debugger = Debugger::new();
        debugger.enable();
        debugger.set_scenario_breakpoint("Test Scenario", true);

        assert!(debugger.should_pause("Test Scenario", "I click button"));
    }

    #[test]
    fn test_should_not_pause_without_breakpoint() {
        let mut debugger = Debugger::new();
        debugger.enable();

        assert!(!debugger.should_pause("Test Scenario", "I click button"));
    }

    #[test]
    fn test_parse_commands() {
        let debugger = Debugger::new();

        assert!(matches!(
            debugger.parse_command("c"),
            DebugCommand::Continue
        ));
        assert!(matches!(
            debugger.parse_command("continue"),
            DebugCommand::Continue
        ));
        assert!(matches!(debugger.parse_command("n"), DebugCommand::Step));
        assert!(matches!(debugger.parse_command("step"), DebugCommand::Step));
        assert!(matches!(debugger.parse_command("r"), DebugCommand::Repeat));
        assert!(matches!(debugger.parse_command("s"), DebugCommand::Skip));
        assert!(matches!(debugger.parse_command("h"), DebugCommand::Help));
        assert!(matches!(debugger.parse_command("q"), DebugCommand::Quit));
    }

    #[test]
    fn test_execution_state() {
        let mut state = ExecutionState::new();
        state.set_variable("test_var", "test_value");

        assert_eq!(state.get_variable("test_var"), Some("test_value"));
        assert_eq!(state.get_variable("nonexistent"), None);
    }

    #[test]
    fn test_clear_breakpoints() {
        let mut debugger = Debugger::new();
        debugger.set_scenario_breakpoint("Test 1", true);
        debugger.set_scenario_breakpoint("Test 2", true);
        debugger.set_step_breakpoint("Step 1", true);

        assert!(!debugger.breakpoints.is_empty());
        debugger.clear_breakpoints();
        assert!(debugger.breakpoints.is_empty());
        assert!(debugger.step_breakpoints.is_empty());
    }
}
