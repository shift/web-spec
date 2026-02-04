Feature: Performance Alerts
    As a performance engineer
    I want to configure performance thresholds
    So that I can detect performance regressions

    Background:
        Given the web-spec CLI is available

    @alerts @configuration
    Scenario: Loading alert configuration
        Given an alert configuration file "alerts.yml" with:
            """
            - name: performance-monitoring
              enabled: true
              thresholds:
                - name: slow_scenario
                  metric: ScenarioDurationMs
                  operator: GreaterThan
                  value: 30000
                  severity: Warning
            """
        When I load the alert configuration
        Then the exit code should be 0
        And the configuration should be valid

    @alerts @threshold_scenario
    Scenario: Configuring scenario duration threshold
        Given a threshold with:
            | metric              | operator     | value   |
            | ScenarioDurationMs | GreaterThan | 30000   |
        When I evaluate the threshold
        Then it should detect scenarios exceeding 30 seconds

    @alerts @threshold_failure_rate
    Scenario: Configuring failure rate threshold
        Given a threshold with:
            | metric               | operator     | value |
            | FailureRatePercent | GreaterThan | 10.0  |
        When I evaluate the threshold
        Then it should detect failure rates above 10%

    @alerts @threshold_throughput
    Scenario: Configuring throughput threshold
        Given a threshold with:
            | metric              | operator  | value |
            | ScenariosPerSecond | LessThan | 1.0   |
        When I evaluate the threshold
        Then it should detect low throughput scenarios

    @alerts @severity_levels
    Scenario: Testing severity level classification
        Given thresholds with different severity levels:
            | name              | severity  |
            | warning_threshold | Warning   |
            | critical_threshold| Critical  |
        When I evaluate thresholds
        Then warnings should be classified as Warning
        And critical issues should be classified as Critical

    @alerts @output_format
    Scenario: Testing alert output formats
        Given alerts are triggered
        When I output in "json" format
        Then the output should be valid JSON
        When I output in "yaml" format
        Then the output should be valid YAML

    @alerts @enabled_disabled
    Scenario: Enabling and disabling alerts
        Given an alert configuration
        When I enable alerts
        Then monitoring should be active
        When I disable alerts
        Then monitoring should be inactive

    @alerts @custom_metrics
    Scenario: Using custom performance metrics
        Given a custom metric definition
        When I record custom metric data
        Then the threshold evaluation should use custom metrics
