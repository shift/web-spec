Getting Started with web-spec
===================================

web-spec is a powerful Rust-based BDD testing framework for automating web browser interactions using Gherkin feature files. This guide will help you get started in just a few minutes.

Table of Contents
-----------------

1. Quick Start
2. Installation and Setup
3. Writing Your First Feature File
4. Running Tests
5. Understanding Results
6. Common Step Patterns
7. Real-World Examples
8. Troubleshooting
9. Next Steps

Quick Start (5 minutes)
----------------------

Here's the fastest way to get a test running:

1. **Install Dependencies**
   ```bash
   # Ensure you have Rust installed
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Navigate to web-spec directory
   cd web-spec
   ```

2. **Build the Project**
   ```bash
   nix develop -c -- cargo build --features chromiumoxide-backend
   ```

3. **Run Your First Test**
   ```bash
   nix develop -c -- cargo run --features chromiumoxide-backend -- run --feature examples/features/navigation.feature
   ```

That's it! You should see the test results in your terminal.

Installation and Setup
----------------------

### Prerequisites

- Rust 1.70 or later
- A Unix-like operating system (Linux, macOS)
- 2GB+ free disk space for dependencies

### Full Setup

1. **Clone or navigate to the repository**
   ```bash
   cd /path/to/web-spec
   ```

2. **Use the development environment**
   ```bash
   nix develop
   ```
   This loads the development shell with all required tools.

3. **Build the project**
   ```bash
   cargo build --features chromiumoxide-backend
   ```

4. **Verify installation**
   ```bash
   cargo run -- list-steps
   ```
   This should display all available step patterns. If you see 174+ steps, you're ready to go!

Writing Your First Feature File
--------------------------------

Feature files use the Gherkin language - a simple, human-readable syntax for describing test scenarios.

### Basic Structure

Every feature file has this structure:

```gherkin
Feature: Brief description of what you're testing
  Longer description explaining why this feature matters

  Scenario: What should happen in this specific case
    Given I have some precondition
    When I perform an action
    Then I expect a specific outcome
```

### Complete Example

Create a file `login-test.feature`:

```gherkin
Feature: User Login
  As a user, I want to log in to the application
  So that I can access my account

  Scenario: Successful login
    Given I navigate to "https://example.com/login"
    Then the page title should contain "Login"
    When I type into the username field "user@example.com"
    And I type into the password field "mypassword"
    And I click the button "Sign In"
    Then the page title should contain "Dashboard"
```

### Key Parts Explained

1. **Given** - Sets up the precondition (usually navigating to a page)
2. **When** - Describes the action being performed
3. **Then** - Describes what should happen after the action

### Available Elements

Steps use CSS selectors or element names. Examples:

- By ID: `#login-button`
- By class: `.submit-btn`
- By tag: `button`
- By name: `[name="username"]`
- By text: Just use the visible text like `"Sign In"`

Running Tests
-----------

### Run All Tests in a Feature File

```bash
cargo run --features chromiumoxide-backend -- run --feature your-feature.feature
```

### Run Tests with JSON Output

```bash
cargo run --features chromiumoxide-backend -- run --feature your-feature.feature --format json
```

### Pretty Print Results

```bash
cargo run --features chromiumoxide-backend -- run --feature your-feature.feature --pretty
```

### Save Results to File

```bash
cargo run --features chromiumoxide-backend -- run --feature your-feature.feature -o results.txt
```

### Validate Before Running

Always validate your feature file first:

```bash
cargo run --features chromiumoxide-backend -- validate --feature your-feature.feature
```

If validation passes with no errors, your feature file is syntactically correct.

Understanding Results
--------------------

### Text Output Example

```
Running feature: Login Test
Scenario: Successful login
  ✓ Given I navigate to "https://example.com/login"
  ✓ When I type into the username field "user@example.com"
  ✓ When I type into the password field "mypassword"
  ✓ When I click the button "Sign In"
  ✓ Then the page title should contain "Dashboard"

Feature completed: 5 steps passed
```

### Interpreting Results

- **✓** = Step passed successfully
- **✗** = Step failed (see error message for details)
- **Gray text** = Step was skipped or not executed

### JSON Output Example

```json
{
  "status": "passed",
  "scenarios": [
    {
      "name": "Successful login",
      "steps": [
        {
          "step": "Given I navigate to \"https://example.com/login\"",
          "status": "passed",
          "duration_ms": 1234
        }
      ],
      "total_steps": 5,
      "passed_steps": 5
    }
  ]
}
```

Common Step Patterns
-------------------

### Navigation

```gherkin
Given I navigate to "https://example.com"
Then the current URL should be "https://example.com/"
And the current path should be "/"
And the page title should be "Example"
```

### Form Input

```gherkin
When I type into the username field "john@example.com"
And I type into the password field "secure123"
And I click the submit button
And I select the "Admin" option from the role dropdown
And I check the "Remember me" checkbox
```

### Verification

```gherkin
Then the page should contain the text "Welcome"
And the success message should be visible
And the error message should not be visible
And the submit button should be enabled
And the results should contain exactly "10" items
```

### Waiting

```gherkin
When I click the "Load Data" button
And I wait for the "Loading..." element to disappear
Then the data should be visible
```

### Scrolling

```gherkin
When I scroll to the "Footer" element
Or I scroll down to "50" percent
Then the footer section should be visible
```

Real-World Examples
-------------------

The `examples/features/` directory contains several complete examples:

### 1. navigation.feature
Demonstrates basic page navigation and URL verification.

```bash
cargo run --features chromiumoxide-backend -- run --feature examples/features/navigation.feature
```

### 2. form-interaction.feature
Shows how to fill forms, select options, and upload files.

```bash
cargo run --features chromiumoxide-backend -- run --feature examples/features/form-interaction.feature
```

### 3. verification.feature
Examples of verifying elements, text, attributes, and states.

```bash
cargo run --features chromiumoxide-backend -- run --feature examples/features/verification.feature
```

### 4. interactions.feature
Demonstrates clicks, hovering, drag-and-drop, and context menus.

```bash
cargo run --features chromiumoxide-backend -- run --feature examples/features/interactions.feature
```

### 5. waiting.feature
Shows how to handle dynamic content and loading states.

```bash
cargo run --features chromiumoxide-backend -- run --feature examples/features/waiting.feature
```

### 6. scrolling.feature
Demonstrates various scrolling techniques.

```bash
cargo run --features chromiumoxide-backend -- run --feature examples/features/scrolling.feature
```

### 7. ecommerce.feature
A complete e-commerce workflow example.

```bash
cargo run --features chromiumoxide-backend -- run --feature examples/features/ecommerce.feature
```

Troubleshooting
---------------

### Issue: "Step not found" error

**Cause**: Your step pattern doesn't match any registered steps.

**Solution**: Use the search command to find the correct pattern:
```bash
cargo run --features chromiumoxide-backend -- search-steps "click"
```

This will show all steps containing "click":
```
[Interaction] click - Click on an element
[Interaction] click_button - Click a button element
[Interaction] click_link - Click a link element
```

### Issue: Feature file fails validation

**Cause**: Syntax errors in the feature file.

**Solution**: Check the validation output:
```bash
cargo run --features chromiumoxide-backend -- validate --feature your-file.feature
```

Common issues:
- Missing colons (`:`) after Feature/Scenario
- Inconsistent indentation
- Invalid step keywords (use `Given`, `When`, `Then`, `And`, `But`)

### Issue: Test times out or hangs

**Cause**: Element not found or page not loading.

**Solution**:
1. Use `wait_for_element_text` before verifying element content
2. Increase wait times if dealing with slow pages
3. Verify the CSS selector or element name is correct

### Issue: Cannot run nix develop

**Cause**: Nix not installed or configuration issue.

**Solution**:
```bash
# Install nix if not present
curl --proto '=https' --tlsv1.2 -sSf https://install.determinate.systems/nix | sh

# Or use direnv to auto-activate devShell
direnv allow
```

Next Steps
-----------

### 1. Explore All Available Steps

```bash
cargo run --features chromiumoxide-backend -- list-steps
```

Spend a few minutes looking at what's available. You have 174+ steps covering:
- Navigation and page interaction
- Form filling and selection
- Element verification and assertions
- Waiting and timing
- Scrolling and positioning
- File uploads
- And much more!

### 2. Read the Full CLI Documentation

See `docs/CLI.md` for detailed information about:
- All command options
- Output formats and formatting
- Exit codes and error handling
- Performance tips

### 3. Write Tests for Your Own Application

1. Identify key user flows
2. Write scenarios in Gherkin
3. Use `validate --feature` to check syntax
4. Run `run --feature` to execute
5. Iterate based on results

### 4. Integrate with CI/CD

web-spec works great in CI/CD pipelines:

```yaml
# Example GitHub Actions workflow
- name: Run web-spec tests
  run: |
    cargo run --features chromiumoxide-backend -- run --feature features/ --format json -o test-results.json
```

### 5. Check Exit Codes

Use exit codes to integrate with scripts:

```bash
cargo run --features chromiumoxide-backend -- run --feature test.feature
echo $?  # 0 = success, 1 = failure
```

Tips for Writing Good Tests
---------------------------

### 1. Keep Scenarios Focused
Each scenario should test one specific behavior.

Bad:
```gherkin
Scenario: Everything
  Given I navigate to the site
  When I log in and search and add to cart and checkout
  Then everything should work
```

Good:
```gherkin
Scenario: User can log in
  Given I navigate to the login page
  When I enter valid credentials
  Then I should see the dashboard

Scenario: User can search products
  Given I am logged in and on the products page
  When I search for "laptop"
  Then I should see matching results
```

### 2. Use Descriptive Names
Name scenarios to clearly describe what is being tested.

Bad: `Scenario: Test 1`

Good: `Scenario: User receives email confirmation after registration`

### 3. Include Meaningful Assertions
Always verify the outcome, not just that the action completed.

Bad:
```gherkin
When I click the submit button
```

Good:
```gherkin
When I click the submit button
Then the success message should be visible
And the form should be cleared
```

### 4. Test User Workflows
Write tests that simulate real user behavior.

Good example - Login flow:
```gherkin
Scenario: Complete login workflow
  Given I navigate to "https://example.com"
  When I click on "Sign In"
  And I type into the email field "user@example.com"
  And I type into the password field "password"
  And I click the "Login" button
  And I wait for the dashboard to become visible
  Then the page title should contain "Dashboard"
```

### 5. Avoid Test Interdependency
Each test should be independent and runnable in any order.

Bad: Test 2 relies on Test 1 creating a user

Good: Each test sets up its own preconditions

Getting Help
------------

### Resources

- **Step Documentation**: Run `list-steps` and `search-steps` commands
- **Validate Feature Files**: Use `validate` to check syntax
- **Full CLI Guide**: See `docs/CLI.md`
- **Example Features**: Check `examples/features/` directory

### Common Questions

**Q: Can I use regular expressions in assertions?**
A: Yes, use the `text_should_match` step with regex patterns.

**Q: Can I extract data from pages?**
A: Yes, use steps like `extract_text_from_element` to get values.

**Q: Can I run tests in parallel?**
A: Currently, tests run sequentially. Use multiple feature files for parallel execution in your CI/CD system.

**Q: Can I add custom steps?**
A: The current version uses registered steps. See the full documentation for future plugin system details.

Conclusion
----------

You now have everything you need to start writing and running web automation tests with web-spec!

Start with one of the example feature files, modify it for your use case, and grow from there. The extensive library of 174+ step patterns should cover most common web testing scenarios.

Happy testing!
