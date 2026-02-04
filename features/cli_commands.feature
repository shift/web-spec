Feature: CLI Command Execution
    As a test engineer
    I want to execute CLI commands
    So that I can validate all web-spec functionality

    Background:
        Given the web-spec CLI is available
        And I have a temporary directory for test output

    @cli @run
    Scenario: Running a valid feature file
        Given a valid feature file "test_login.feature" with content:
            """
            Feature: User Login
                As a registered user
                I want to log into the system
                So that I can access my account

                Scenario: Successful login with valid credentials
                    Given I extract the page title
                    When I extract all links from the page
                    Then I should have at least 1 link
            """
        When I run the command "run --feature test_login.feature"
        Then the exit code should be 0
        And the output should contain "Feature: User Login"
        And the output should contain "Scenario:"

    @cli @validate
    Scenario: Validating a feature file
        Given a valid feature file "valid.feature" with content:
            """
            Feature: Valid Feature
                Scenario: Test scenario
                    Given I extract the page title
                    When I extract all links
                    Then I should have extracted data
            """
        When I run the command "validate --feature valid.feature"
        Then the exit code should be 0
        And the output should contain "valid.feature"

    @cli @list_steps
    Scenario: Listing available steps
        When I run the command "list-steps"
        Then the exit code should be 0
        And the output should contain "step"

    @cli @search_steps
    Scenario: Searching for steps
        When I run the command "search-steps extract"
        Then the exit code should be 0
        And the output should contain "extract"

    @cli @version
    Scenario: Checking version information
        When I run the command "version"
        Then the exit code should be 0
        And the output should contain "web-spec"

    @cli @help
    Scenario: Displaying help information
        When I run the command "--help"
        Then the exit code should be 0
        And the output should contain "Commands:"

    @cli @invalid_file
    Scenario: Handling non-existent feature file
        When I run the command "run --feature nonexistent.feature"
        Then the exit code should not be 0

    @cli @output_formats
    Scenario: Testing different output formats
        Given a valid feature file "output_test.feature" with content:
            """
            Feature: Output Test
                Scenario: Test output formats
                    Given I extract the page title
            """
        When I run with format "json"
        Then the output should contain "{"
