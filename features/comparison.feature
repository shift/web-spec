Feature: Result Comparison
    As a QA engineer
    I want to compare test execution results
    So that I can detect regressions and improvements

    Background:
        Given the web-spec CLI is available
        And I have sample execution result files

    @comparison @identical
    Scenario: Comparing identical results
        Given baseline result "baseline.json" with content:
            """
            {
                "status": "passed",
                "timestamp": "2024-02-04T10:00:00Z",
                "duration_ms": 1000,
                "feature": {"name": "Test", "file": "test.feature", "description": null},
                "scenarios": [],
                "summary": {"total_scenarios": 5, "passed_scenarios": 5, "failed_scenarios": 0, "skipped_scenarios": 0, "total_steps": 20, "passed_steps": 20, "failed_steps": 0, "skipped_steps": 0}
            }
            """
        And current result "current.json" with content:
            """
            {
                "status": "passed",
                "timestamp": "2024-02-04T11:00:00Z",
                "duration_ms": 1000,
                "feature": {"name": "Test", "file": "test.feature", "description": null},
                "scenarios": [],
                "summary": {"total_scenarios": 5, "passed_scenarios": 5, "failed_scenarios": 0, "skipped_scenarios": 0, "total_steps": 20, "passed_steps": 20, "failed_steps": 0, "skipped_steps": 0}
            }
            """
        When I run comparison with baseline "baseline.json" and current "current.json"
        Then the exit code should be 0
        And the output should contain "IDENTICAL"
        And the output should contain "Regressions Detected: 0"

    @comparison @regression
    Scenario: Detecting performance regression
        Given baseline result "baseline_perf.json" with content:
            """
            {
                "status": "passed",
                "timestamp": "2024-02-04T10:00:00Z",
                "duration_ms": 1000,
                "feature": {"name": "Performance Test", "file": "perf.feature", "description": null},
                "scenarios": [],
                "summary": {"total_scenarios": 5, "passed_scenarios": 5, "failed_scenarios": 0, "skipped_scenarios": 0, "total_steps": 20, "passed_steps": 20, "failed_steps": 0, "skipped_steps": 0}
            }
            """
        And current result "current_perf.json" with content:
            """
            {
                "status": "failed",
                "timestamp": "2024-02-04T11:00:00Z",
                "duration_ms": 2000,
                "feature": {"name": "Performance Test", "file": "perf.feature", "description": null},
                "scenarios": [],
                "summary": {"total_scenarios": 5, "passed_scenarios": 4, "failed_scenarios": 1, "skipped_scenarios": 0, "total_steps": 20, "passed_steps": 16, "failed_steps": 4, "skipped_steps": 0}
            }
            """
        When I run comparison with baseline "baseline_perf.json" and current "current_perf.json"
        Then the exit code should be 0
        And the output should contain "REGRESSION"
        And the output should contain "Regressions Detected:"

    @comparison @improvement
    Scenario: Detecting performance improvement
        Given baseline result "baseline_improve.json" with content:
            """
            {
                "status": "passed",
                "timestamp": "2024-02-04T10:00:00Z",
                "duration_ms": 2000,
                "feature": {"name": "Speed Test", "file": "speed.feature", "description": null},
                "scenarios": [],
                "summary": {"total_scenarios": 5, "passed_scenarios": 5, "failed_scenarios": 0, "skipped_scenarios": 0, "total_steps": 20, "passed_steps": 20, "failed_steps": 0, "skipped_steps": 0}
            }
            """
        And current result "current_improve.json" with content:
            """
            {
                "status": "passed",
                "timestamp": "2024-02-04T11:00:00Z",
                "duration_ms": 1000,
                "feature": {"name": "Speed Test", "file": "speed.feature", "description": null},
                "scenarios": [],
                "summary": {"total_scenarios": 5, "passed_scenarios": 5, "failed_scenarios": 0, "skipped_scenarios": 0, "total_steps": 20, "passed_steps": 20, "failed_steps": 0, "skipped_steps": 0}
            }
            """
        When I run comparison with baseline "baseline_improve.json" and current "current_improve.json"
        Then the exit code should be 0
        And the output should contain "IMPROVEMENT"
        And the output should contain "Improvements Detected:"

    @comparison @json_output
    Scenario: Comparing results with JSON output format
        Given two identical execution result files
        When I run comparison with format "json"
        Then the output should contain "{"
        And the output should contain "comparison"

    @comparison @yaml_output
    Scenario: Comparing results with YAML output format
        Given two identical execution result files
        When I run comparison with format "yaml"
        Then the output should contain ":"
        And the output should contain "comparison"

    @comparison @missing_file
    Scenario: Handling missing baseline file
        When I run comparison with missing baseline file
        Then the exit code should not be 0
        And the output should indicate file not found
