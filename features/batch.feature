Feature: Batch Execution
    As a test engineer
    I want to execute multiple features in batch
    So that I can run comprehensive test suites

    Background:
        Given the web-spec CLI is available
        And I have a directory with multiple feature files

    @batch @parallel
    Scenario: Running features in parallel
        Given I have 3 feature files in the directory
        When I run batch execution in parallel mode
        Then the exit code should be 0
        And the output should contain "Batch Execution Summary"
        And features should execute concurrently

    @batch @sequential
    Scenario: Running features sequentially
        Given I have 2 feature files in the directory
        When I run batch execution in sequential mode
        Then the exit code should be 0
        And features should execute one after another

    @batch @progress
    Scenario: Tracking batch execution progress
        Given I have multiple feature files
        When I run batch execution
        Then the output should indicate progress
        And the output should show number of features processed

    @batch @summary
    Scenario: Generating batch execution summary
        Given I execute a batch of features
        When the execution completes
        Then the summary should include:
            | metric              |
            | total_features     |
            | passed_features    |
            | failed_features    |
            | total_duration_ms |

    @batch @json_output
    Scenario: Batch execution with JSON output
        Given I have feature files to execute
        When I run batch with output format "json"
        Then the output should be valid JSON
        And it should contain batch summary

    @batch @yaml_output
    Scenario: Batch execution with YAML output
        Given I have feature files to execute
        When I run batch with output format "yaml"
        Then the output should be valid YAML

    @batch @workers
    Scenario: Controlling number of workers
        Given I have multiple feature files
        When I set max workers to 2
        Then batch execution should use at most 2 parallel workers

    @batch @continue_on_failure
    Scenario: Continuing on feature failure
        Given I have multiple feature files with one failing
        When I enable continue-on-failure
        Then execution should continue with remaining features

    @batch @directory_scan
    Scenario: Discovering features in directory
        Given a directory structure:
            """
            /features
                /subdir
                    feature1.feature
                feature2.feature
            """
        When I run batch execution on the directory
        Then it should discover all .feature files
        And it should include files from subdirectories

    @batch @error_handling
    Scenario: Handling errors in batch
        Given I have a mix of valid and invalid feature files
        When I run batch execution
        Then valid files should execute
        And errors should be reported in the summary
