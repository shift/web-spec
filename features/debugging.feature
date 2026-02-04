Feature: Interactive Debugging
    As a developer
    I want to debug test execution step by step
    So that I can identify and fix issues

    Background:
        Given the web-spec CLI is available
        And I have a feature file to debug

    @debug @breakpoint
    Scenario: Setting a breakpoint at scenario
        Given a feature file with multiple scenarios:
            """
            Feature: Debug Test
                Scenario: First scenario
                    Given I extract the page title
                Scenario: Second scenario
                    Given I extract all links
                Scenario: Third scenario
                    Given I extract page metadata
            """
        When I set a breakpoint at "Second scenario"
        And I start debugging
        Then execution should pause at the breakpoint

    @debug @continue
    Scenario: Continuing execution after breakpoint
        Given debugging is paused at a breakpoint
        When I use the "continue" command
        Then execution should resume until next breakpoint

    @debug @step
    Scenario: Stepping through execution
        Given debugging is paused
        When I use the "step" command
        Then only the current step should execute
        And execution should pause after the step

    @debug @skip
    Scenario: Skipping a step
        Given debugging is paused at a step
        When I use the "skip" command
        Then the current step should be skipped
        And execution should continue

    @debug @repeat
    Scenario: Repeating a step
        Given debugging is paused at a step
        When I use the "repeat" command
        Then the current step should execute again
        And execution should pause after repetition

    @debug @info
    Scenario: Getting step information
        Given debugging is paused at a step
        When I use the "info" command
        Then it should show current step details
        And it should show execution state

    @debug @breakpoints_list
    Scenario: Listing all breakpoints
        Given I have set multiple breakpoints
        When I use the "breakpoints" command
        Then it should list all set breakpoints

    @debug @quit
    Scenario: Quitting debugging session
        Given debugging is in progress
        When I use the "quit" command
        Then the debugging session should end
        And execution should stop

    @debug @help
    Scenario: Getting debug help
        Given debugging is active
        When I use the "help" command
        Then it should show available debug commands

    @debug @auto_step
    Scenario: Auto-stepping through all steps
        Given I start debugging with auto-step enabled
        When execution begins
        Then it should automatically step through each step
        And it should pause after each step
