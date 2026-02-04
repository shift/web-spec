Feature: Webhook Notifications
    As a DevOps engineer
    I want to send notifications via webhooks
    So that I can integrate with external systems

    Background:
        Given the web-spec CLI is available

    @webhook @slack
    Scenario: Sending Slack notification
        Given a webhook configuration file "slack_webhook.yml" with:
            """
            - url: https://hooks.slack.com/services/test
              name: test-slack
              events: [completion, failure]
              retry_count: 1
            """
        When I test the webhook configuration
        Then the exit code should be 0
        And the output should contain "Webhook"

    @webhook @discord
    Scenario: Sending Discord notification
        Given a webhook configuration file "discord_webhook.yml" with:
            """
            - url: https://discord.com/api/webhooks/test
              name: test-discord
              events: [completion]
              retry_count: 1
            """
        When I test the webhook configuration
        Then the exit code should be 0
        And the output should contain "Webhook"

    @webhook @teams
    Scenario: Sending Microsoft Teams notification
        Given a webhook configuration file "teams_webhook.yml" with:
            """
            - url: https://outlook.office.com/webhook/test
              name: test-teams
              events: [completion, failure]
              retry_count: 1
            """
        When I test the webhook configuration
        Then the exit code should be 0
        And the output should contain "Webhook"

    @webhook @events
    Scenario: Testing different webhook event types
        Given a webhook configuration with events "completion"
        When I test with event "completion"
        Then it should handle the event type
        Given a webhook configuration with events "failure"
        When I test with event "failure"
        Then it should handle the event type

    @webhook @retry
    Scenario: Testing webhook retry logic
        Given a webhook configuration with retry_count: 3
        When I send a notification
        Then it should attempt up to 3 retries

    @webhook @validation
    Scenario: Validating webhook configuration
        Given an invalid webhook URL
        When I test the configuration
        Then the output should indicate the validation error

    @webhook @headers
    Scenario: Testing webhook with custom headers
        Given a webhook configuration with custom headers:
            """
            - url: https://example.com/webhook
              name: custom-headers
              headers:
                Authorization: Bearer test-token
                Content-Type: application/json
            """
        When I test the webhook configuration
        Then the exit code should be 0
