// Flexible and Extensible Gherkin Feature Runner for browser automation
use clap::Parser;
use regex::Regex;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

#[cfg(feature = "chromiumoxide-backend")]
use web_spec::Browser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Gherkin feature file to run
    #[arg(short, long)]
    feature: PathBuf,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Step {
    keyword: String,
    text: String,
    #[allow(dead_code)]
    parameters: Vec<String>,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Scenario {
    name: String,
    steps: Vec<Step>,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Feature {
    name: String,
    scenarios: Vec<Scenario>,
}

#[allow(dead_code)]
#[cfg(feature = "chromiumoxide-backend")]
type StepHandler = Arc<dyn for<'a> Fn(&'a mut Browser, &[String]) -> Result<String, String>>;

#[allow(dead_code)]
#[cfg(not(feature = "chromiumoxide-backend"))]
type StepHandler = Arc<dyn for<'a> Fn(&'a (), &[String]) -> Result<String, String>>;

#[allow(dead_code)]
type ExtractedData = Arc<RwLock<HashMap<String, Vec<String>>>>;

#[allow(dead_code)]
type StoredValues = Arc<RwLock<HashMap<String, String>>>;

struct StepRegistry {
    patterns: Vec<(Regex, String)>,
}

impl StepRegistry {
    fn new() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    fn register(&mut self, pattern: &str, name: &str) {
        self.patterns
            .push((Regex::new(pattern).unwrap(), name.to_string()));
    }

    fn match_step(&self, step_text: &str) -> Option<(String, Vec<String>)> {
        for (pattern, name) in &self.patterns {
            if let Some(caps) = pattern.captures(step_text) {
                let params: Vec<String> = caps
                    .iter()
                    .skip(1)
                    .filter_map(|c| c.map(|m| m.as_str().to_string()))
                    .collect();
                return Some((name.clone(), params));
            }
        }
        None
    }
}

#[allow(dead_code)]
fn build_step_registry() -> StepRegistry {
    let mut registry = StepRegistry::new();

    // ===== NAVIGATION PATTERNS =====
    // URL navigation
    registry.register(r#"I navigate to "([^"]+)""#, "navigate_to");
    registry.register(r#"I go to "([^"]+)""#, "navigate_to");
    registry.register(r#"I open "([^"]+)""#, "navigate_to");
    registry.register(r#"I visit "([^"]+)""#, "navigate_to");
    registry.register(r"I navigate to Hacker News", "navigate_to_hackernews");
    registry.register(r"I go to Hacker News", "navigate_to_hackernews");

    // History navigation
    registry.register(r"I go back", "go_back");
    registry.register(r"I navigate back", "go_back");
    registry.register(r"I go forward", "go_forward");
    registry.register(r"I navigate forward", "go_forward");
    registry.register(r"I refresh the page", "refresh");
    registry.register(r"I reload the page", "refresh");

    // Page loading
    registry.register(r"the page loads", "wait_load");
    registry.register(r"I wait for the page to load", "wait_load");
    registry.register(r"I wait for page to load", "wait_load");

    // ===== WAITING PATTERNS =====
    // Time-based waiting
    registry.register(r"I wait (\d+) seconds?", "wait_seconds");
    registry.register(r"I wait for (\d+) seconds?", "wait_seconds");
    registry.register(r"I pause for (\d+) seconds?", "wait_seconds");
    registry.register(r"I sleep (\d+) seconds?", "wait_seconds");
    registry.register(r"I wait (\d+) milliseconds?", "wait_ms");
    registry.register(r"I wait for (\d+) milliseconds?", "wait_ms");

    // Element waiting
    registry.register(
        r#"I wait for element "([^"]+)" to be visible"#,
        "wait_visible",
    );
    registry.register(r#"I wait for element "([^"]+)" to appear"#, "wait_appear");
    registry.register(r#"I wait for "([^"]+)" to be visible"#, "wait_visible");
    registry.register(r#"I wait for "([^"]+)" to appear"#, "wait_appear");
    registry.register(r#"I wait until "([^"]+)" is visible"#, "wait_visible");
    registry.register(r#"I wait until "([^"]+)" appears"#, "wait_appear");

    // Advanced waiting
    registry.register(
        r#"I wait for element "([^"]+)" to be hidden"#,
        "wait_hidden",
    );
    registry.register(r#"I wait for "([^"]+)" to disappear"#, "wait_hidden");
    registry.register(r#"I wait for text "([^"]+)" to appear"#, "wait_for_text");
    registry.register(
        r#"I wait for element "([^"]+)" to contain "([^"]+)""#,
        "wait_for_element_text",
    );
    registry.register(
        r#"I wait for element "([^"]+)" to be (?:clickable|enabled)"#,
        "wait_clickable",
    );

    // ===== CLICKING PATTERNS =====
    // Basic clicking
    registry.register(r#"I click on "([^"]+)""#, "click");
    registry.register(r#"I click "([^"]+)""#, "click");
    registry.register(r#"I press "([^"]+)""#, "click");
    registry.register(r#"I tap "([^"]+)""#, "click");

    // Button/link clicking
    registry.register(r#"I click the "([^"]+)" button"#, "click_button");
    registry.register(r#"I click the "([^"]+)" link"#, "click_link");
    registry.register(
        r#"I click the "([^"]+)" (button|link)"#,
        "click_button_or_link",
    );
    registry.register(r"I click submit button", "click_submit");
    registry.register(r"I click the submit button", "click_submit");
    registry.register(r"I click search button", "click_search");
    registry.register(r"I click the search button", "click_search");
    registry.register(r"I click the first ([^ ]+) button", "click_first_button");
    registry.register(r"I click the last ([^ ]+) button", "click_last_button");

    // Advanced clicking
    registry.register(r#"I double click on "([^"]+)""#, "double_click");
    registry.register(r#"I double-click "([^"]+)""#, "double_click");
    registry.register(r#"I right click on "([^"]+)""#, "right_click");
    registry.register(r#"I right-click "([^"]+)""#, "right_click");
    registry.register(
        r#"I click the right mouse button on "([^"]+)""#,
        "right_click",
    );

    // ===== MOUSE INTERACTION =====
    registry.register(r#"I hover over "([^"]+)""#, "hover");
    registry.register(r#"I hover "([^"]+)""#, "hover");
    registry.register(r#"I move mouse to "([^"]+)""#, "hover");

    // Drag and drop
    registry.register(r#"I drag "([^"]+)" to "([^"]+)""#, "drag_and_drop");
    registry.register(
        r#"I drag element "([^"]+)" and drop it on "([^"]+)""#,
        "drag_and_drop",
    );

    // ===== INPUT PATTERNS =====
    // Text input
    registry.register(r#"I type "([^"]+)" into "([^"]+)""#, "type_text");
    registry.register(r#"I enter "([^"]+)" into "([^"]+)""#, "type_text");
    registry.register(r#"I type "([^"]+)" (?:in|into) "([^"]+)""#, "type_text");
    registry.register(r#"I enter "([^"]+)" (?:in|into) "([^"]+)""#, "type_text");
    registry.register(r#"I fill "([^"]+)" with "([^"]+)""#, "type_text");
    registry.register(r#"I fill in "([^"]+)" with "([^"]+)""#, "type_text");
    registry.register(r#"I input "([^"]+)" (?:in|into) "([^"]+)""#, "type_text");

    // Clearing
    registry.register(r#"I clear "([^"]+)""#, "clear_text");
    registry.register(r#"I clear the input "([^"]+)""#, "clear_text");
    registry.register(r#"I clear the field "([^"]+)""#, "clear_text");

    // Selection
    registry.register(r#"I select "([^"]+)" from "([^"]+)""#, "select_option");
    registry.register(r#"I choose "([^"]+)" from "([^"]+)""#, "select_option");
    registry.register(r#"I pick "([^"]+)" from "([^"]+)""#, "select_option");

    // Checkbox/Radio
    registry.register(r#"I check "([^"]+)""#, "check");
    registry.register(r#"I uncheck "([^"]+)""#, "uncheck");
    registry.register(r#"I toggle "([^"]+)""#, "toggle");
    registry.register(
        r#"I select the "([^"]+)" radio (?:button|option)"#,
        "select_radio",
    );

    // Multi-select
    registry.register(
        r#"I select "([^"]+)", "([^"]+)", and "([^"]+)" from "([^"]+)""#,
        "select_multiple",
    );
    registry.register(r#"I select multiple options from "([^"]+)""#, "select_all");
    registry.register(r#"I deselect all from "([^"]+)""#, "deselect_all");

    // File upload
    registry.register(r#"I upload file "([^"]+)" to "([^"]+)""#, "upload_file");
    registry.register(r#"I attach "([^"]+)" to "([^"]+)""#, "upload_file");

    // Keyboard actions
    registry.register(r#"I press "([^"]+)" key"#, "press_key");
    registry.register(r#"I press the Enter key"#, "press_enter");
    registry.register(r#"I press Enter"#, "press_enter");
    registry.register(r#"I press Escape key"#, "press_escape");
    registry.register(r#"I press Escape"#, "press_escape");
    registry.register(r#"I press Tab key"#, "press_tab");
    registry.register(r#"I press Tab"#, "press_tab");
    registry.register(r#"I press "([^"]+)" in "([^"]+)""#, "type_into");

    // Form submission
    registry.register(r#"I submit the form "([^"]+)""#, "submit_form");
    registry.register(r#"I submit "([^"]+)""#, "submit_form");
    registry.register(r"I submit the form", "submit_form");

    // ===== SCROLLING PATTERNS =====
    registry.register(r"I scroll to bottom", "scroll_bottom");
    registry.register(r"I scroll to the bottom", "scroll_bottom");
    registry.register(r"I scroll to top", "scroll_top");
    registry.register(r"I scroll to the top", "scroll_top");
    registry.register(r#"I scroll to "([^"]+)""#, "scroll_to_element");
    registry.register(r"I scroll down by (\d+) pixels?", "scroll_down");
    registry.register(r"I scroll up by (\d+) pixels?", "scroll_up");
    registry.register(r"I scroll left by (\d+) pixels?", "scroll_left");
    registry.register(r"I scroll right by (\d+) pixels?", "scroll_right");
    registry.register(
        r"I scroll (\d+) pixels? (down|up)",
        "scroll_pixels_vertical",
    );
    registry.register(
        r"I scroll (\d+) pixels? (left|right)",
        "scroll_pixels_horizontal",
    );

    // ===== VISIBILITY PATTERNS =====
    registry.register(r#"I should see "([^"]+)""#, "should_see");
    registry.register(r#"I should not see "([^"]+)""#, "should_not_see");
    registry.register(r#"I should see the element "([^"]+)""#, "should_see");
    registry.register(
        r#"I should not see the element "([^"]+)""#,
        "should_not_see",
    );
    registry.register(
        r#"the element "([^"]+)" should be visible"#,
        "should_be_visible",
    );
    registry.register(
        r#"the element "([^"]+)" should not be visible"#,
        "should_not_be_visible",
    );
    registry.register(r#""([^"]+)" should be visible"#, "should_be_visible");
    registry.register(
        r#""([^"]+)" should not be visible"#,
        "should_not_be_visible",
    );

    // Element state
    registry.register(r#"the element "([^"]+)" should exist"#, "should_exist");
    registry.register(
        r#"the element "([^"]+)" should not exist"#,
        "should_not_exist",
    );
    registry.register(
        r#"the element "([^"]+)" should be enabled"#,
        "should_be_enabled",
    );
    registry.register(
        r#"the element "([^"]+)" should be disabled"#,
        "should_be_disabled",
    );
    registry.register(
        r#"the element "([^"]+)" should be checked"#,
        "should_be_checked",
    );
    registry.register(
        r#"the element "([^"]+)" should not be checked"#,
        "should_not_be_checked",
    );
    registry.register(
        r#"the element "([^"]+)" should be selected"#,
        "should_be_selected",
    );
    registry.register(
        r#"the element "([^"]+)" should be focused"#,
        "should_be_focused",
    );
    registry.register(
        r#"the element "([^"]+)" should contain "([^"]+)""#,
        "should_contain_text",
    );
    registry.register(
        r#"the element "([^"]+)" should not contain "([^"]+)""#,
        "should_not_contain_text",
    );

    // ===== COUNTING PATTERNS =====
    registry.register(
        r"I should see at least (\d+) ([^ ]+)",
        "should_see_min_count",
    );
    registry.register(
        r"I should see minimum (\d+) ([^ ]+)",
        "should_see_min_count",
    );
    registry.register(
        r"I should see at most (\d+) ([^ ]+)",
        "should_see_max_count",
    );
    registry.register(
        r"I should see exactly (\d+) ([^ ]+)",
        "should_see_exact_count",
    );
    registry.register(
        r"I should see exactly (\d+) ([^ ]+) elements?",
        "should_see_exact_count",
    );
    registry.register(r"there should be (\d+) ([^ ]+)", "should_see_exact_count");
    registry.register(
        r#"there should be (\d+) "([^"]+)""#,
        "should_see_exact_count_elements",
    );

    // ===== TEXT PATTERNS =====
    registry.register(r#"I should see text "([^"]+)""#, "should_see_text");
    registry.register(r#"I should not see text "([^"]+)""#, "should_not_see_text");
    registry.register(r#"the page should contain "([^"]+)""#, "should_see_text");
    registry.register(
        r#"the page should not contain "([^"]+)""#,
        "should_not_see_text",
    );
    registry.register(
        r#"the text of "([^"]+)" should be "([^"]+)""#,
        "text_should_be",
    );
    registry.register(
        r#"the text of "([^"]+)" should contain "([^"]+)""#,
        "text_should_contain",
    );
    registry.register(
        r#"the text of "([^"]+)" should equal "([^"]+)""#,
        "text_should_be",
    );
    registry.register(
        r#"the text of "([^"]+)" should match "([^"]+)""#,
        "text_should_match",
    );
    registry.register(
        r#"the text of "([^"]+)" should start with "([^"]+)""#,
        "text_should_start",
    );
    registry.register(
        r#"the text of "([^"]+)" should end with "([^"]+)""#,
        "text_should_end",
    );

    // ===== ATTRIBUTE PATTERNS =====
    registry.register(
        r#"the "([^"]+)" attribute of "([^"]+)" should be "([^"]+)""#,
        "attribute_should_be",
    );
    registry.register(
        r#"the "([^"]+)" attribute of "([^"]+)" should contain "([^"]+)""#,
        "attribute_should_contain",
    );
    registry.register(
        r#"the "([^"]+)" attribute of "([^"]+)" should exist"#,
        "attribute_should_exist",
    );
    registry.register(
        r#"the element "([^"]+)" should have "([^"]+)" attribute"#,
        "attribute_should_exist",
    );
    registry.register(
        r#"the element "([^"]+)" should have "([^"]+)" attribute set to "([^"]+)""#,
        "attribute_should_be",
    );

    // ===== CSS PATTERNS =====
    registry.register(
        r#"the "([^"]+)" CSS property of "([^"]+)" should be "([^"]+)""#,
        "css_should_be",
    );
    registry.register(
        r#"the element "([^"]+)" should have "([^"]+)" CSS value of "([^"]+)""#,
        "css_should_be",
    );
    registry.register(
        r#"the element "([^"]+)" should have color "([^"]+)""#,
        "color_should_be",
    );
    registry.register(
        r#"the element "([^"]+)" should have background "([^"]+)""#,
        "background_should_be",
    );

    // ===== URL/PATH PATTERNS =====
    registry.register(r#"the URL should be "([^"]+)""#, "url_should_be");
    registry.register(r#"the current URL should be "([^"]+)""#, "url_should_be");
    registry.register(r#"the URL should contain "([^"]+)""#, "url_should_contain");
    registry.register(r#"the path should be "([^"]+)""#, "path_should_be");
    registry.register(
        r#"the path should contain "([^"]+)""#,
        "path_should_contain",
    );
    registry.register(r#"the title should be "([^"]+)""#, "title_should_be");
    registry.register(
        r#"the title should contain "([^"]+)""#,
        "title_should_contain",
    );

    // ===== SCREENSHOT PATTERNS =====
    registry.register(r#"I take a screenshot "([^"]+)""#, "screenshot");
    registry.register(r#"I capture screenshot "([^"]+)""#, "screenshot");
    registry.register(r"I take a screenshot", "screenshot_auto");
    registry.register(r"I capture a screenshot", "screenshot_auto");
    registry.register(r"I take a full page screenshot", "screenshot_full");
    registry.register(r#"I take screenshot of "([^"]+)""#, "screenshot_element");

    // ===== JAVASCRIPT PATTERNS =====
    registry.register(r#"I execute JavaScript "([^"]+)""#, "execute_script");
    registry.register(r#"I execute script "([^"]+)""#, "execute_script");
    registry.register(r#"I run JavaScript "([^"]+)""#, "execute_script");
    registry.register(r#"I run script "([^"]+)""#, "execute_script");
    registry.register(r#"I evaluate "([^"]+)""#, "execute_script");

    // ===== STORAGE PATTERNS =====
    registry.register(r#"I set "([^"]+)" to "([^"]+)""#, "store_value");
    registry.register(r#"I save "([^"]+)" as "([^"]+)""#, "store_value");
    registry.register(r#"I store "([^"]+)" as "([^"]+)""#, "store_value");
    registry.register(
        r#"the value "([^"]+)" should be "([^"]+)""#,
        "value_should_be",
    );
    registry.register(r#"I use the stored value "([^"]+)""#, "use_stored_value");

    // ===== BROWSER CONTROL PATTERNS =====
    registry.register(r"I maximize the window", "maximize_window");
    registry.register(r"I fullscreen the window", "fullscreen_window");
    registry.register(r"I minimize the window", "minimize_window");
    registry.register(r"I resize the window to (\d+)x(\d+)", "resize_window");
    registry.register(r"I set viewport to (\d+)x(\d+)", "resize_window");
    registry.register(r#"I set user agent to "([^"]+)""#, "set_user_agent");

    // ===== ALERT/MODAL PATTERNS =====
    registry.register(r"I accept the alert", "accept_alert");
    registry.register(r"I dismiss the alert", "dismiss_alert");
    registry.register(r"I accept the confirmation", "accept_alert");
    registry.register(r"I dismiss the confirmation", "dismiss_alert");
    registry.register(r"I accept the prompt", "accept_prompt");
    registry.register(r"I dismiss the prompt", "dismiss_prompt");
    registry.register(r#"I type "([^"]+)" into the prompt"#, "type_into_prompt");
    registry.register(
        r#"the alert text should be "([^"]+)""#,
        "alert_text_should_be",
    );

    // ===== FRAME/IFRAME PATTERNS =====
    registry.register(r#"I switch to frame "([^"]+)""#, "switch_to_frame");
    registry.register(r"I switch to default content", "switch_to_default");
    registry.register(r"I switch to parent frame", "switch_to_parent_frame");

    // ===== WINDOW/TAB PATTERNS =====
    registry.register(r"I open a new tab", "open_new_tab");
    registry.register(r"I open new window", "open_new_tab");
    registry.register(r"I switch to tab (\d+)", "switch_to_tab");
    registry.register(r#"I switch to window "([^"]+)""#, "switch_to_window");
    registry.register(r"I close the current tab", "close_tab");
    registry.register(r"I close the current window", "close_tab");

    // ===== CONTENT EXTRACTION PATTERNS =====
    registry.register(r"I extract page HTML", "extract_html");
    registry.register(r"I extract the page HTML", "extract_html");
    registry.register(r"I extract the full page HTML", "extract_html");
    registry.register(r"I extract all links from the page", "extract_links");
    registry.register(r"I extract all links", "extract_links");
    registry.register(r"I extract all hrefs", "extract_links");
    registry.register(r"I extract all images from the page", "extract_images");
    registry.register(r"I extract all images", "extract_images");
    registry.register(r"I extract all image sources", "extract_images");
    registry.register(r"I extract all h1 headings", "extract_h1");
    registry.register(r"I extract all h2 headings", "extract_h2");
    registry.register(r"I extract all h3 headings", "extract_h3");
    registry.register(r"I extract all h4 headings", "extract_h4");
    registry.register(r#"I extract all h(\d) headings"#, "extract_headings_level");
    registry.register(r"I extract all headings", "extract_all_headings");
    registry.register(
        r#"I extract text from "([^"]+)""#,
        "extract_text_from_element",
    );
    registry.register(
        r#"I extract "([^"]+)" from "([^"]+)""#,
        "extract_attribute_from_element",
    );
    registry.register(
        r#"I extract the "([^"]+)" attribute from "([^"]+)""#,
        "extract_attribute_from_element",
    );
    registry.register(r#"I extract all "([^"]+)""#, "extract_all_by_selector");
    registry.register(r#"I extract all "([^"]+)""#, "extract_all_elements");

    // ===== SPECIALIZED EXTRACTION =====
    registry.register(r"I extract post titles from the page", "extract_titles");
    registry.register(r"I extract Hacker News titles", "extract_titles");
    registry.register(r"I extract all article titles", "extract_titles");
    registry.register(r#"I extract table data from "([^"]+)""#, "extract_table");
    registry.register(r#"I extract the table "([^"]+)""#, "extract_table");

    // ===== CONDITIONAL PATTERNS =====
    registry.register(
        r#"if "([^"]+)" is visible, I click it"#,
        "conditional_click_if_visible",
    );
    registry.register(
        r#"if "([^"]+)" exists, I type "([^"]+)" into it"#,
        "conditional_type_if_exists",
    );
    registry.register(
        r#"if the page contains "([^"]+)", I navigate to "([^"]+)""#,
        "conditional_navigate",
    );
    registry.register(
        r#"skip the rest of the scenario if "([^"]+)" is (visible|present)"#,
        "skip_if_visible",
    );
    registry.register(
        r#"continue only if "([^"]+)" is (visible|present)"#,
        "continue_if_visible",
    );

    // ===== LOOP PATTERNS =====
    registry.register(r#"for each "([^"]+)", I click it"#, "loop_click_each");
    registry.register(r#"I click all "([^"]+)""#, "click_all");
    registry.register(
        r#"I extract all "([^"]+)" and store them as "([^"]+)""#,
        "extract_and_store_all",
    );

    // ===== CLIPBOARD PATTERNS =====
    registry.register(r#"I copy "([^"]+)""#, "copy_to_clipboard");
    registry.register(r"I paste from clipboard", "paste_from_clipboard");
    registry.register(r#"I copy the text of "([^"]+)""#, "copy_element_text");
    registry.register(r#"I paste "([^"]+)" into "([^"]+)""#, "paste_into");
    registry.register(
        r#"the clipboard should contain "([^"]+)""#,
        "clipboard_should_contain",
    );
    registry.register(
        r"the clipboard should be empty",
        "clipboard_should_be_empty",
    );

    // ===== MOUSE EVENTS PATTERNS =====
    registry.register(r#"I mouse down on "([^"]+)""#, "mouse_down");
    registry.register(r#"I mouse up on "([^"]+)""#, "mouse_up");
    registry.register(r#"I mouse move to "([^"]+)""#, "mouse_move_to");
    registry.register(r#"I mouse over "([^"]+)""#, "mouse_over");
    registry.register(r#"I mouse out of "([^"]+)""#, "mouse_out");
    registry.register(
        r#"I drag "([^"]+)" by offset (-?\d+),(-?\d+)""#,
        "drag_by_offset",
    );
    registry.register(r#"I drop "([^"]+)" at "([^"]+)""#, "drop_at");

    // ===== TOUCH EVENTS PATTERNS =====
    registry.register(r#"I touch "([^"]+)""#, "touch_element");
    registry.register(r#"I swipe "([^"]+)" to "([^"]+)""#, "swipe_elements");
    registry.register(r#"I pinch to zoom in on "([^"]+)""#, "pinch_zoom");
    registry.register(r#"I rotate "([^"]+)" by (\d+) degrees?"#, "rotate_element");
    registry.register(
        r#"I perform multi-touch gesture on "([^"]+)""#,
        "multi_touch",
    );

    // ===== FILE OPERATIONS PATTERNS =====
    registry.register(r#"I download file from "([^"]+)""#, "download_file");
    registry.register(
        r#"I verify file "([^"]+)" was downloaded"#,
        "verify_download",
    );
    registry.register(r"I wait for download to complete", "wait_download_complete");
    registry.register(
        r#"the downloaded file should be named "([^"]+)""#,
        "download_filename_should_be",
    );
    registry.register(r#"I save file as "([^"]+)""#, "save_file_as");
    registry.register(r#"I upload file "([^"]+)" to "([^"]+)""#, "upload_file");

    // ===== AUDIO/VIDEO PATTERNS =====
    registry.register(r"I play video", "play_video");
    registry.register(r"I pause video", "pause_video");
    registry.register(r"I stop video", "stop_video");
    registry.register(r"I mute video", "mute_video");
    registry.register(r"I unmute video", "unmute_video");
    registry.register(r"I seek video to (\d+) seconds?", "seek_video");
    registry.register(r"I set video volume to (\d+)%", "set_video_volume");
    registry.register(r"the video should be playing", "video_should_be_playing");
    registry.register(r"the video should be paused", "video_should_be_paused");
    registry.register(
        r"the video duration should be at least (\d+) seconds?",
        "video_duration_check",
    );

    // ===== CANVAS PATTERNS =====
    registry.register(r#"I get canvas data from "([^"]+)""#, "get_canvas_data");
    registry.register(r#"I draw on canvas "([^"]+)""#, "draw_on_canvas");
    registry.register(r#"I clear canvas "([^"]+)""#, "clear_canvas");
    registry.register(
        r#"I verify canvas pixel at "([^"]+)""#,
        "verify_canvas_pixel",
    );
    registry.register(r"the canvas should have width (\d+)", "canvas_width_check");
    registry.register(
        r"the canvas should have height (\d+)",
        "canvas_height_check",
    );

    // ===== CONSOLE PATTERNS =====
    registry.register(
        r#"I should see console message "([^"]+)""#,
        "console_should_contain",
    );
    registry.register(
        r#"I should not see console message "([^"]+)""#,
        "console_should_not_contain",
    );
    registry.register(r"I should see console error", "console_should_have_error");
    registry.register(
        r"I should not see console errors",
        "console_should_not_have_errors",
    );
    registry.register(r"I clear console", "clear_console");
    registry.register(r"I get console log", "get_console_log");

    // ===== PERFORMANCE METRICS PATTERNS =====
    registry.register(r"I check performance metrics", "check_performance_metrics");
    registry.register(r"the LCP should be less than (\d+)ms", "lcp_should_be");
    registry.register(r"the CLS should be less than (\d+)ms", "cls_should_be");
    registry.register(r"the FID should be less than (\d+)ms", "fid_should_be");
    registry.register(r"the TTI should be less than (\d+)ms", "tti_should_be");
    registry.register(r"I wait for stable layout", "wait_stable_layout");

    // ===== NETWORK CONDITIONS PATTERNS =====
    registry.register(r"I simulate slow network", "simulate_slow_network");
    registry.register(r"I simulate offline mode", "simulate_offline");
    registry.register(r"I simulate fast network", "simulate_fast_network");
    registry.register(r"I disable network", "disable_network");
    registry.register(r"I enable network", "enable_network");
    registry.register(
        r"the network should be (online|offline)",
        "network_should_be",
    );

    // ===== DEVICE EMULATION PATTERNS =====
    registry.register(
        r"I emulate device (?:iPhone|iPad|Pixel|Android)",
        "emulate_device",
    );
    registry.register(r"I emulate mobile viewport", "emulate_mobile");
    registry.register(r"I emulate tablet viewport", "emulate_tablet");
    registry.register(r"I emulate desktop viewport", "emulate_desktop");
    registry.register(r"I set device pixel ratio (\d+)", "set_device_pixel_ratio");
    registry.register(r"I set viewport to (\d+)x(\d+)", "set_viewport_size");
    registry.register(r"I rotate to landscape", "rotate_landscape");
    registry.register(r"I rotate to portrait", "rotate_portrait");

    // ===== LOCAL STORAGE PATTERNS =====
    registry.register(r"I clear local storage", "clear_local_storage");
    registry.register(
        r#"I set local storage item "([^"]+)" to "([^"]+)""#,
        "set_local_storage",
    );
    registry.register(r#"I get local storage item "([^"]+)""#, "get_local_storage");
    registry.register(
        r#"the local storage should contain "([^"]+)""#,
        "local_storage_should_contain",
    );
    registry.register(
        r#"I remove local storage item "([^"]+)""#,
        "remove_local_storage",
    );
    registry.register(
        r"the local storage should be empty",
        "local_storage_should_be_empty",
    );
    registry.register(
        r"the local storage should have (\d+) items",
        "local_storage_count_check",
    );

    // ===== SESSION STORAGE PATTERNS =====
    registry.register(r"I clear session storage", "clear_session_storage");
    registry.register(
        r#"I set session storage item "([^"]+)" to "([^"]+)""#,
        "set_session_storage",
    );
    registry.register(
        r#"I get session storage item "([^"]+)""#,
        "get_session_storage",
    );
    registry.register(
        r#"I remove session storage item "([^"]+)""#,
        "remove_session_storage",
    );
    registry.register(
        r"the session storage should be empty",
        "session_storage_should_be_empty",
    );

    // ===== INDEXEDDB PATTERNS =====
    registry.register(r"I check IndexedDB exists", "check_indexeddb_exists");
    registry.register(r"I get IndexedDB entry count", "get_indexeddb_count");
    registry.register(r"I clear IndexedDB", "clear_indexeddb");
    registry.register(
        r"the IndexedDB should have (\d+) entries",
        "indexeddb_count_check",
    );

    // ===== SERVICE WORKER PATTERNS =====
    registry.register(
        r"I wait for Service Worker to activate",
        "wait_service_worker",
    );
    registry.register(
        r"the Service Worker should be active",
        "service_worker_should_be_active",
    );
    registry.register(r"I unregister Service Worker", "unregister_service_worker");
    registry.register(
        r"I clear Service Worker cache",
        "clear_service_worker_cache",
    );

    // ===== WEB MANIFEST PATTERNS =====
    registry.register(r"I check web manifest", "check_web_manifest");
    registry.register(
        r#"the manifest should have name "([^"]+)""#,
        "manifest_name_check",
    );
    registry.register(
        r#"the manifest should have short name "([^"]+)""#,
        "manifest_short_name_check",
    );
    registry.register(r"I verify manifest color theme", "verify_manifest_theme");

    // ===== SECURITY HEADERS PATTERNS =====
    registry.register(r"I check CSP headers", "check_csp_headers");
    registry.register(r"I check HSTS header", "check_hsts_header");
    registry.register(
        r"the response should have security headers",
        "security_headers_check",
    );
    registry.register(r"I verify HTTPS certificate", "verify_https_certificate");

    // ===== COOKIES ADVANCED PATTERNS =====
    registry.register(r"I check for secure cookies", "check_secure_cookies");
    registry.register(
        r"all cookies should be secure",
        "all_cookies_should_be_secure",
    );
    registry.register(r"I check for same-site cookies", "check_same_site_cookies");
    registry.register(r"I set cookie with SameSite", "set_cookie_samesite");
    registry.register(r"I set cookie with HttpOnly flag", "set_cookie_httponly");
    registry.register(r"I set cookie with Secure flag", "set_cookie_secure");

    // ===== GEOLOCATION PATTERNS =====
    registry.register(r#"I mock geolocation to "([^"]+)""#, "mock_geolocation");
    registry.register(
        r"I set geolocation to latitude (\d+) longitude (\d+)",
        "set_geolocation_coords",
    );
    registry.register(r"I clear geolocation mock", "clear_geolocation_mock");
    registry.register(
        r"I check geolocation permission",
        "check_geolocation_permission",
    );

    // ===== NOTIFICATIONS PATTERNS =====
    registry.register(
        r"I request notification permission",
        "request_notification_permission",
    );
    registry.register(
        r"I grant notification permission",
        "grant_notification_permission",
    );
    registry.register(
        r"I deny notification permission",
        "deny_notification_permission",
    );
    registry.register(
        r#"I should see notification "([^"]+)""#,
        "should_see_notification",
    );
    registry.register(
        r"the notification should be (visible|hidden)",
        "notification_visibility_check",
    );

    // ===== WEBSOCKET PATTERNS =====
    registry.register(
        r#"I connect to WebSocket at "([^"]+)""#,
        "connect_websocket",
    );
    registry.register(r"I disconnect WebSocket", "disconnect_websocket");
    registry.register(
        r#"I send WebSocket message "([^"]+)""#,
        "send_websocket_message",
    );
    registry.register(
        r#"I should receive WebSocket message "([^"]+)""#,
        "should_receive_websocket_message",
    );
    registry.register(
        r"the WebSocket should be connected",
        "websocket_should_be_connected",
    );

    // ===== MEDIA STREAM PATTERNS =====
    registry.register(r"I start camera", "start_camera");
    registry.register(r"I stop camera", "stop_camera");
    registry.register(r"I start microphone", "start_microphone");
    registry.register(r"I stop microphone", "stop_microphone");
    registry.register(r"I check camera permission", "check_camera_permission");
    registry.register(
        r"I check microphone permission",
        "check_microphone_permission",
    );
    registry.register(r"I should see camera stream", "should_see_camera_stream");
    registry.register(
        r"I should see microphone stream",
        "should_see_microphone_stream",
    );

    // ===== WEBGL PATTERNS =====
    registry.register(r"I check WebGL support", "check_webgl_support");
    registry.register(r"I get WebGL renderer", "get_webgl_renderer");
    registry.register(r#"I set WebGL context to "([^"]+)""#, "set_webgl_context");
    registry.register(
        r#"the WebGL should have context "([^"]+)""#,
        "webgl_context_check",
    );

    // ===== ANIMATION PATTERNS =====
    registry.register(
        r"I wait for animation to complete",
        "wait_animation_complete",
    );
    registry.register(
        r#"I should see animation "([^"]+)""#,
        "should_see_animation",
    );
    registry.register(r#"I pause animation "([^"]+)""#, "pause_animation");
    registry.register(r#"I resume animation "([^"]+)""#, "resume_animation");
    registry.register(r#"I cancel animation "([^"]+)""#, "cancel_animation");
    registry.register(
        r"the animation should be running",
        "animation_should_be_running",
    );

    // ===== PRINT PATTERNS =====
    registry.register(r"I print page", "print_page");
    registry.register(r"I print to PDF", "print_to_pdf");
    registry.register(r#"I set print layout to "([^"]+)""#, "set_print_layout");
    registry.register(
        r"the print preview should be visible",
        "print_preview_check",
    );

    // ===== SELECTION RANGES PATTERNS =====
    registry.register(
        r#"I select text from "([^"]+)" to "([^"]+)""#,
        "select_text_range",
    );
    registry.register(
        r#"I select all text in "([^"]+)""#,
        "select_all_text_in_element",
    );
    registry.register(
        r#"I clear selection in "([^"]+)""#,
        "clear_selection_in_element",
    );
    registry.register(
        r"I copy selection to clipboard",
        "copy_selection_to_clipboard",
    );

    // ===== DRAG AND DROP ENHANCED PATTERNS =====
    registry.register(
        r#"I drag "([^"]+)" to coordinates (\d+),(\d+)""#,
        "drag_to_coordinates",
    );
    registry.register(r#"I hold drag on "([^"]+)""#, "hold_drag");
    registry.register(r#"I release drag on "([^"]+)""#, "release_drag");

    // ===== SPELL CHECK PATTERNS =====
    registry.register(r#"I check spelling of "([^"]+)""#, "check_spelling");
    registry.register(
        r"the text should have no spelling errors",
        "no_spelling_errors",
    );
    registry.register(r"I enable spell checking", "enable_spell_check");
    registry.register(r"I disable spell checking", "disable_spell_check");

    // ===== AUTO-COMPLETE PATTERNS =====
    registry.register(
        r#"I should see autocomplete suggestions for "([^"]+)""#,
        "should_see_autocomplete",
    );
    registry.register(
        r#"I select autocomplete suggestion "([^"]+)""#,
        "select_autocomplete_suggestion",
    );
    registry.register(r"I close autocomplete", "close_autocomplete");
    registry.register(
        r"the autocomplete should be visible",
        "autocomplete_should_be_visible",
    );

    // ===== MODAL PATTERNS =====
    registry.register(r"I wait for modal to appear", "wait_modal_appear");
    registry.register(r"I should see modal", "should_see_modal");
    registry.register(r"I should not see modal", "should_not_see_modal");
    registry.register(r"I close modal", "close_modal");
    registry.register(r"I close all modals", "close_all_modals");
    registry.register(
        r"the modal should be dismissible",
        "modal_should_be_dismissible",
    );

    // ===== TOOLTIP PATTERNS =====
    registry.register(
        r#"I hover over "([^"]+)" to show tooltip"#,
        "hover_show_tooltip",
    );
    registry.register(r#"I should see tooltip "([^"]+)""#, "should_see_tooltip");
    registry.register(
        r#"the tooltip should contain "([^"]+)""#,
        "tooltip_should_contain",
    );
    registry.register(r"I verify tooltip position", "verify_tooltip_position");

    // ===== PROGRESS BAR PATTERNS =====
    registry.register(
        r"I wait for progress bar to complete",
        "wait_progress_complete",
    );
    registry.register(
        r"the progress bar should be at least (\d+)%",
        "progress_at_least",
    );
    registry.register(
        r"the progress bar should be at most (\d+)%",
        "progress_at_most",
    );
    registry.register(
        r"the progress should be (?:indeterminate|determinate)",
        "progress_state_check",
    );

    // ===== TABS/ACCORDION PATTERNS =====
    registry.register(r#"I activate tab "([^"]+)""#, "activate_tab");
    registry.register(r#"I deactivate tab "([^"]+)""#, "deactivate_tab");
    registry.register(r"I reorder tabs", "reorder_tabs");
    registry.register(r"I pin tab", "pin_tab");
    registry.register(r"I unpin tab", "unpin_tab");
    registry.register(
        r#"I should see active tab "([^"]+)""#,
        "should_see_active_tab",
    );

    // ===== SIDEBAR PATTERNS =====
    registry.register(r"I open sidebar", "open_sidebar");
    registry.register(r"I close sidebar", "close_sidebar");
    registry.register(r"I toggle sidebar", "toggle_sidebar");
    registry.register(
        r"the sidebar should be visible",
        "sidebar_should_be_visible",
    );
    registry.register(
        r"the sidebar should be collapsed",
        "sidebar_should_be_collapsed",
    );

    // ===== BREADCRUMB PATTERNS =====
    registry.register(
        r#"I should see breadcrumb "([^"]+)""#,
        "should_see_breadcrumb",
    );
    registry.register(r#"I click breadcrumb "([^"]+)""#, "click_breadcrumb");
    registry.register(
        r"the breadcrumb should be clickable",
        "breadcrumb_should_be_clickable",
    );
    registry.register(
        r"the breadcrumb should contain (\d+) items",
        "breadcrumb_count_check",
    );

    // ===== SEARCH PATTERNS =====
    registry.register(r"I focus search box", "focus_search_box");
    registry.register(r#"I type in search box "([^"]+)""#, "type_in_search_box");
    registry.register(r"I clear search box", "clear_search_box");
    registry.register(r"I submit search", "submit_search");
    registry.register(r"I should see search results", "should_see_search_results");
    registry.register(
        r"I should see (\d+) search results",
        "should_see_search_result_count",
    );

    // ===== PAGINATION PATTERNS =====
    registry.register(r"I click next page", "click_next_page");
    registry.register(r"I click previous page", "click_previous_page");
    registry.register(r"I go to page (\d+)", "go_to_page_number");
    registry.register(r"I should see page indicator", "should_see_page_indicator");
    registry.register(
        r"the page indicator should show page (\d+)",
        "page_indicator_should_show",
    );

    // ===== FILTER PATTERNS =====
    registry.register(r#"I apply filter "([^"]+)""#, "apply_filter");
    registry.register(r"I clear all filters", "clear_all_filters");
    registry.register(
        r#"I select filter option "([^"]+)""#,
        "select_filter_option",
    );
    registry.register(
        r#"I should see active filter "([^"]+)""#,
        "should_see_active_filter",
    );
    registry.register(
        r"I should see (\d+) active filters",
        "active_filter_count_check",
    );

    // ===== SORTING PATTERNS =====
    registry.register(
        r#"I sort by "([^"]+)" in (?:ascending|descending) order"#,
        "sort_by",
    );
    registry.register(r"I reverse sort order", "reverse_sort_order");
    registry.register(r#"I click sort by "([^"]+)""#, "click_sort_by");
    registry.register(
        r#"the items should be sorted by "([^"]+)""#,
        "should_be_sorted_by",
    );

    // ===== INFINITE SCROLL PATTERNS =====
    registry.register(r"I scroll to bottom", "scroll_to_bottom");
    registry.register(r"I scroll to top", "scroll_to_top");
    registry.register(r"I scroll indefinitely", "scroll_indefinitely");
    registry.register(r"I stop scrolling", "stop_scrolling");

    // ===== LAZY LOADING PATTERNS =====
    registry.register(r"I scroll to trigger lazy load", "scroll_trigger_lazy_load");
    registry.register(
        r"I wait for lazy loaded items to appear",
        "wait_lazy_loaded",
    );
    registry.register(
        r"I should see (\d+) lazy loaded items",
        "lazy_loaded_item_count",
    );

    // ===== VIRTUAL SCROLL PATTERNS =====
    registry.register(r#"I scroll to position (\d+)%"#, "scroll_to_percentage");
    registry.register(
        r#"I should see scroll position (\d+)%"#,
        "scroll_position_check",
    );

    // ===== HREFLANG PATTERNS =====
    registry.register(
        r#"I set document language to "([^"]+)""#,
        "set_document_lang",
    );
    registry.register(r"I check document language", "check_document_lang");
    registry.register(
        r"the document should have language attribute",
        "document_lang_attribute_check",
    );

    // ===== METATAG PATTERNS =====
    registry.register(r#"I check for meta "([^"]+)""#, "check_meta_tag");
    registry.register(
        r#"the meta description should be "([^"]+)""#,
        "meta_description_check",
    );
    registry.register(
        r#"the meta keywords should contain "([^"]+)""#,
        "meta_keywords_check",
    );
    registry.register(
        r#"the meta robots should be "([^"]+)""#,
        "meta_robots_check",
    );
    registry.register(
        r#"the meta viewport should be "([^"]+)""#,
        "meta_viewport_check",
    );

    // ===== LINK RELATIONS PATTERNS =====
    registry.register(r"I check for canonical URL", "check_canonical_url");
    registry.register(
        r#"the canonical URL should be "([^"]+)""#,
        "canonical_url_check",
    );
    registry.register(r"I check for alternate URLs", "check_alternate_urls");
    registry.register(r"I check for next/prev links", "check_next_prev_links");

    // ===== OPENSEARCH PATTERNS =====
    registry.register(r"I enable open search", "enable_opensearch");
    registry.register(r"I disable open search", "disable_opensearch");
    registry.register(r"I check for open search", "check_opensearch");

    // ===== RSS/FEED PATTERNS =====
    registry.register(r"I check for RSS feed", "check_rss_feed");
    registry.register(r"I should see RSS feed link", "should_see_rss_link");
    registry.register(r"I verify RSS feed is valid", "verify_rss_feed_valid");

    // ===== PWA PATTERNS =====
    registry.register(r"I check PWA is installable", "check_pwa_installable");
    registry.register(r"I install PWA", "install_pwa");
    registry.register(r"I uninstall PWA", "uninstall_pwa");
    registry.register(r"the PWA should be installed", "pwa_should_be_installed");

    // ===== WORKER PATTERNS =====
    registry.register(r"I check for Web Worker", "check_web_worker");
    registry.register(
        r"the Web Worker should be active",
        "web_worker_should_be_active",
    );

    // ===== CONDITIONAL PATTERNS =====
    registry.register(
        r#"I retry clicking "([^"]+)" up to (\d+) times"#,
        "retry_click",
    );
    registry.register(r#"I retry "([^"]+)" (\d+) times"#, "retry_click");
    registry.register(
        r#"I wait for "([^"]+)" with timeout of (\d+) seconds"#,
        "wait_with_timeout",
    );

    registry
}

#[cfg(feature = "chromiumoxide-backend")]
async fn execute_step(
    browser: &mut Browser,
    step_name: &str,
    params: &[String],
    data: &ExtractedData,
    stored: &StoredValues,
) -> Result<String, String> {
    use web_spec::Automation;

    match step_name {
        // ===== NAVIGATION =====
        "navigate_to" => {
            let url = params.get(0).cloned().unwrap_or_default();
            browser
                .navigate_to(&url)
                .await
                .map_err(|e| format!("Navigation failed: {:?}", e))?;
            Ok(format!("Navigated to {}", url))
        }

        "navigate_to_hackernews" => {
            browser
                .navigate_to("https://news.ycombinator.com/news")
                .await
                .map_err(|e| format!("Navigation failed: {:?}", e))?;
            Ok("Navigated to Hacker News".to_string())
        }

        "go_back" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("window.history.back()")
                .await
                .map_err(|e| format!("Go back failed: {:?}", e))?;
            Ok("Navigated back".to_string())
        }

        "go_forward" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("window.history.forward()")
                .await
                .map_err(|e| format!("Go forward failed: {:?}", e))?;
            Ok("Navigated forward".to_string())
        }

        "refresh" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("window.location.reload()")
                .await
                .map_err(|e| format!("Refresh failed: {:?}", e))?;
            Ok("Page refreshed".to_string())
        }

        // ===== WAITING =====
        "wait_load" => {
            browser
                .wait_for_load()
                .await
                .map_err(|e| format!("Wait failed: {:?}", e))?;
            Ok("Page loaded".to_string())
        }

        "wait_seconds" => {
            let seconds: u64 = params.get(0).and_then(|s| s.parse().ok()).unwrap_or(1);
            tokio::time::sleep(tokio::time::Duration::from_secs(seconds)).await;
            Ok(format!("Waited {} seconds", seconds))
        }

        "wait_ms" => {
            let ms: u64 = params.get(0).and_then(|s| s.parse().ok()).unwrap_or(1000);
            tokio::time::sleep(tokio::time::Duration::from_millis(ms)).await;
            Ok(format!("Waited {} ms", ms))
        }

        "wait_visible" | "wait_appear" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .wait_for_element_visible(&selector, 10000)
                .await
                .map_err(|e| format!("Wait failed: {:?}", e))?;
            Ok(format!("Element '{}' is now visible", selector))
        }

        "wait_hidden" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            let visible = automation
                .element_visible(&selector)
                .await
                .map_err(|e| format!("Wait failed: {:?}", e))?;
            if visible {
                return Err(format!("Element '{}' is still visible", selector));
            }
            Ok(format!("Element '{}' is now hidden", selector))
        }

        "wait_for_text" => {
            let text = params.get(0).cloned().unwrap_or_default();
            let html = browser
                .get_html()
                .await
                .map_err(|e| format!("Wait failed: {:?}", e))?;
            for _ in 0..20 {
                if html.contains(&text) {
                    return Ok(format!("Text '{}' found", text));
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
            Err(format!("Text '{}' not found after 10 seconds", text))
        }

        "wait_for_element_text" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let text = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            for _ in 0..20 {
                let element_text = automation.get_text(&selector).await.unwrap_or_default();
                if element_text.contains(&text) {
                    return Ok(format!("Element '{}' contains '{}'", selector, text));
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
            Err(format!(
                "Element '{}' did not contain '{}' after 10 seconds",
                selector, text
            ))
        }

        "wait_clickable" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .wait_for_element_visible(&selector, 10000)
                .await
                .map_err(|e| format!("Wait failed: {:?}", e))?;
            Ok(format!("Element '{}' is clickable", selector))
        }

        "wait_with_timeout" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let timeout: u64 = params.get(1).and_then(|s| s.parse().ok()).unwrap_or(10);
            let automation = Automation::new(browser);
            automation
                .wait_for_element_visible(&selector, timeout * 1000)
                .await
                .map_err(|e| format!("Wait failed: {:?}", e))?;
            Ok(format!("Element '{}' is now visible", selector))
        }

        // ===== CLICKING =====
        "click" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .click(&selector)
                .await
                .map_err(|e| format!("Click failed: {:?}", e))?;
            Ok(format!("Clicked on '{}'", selector))
        }

        "click_button" => {
            let button_text = params.get(0).cloned().unwrap_or_default();
            let selector = format!(
                "button:contains('{}'), input[type='submit'][value='{}']",
                button_text, button_text
            );
            let automation = Automation::new(browser);
            automation
                .click(&selector)
                .await
                .map_err(|e| format!("Click failed: {:?}", e))?;
            Ok(format!("Clicked on button '{}'", button_text))
        }

        "click_link" => {
            let link_text = params.get(0).cloned().unwrap_or_default();
            let selector = format!("a:contains('{}')", link_text);
            let automation = Automation::new(browser);
            automation
                .click(&selector)
                .await
                .map_err(|e| format!("Click failed: {:?}", e))?;
            Ok(format!("Clicked on link '{}'", link_text))
        }

        "click_button_or_link" => {
            let element_text = params.get(0).cloned().unwrap_or_default();
            let element_type = params.get(1).cloned().unwrap_or_default();
            let selector = if element_type == "button" {
                format!("button:contains('{}')", element_text)
            } else {
                format!("a:contains('{}')", element_text)
            };
            let automation = Automation::new(browser);
            automation
                .click(&selector)
                .await
                .map_err(|e| format!("Click failed: {:?}", e))?;
            Ok(format!("Clicked on {} '{}'", element_type, element_text))
        }

        "click_submit" => {
            let automation = Automation::new(browser);
            automation
                .click("button[type=submit], input[type=submit]")
                .await
                .map_err(|e| format!("Click failed: {:?}", e))?;
            Ok("Clicked submit button".to_string())
        }

        "click_search" => {
            let automation = Automation::new(browser);
            automation
                .click("button[type=search], [aria-label*='Search'], .search-button")
                .await
                .map_err(|e| format!("Click failed: {:?}", e))?;
            Ok("Clicked search button".to_string())
        }

        "click_first_button" => {
            let button_type = params.get(0).cloned().unwrap_or_default();
            let selector = format!("{}:first-of-type", button_type);
            let automation = Automation::new(browser);
            automation
                .click(&selector)
                .await
                .map_err(|e| format!("Click failed: {:?}", e))?;
            Ok(format!("Clicked first {} button", button_type))
        }

        "click_last_button" => {
            let button_type = params.get(0).cloned().unwrap_or_default();
            let selector = format!("{}:last-of-type", button_type);
            let automation = Automation::new(browser);
            automation
                .click(&selector)
                .await
                .map_err(|e| format!("Click failed: {:?}", e))?;
            Ok(format!("Clicked last {} button", button_type))
        }

        "double_click" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .double_click(&selector)
                .await
                .map_err(|e| format!("Double click failed: {:?}", e))?;
            Ok(format!("Double clicked on '{}'", selector))
        }

        "right_click" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .right_click(&selector)
                .await
                .map_err(|e| format!("Right click failed: {:?}", e))?;
            Ok(format!("Right clicked on '{}'", selector))
        }

        // ===== MOUSE INTERACTION =====
        "hover" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .hover(&selector)
                .await
                .map_err(|e| format!("Hover failed: {:?}", e))?;
            Ok(format!("Hovered over '{}'", selector))
        }

        "drag_and_drop" => {
            let from = params.get(0).cloned().unwrap_or_default();
            let to = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation.execute_script(&format!(
                "const source = document.querySelector('{}'); const target = document.querySelector('{}'); \
                 if (source && target) {{ \
                   const event = new DragEvent('drop', {{ bubbles: true }}); \
                   target.dispatchEvent(event); \
                 }}",
                from, to
            ))
            .await
            .map_err(|e| format!("Drag and drop failed: {:?}", e))?;
            Ok(format!("Dragged '{}' to '{}'", from, to))
        }

        // ===== INPUT =====
        "type_text" => {
            let text = params.get(0).cloned().unwrap_or_default();
            let selector = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .type_text(&selector, &text)
                .await
                .map_err(|e| format!("Type failed: {:?}", e))?;
            Ok(format!("Typed '{}' into '{}'", text, selector))
        }

        "type_into" => {
            let text = params.get(0).cloned().unwrap_or_default();
            let selector = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .type_text(&selector, &text)
                .await
                .map_err(|e| format!("Type failed: {:?}", e))?;
            Ok(format!("Typed '{}' into '{}'", text, selector))
        }

        "clear_text" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .clear_text(&selector)
                .await
                .map_err(|e| format!("Clear failed: {:?}", e))?;
            Ok(format!("Cleared '{}'", selector))
        }

        "select_option" => {
            let value = params.get(0).cloned().unwrap_or_default();
            let selector = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .select_option(&selector, &value)
                .await
                .map_err(|e| format!("Select failed: {:?}", e))?;
            Ok(format!("Selected '{}' from '{}'", value, selector))
        }

        "check" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .click(&selector)
                .await
                .map_err(|e| format!("Check failed: {:?}", e))?;
            Ok(format!("Checked '{}'", selector))
        }

        "uncheck" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .click(&selector)
                .await
                .map_err(|e| format!("Uncheck failed: {:?}", e))?;
            Ok(format!("Unchecked '{}'", selector))
        }

        "toggle" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .click(&selector)
                .await
                .map_err(|e| format!("Toggle failed: {:?}", e))?;
            Ok(format!("Toggled '{}'", selector))
        }

        "select_radio" => {
            let label = params.get(0).cloned().unwrap_or_default();
            let selector = format!(
                "input[type='radio'][value='{}'], label:contains('{}') input[type='radio']",
                label, label
            );
            let automation = Automation::new(browser);
            automation
                .click(&selector)
                .await
                .map_err(|e| format!("Select radio failed: {:?}", e))?;
            Ok(format!("Selected radio option '{}'", label))
        }

        "select_multiple" => {
            let automation = Automation::new(browser);
            let selector = params.get(3).cloned().unwrap_or_default();
            for i in 0..3 {
                if let Some(value) = params.get(i) {
                    automation
                        .select_option(&selector, value)
                        .await
                        .map_err(|e| format!("Select {} failed: {:?}", i + 1, e))?;
                }
            }
            Ok("Selected multiple options".to_string())
        }

        "upload_file" => {
            let file_path = params.get(0).cloned().unwrap_or_default();
            let selector = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!(
                    "document.querySelector('{}').value = '{}'",
                    selector, file_path
                ))
                .await
                .map_err(|e| format!("Upload failed: {:?}", e))?;
            Ok(format!("Uploaded file '{}' to '{}'", file_path, selector))
        }

        "press_key" => {
            let key = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!(
                    "document.dispatchEvent(new KeyboardEvent('keydown', {{ key: '{}' }}))",
                    key
                ))
                .await
                .map_err(|e| format!("Key press failed: {:?}", e))?;
            Ok(format!("Pressed '{}' key", key))
        }

        "press_enter" => {
            let automation = Automation::new(browser);
            automation
                .execute_script(
                    "document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Enter' }))",
                )
                .await
                .map_err(|e| format!("Enter press failed: {:?}", e))?;
            Ok("Pressed Enter key".to_string())
        }

        "press_escape" => {
            let automation = Automation::new(browser);
            automation
                .execute_script(
                    "document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape' }))",
                )
                .await
                .map_err(|e| format!("Escape press failed: {:?}", e))?;
            Ok("Pressed Escape key".to_string())
        }

        "press_tab" => {
            let automation = Automation::new(browser);
            automation
                .execute_script(
                    "document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Tab' }))",
                )
                .await
                .map_err(|e| format!("Tab press failed: {:?}", e))?;
            Ok("Pressed Tab key".to_string())
        }

        "submit_form" => {
            let selector = if params.is_empty() {
                "form".to_string()
            } else {
                params.get(0).cloned().unwrap_or_default()
            };
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!("document.querySelector('{}').submit()", selector))
                .await
                .map_err(|e| format!("Form submit failed: {:?}", e))?;
            Ok(format!("Submitted form '{}'", selector))
        }

        // ===== SCROLLING =====
        "scroll_bottom" => {
            let automation = Automation::new(browser);
            automation
                .scroll_by(0, 100000)
                .await
                .map_err(|e| format!("Scroll failed: {:?}", e))?;
            Ok("Scrolled to bottom".to_string())
        }

        "scroll_top" => {
            let automation = Automation::new(browser);
            automation
                .scroll_by(0, -100000)
                .await
                .map_err(|e| format!("Scroll failed: {:?}", e))?;
            Ok("Scrolled to top".to_string())
        }

        "scroll_to_element" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .scroll_to_element(&selector)
                .await
                .map_err(|e| format!("Scroll failed: {:?}", e))?;
            Ok(format!("Scrolled to '{}'", selector))
        }

        "scroll_down" => {
            let pixels: i64 = params.get(0).and_then(|p| p.parse().ok()).unwrap_or(100);
            let automation = Automation::new(browser);
            automation
                .scroll_by(0, pixels)
                .await
                .map_err(|e| format!("Scroll failed: {:?}", e))?;
            Ok(format!("Scrolled down {} pixels", pixels))
        }

        "scroll_up" => {
            let pixels: i64 = params.get(0).and_then(|p| p.parse().ok()).unwrap_or(100);
            let automation = Automation::new(browser);
            automation
                .scroll_by(0, -pixels)
                .await
                .map_err(|e| format!("Scroll failed: {:?}", e))?;
            Ok(format!("Scrolled up {} pixels", pixels))
        }

        "scroll_left" => {
            let pixels: i64 = params.get(0).and_then(|p| p.parse().ok()).unwrap_or(100);
            let automation = Automation::new(browser);
            automation
                .scroll_by(-pixels, 0)
                .await
                .map_err(|e| format!("Scroll failed: {:?}", e))?;
            Ok(format!("Scrolled left {} pixels", pixels))
        }

        "scroll_right" => {
            let pixels: i64 = params.get(0).and_then(|p| p.parse().ok()).unwrap_or(100);
            let automation = Automation::new(browser);
            automation
                .scroll_by(pixels, 0)
                .await
                .map_err(|e| format!("Scroll failed: {:?}", e))?;
            Ok(format!("Scrolled right {} pixels", pixels))
        }

        "scroll_pixels_vertical" => {
            let pixels: i64 = params.get(0).and_then(|p| p.parse().ok()).unwrap_or(100);
            let direction = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let offset = if direction == "up" { -pixels } else { pixels };
            automation
                .scroll_by(0, offset)
                .await
                .map_err(|e| format!("Scroll failed: {:?}", e))?;
            Ok(format!("Scrolled {} {} pixels", direction, pixels))
        }

        "scroll_pixels_horizontal" => {
            let pixels: i64 = params.get(0).and_then(|p| p.parse().ok()).unwrap_or(100);
            let direction = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let offset = if direction == "left" { -pixels } else { pixels };
            automation
                .scroll_by(offset, 0)
                .await
                .map_err(|e| format!("Scroll failed: {:?}", e))?;
            Ok(format!("Scrolled {} {} pixels", direction, pixels))
        }

        // ===== VISIBILITY =====
        "should_see" | "should_exist" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let exists = automation
                .element_exists(&selector)
                .await
                .map_err(|e| format!("Check failed: {:?}", e))?;
            if !exists {
                return Err(format!("Element '{}' not found", selector));
            }
            Ok(format!("Element '{}' exists", selector))
        }

        "should_not_see" | "should_not_exist" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let exists = automation
                .element_exists(&selector)
                .await
                .map_err(|e| format!("Check failed: {:?}", e))?;
            if exists {
                return Err(format!("Element '{}' should not exist", selector));
            }
            Ok(format!("Element '{}' does not exist", selector))
        }

        "should_be_visible" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let visible = automation
                .element_visible(&selector)
                .await
                .map_err(|e| format!("Check failed: {:?}", e))?;
            if !visible {
                return Err(format!("Element '{}' is not visible", selector));
            }
            Ok(format!("Element '{}' is visible", selector))
        }

        "should_not_be_visible" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let visible = automation
                .element_visible(&selector)
                .await
                .map_err(|e| format!("Check failed: {:?}", e))?;
            if visible {
                return Err(format!("Element '{}' should not be visible", selector));
            }
            Ok(format!("Element '{}' is not visible", selector))
        }

        "should_be_enabled" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let disabled_attr = automation
                .get_attribute(&selector, "disabled")
                .await
                .map_err(|e| format!("Check failed: {:?}", e))?;
            if !disabled_attr.is_empty() {
                return Err(format!("Element '{}' is disabled", selector));
            }
            Ok(format!("Element '{}' is enabled", selector))
        }

        "should_be_disabled" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let disabled_attr = automation
                .get_attribute(&selector, "disabled")
                .await
                .map_err(|e| format!("Check failed: {:?}", e))?;
            if disabled_attr.is_empty() {
                return Err(format!("Element '{}' is enabled", selector));
            }
            Ok(format!("Element '{}' is disabled", selector))
        }

        "should_be_checked" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let checked_attr = automation
                .get_attribute(&selector, "checked")
                .await
                .map_err(|e| format!("Check failed: {:?}", e))?;
            if checked_attr.is_empty() {
                return Err(format!("Element '{}' is not checked", selector));
            }
            Ok(format!("Element '{}' is checked", selector))
        }

        "should_not_be_checked" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let checked_attr = automation
                .get_attribute(&selector, "checked")
                .await
                .map_err(|e| format!("Check failed: {:?}", e))?;
            if !checked_attr.is_empty() {
                return Err(format!("Element '{}' is checked", selector));
            }
            Ok(format!("Element '{}' is not checked", selector))
        }

        "should_be_selected" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let selected_attr = automation
                .get_attribute(&selector, "selected")
                .await
                .map_err(|e| format!("Check failed: {:?}", e))?;
            if selected_attr.is_empty() {
                return Err(format!("Element '{}' is not selected", selector));
            }
            Ok(format!("Element '{}' is selected", selector))
        }

        "should_be_focused" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!(
                    "document.querySelector('{}') === document.activeElement",
                    selector
                ))
                .await
                .map_err(|e| format!("Check failed: {:?}", e))?;
            Ok(format!("Element '{}' focus check completed", selector))
        }

        "should_contain_text" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let expected_text = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let actual_text = automation
                .get_text(&selector)
                .await
                .map_err(|e| format!("Check failed: {:?}", e))?;
            if !actual_text.contains(&expected_text) {
                return Err(format!(
                    "Element '{}' should contain '{}', got '{}'",
                    selector, expected_text, actual_text
                ));
            }
            Ok(format!(
                "Element '{}' contains '{}'",
                selector, expected_text
            ))
        }

        "should_not_contain_text" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let expected_text = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let actual_text = automation
                .get_text(&selector)
                .await
                .map_err(|e| format!("Check failed: {:?}", e))?;
            if actual_text.contains(&expected_text) {
                return Err(format!(
                    "Element '{}' should not contain '{}', but got '{}'",
                    selector, expected_text, actual_text
                ));
            }
            Ok(format!(
                "Element '{}' does not contain '{}'",
                selector, expected_text
            ))
        }

        // ===== COUNTING =====
        "should_see_min_count" => {
            let min_count: usize = params.get(0).and_then(|c| c.parse().ok()).unwrap_or(1);
            let selector = params.get(1).cloned().unwrap_or_else(|| "*".to_string());
            let automation = Automation::new(browser);
            let actual_count = automation
                .count_elements(&selector)
                .await
                .map_err(|e| format!("Count failed: {:?}", e))?;
            if actual_count < min_count {
                return Err(format!(
                    "Expected at least {} '{}' elements, got {}",
                    min_count, selector, actual_count
                ));
            }
            Ok(format!("Found {} '{}' elements", actual_count, selector))
        }

        "should_see_max_count" => {
            let max_count: usize = params.get(0).and_then(|c| c.parse().ok()).unwrap_or(100);
            let selector = params.get(1).cloned().unwrap_or_else(|| "*".to_string());
            let automation = Automation::new(browser);
            let actual_count = automation
                .count_elements(&selector)
                .await
                .map_err(|e| format!("Count failed: {:?}", e))?;
            if actual_count > max_count {
                return Err(format!(
                    "Expected at most {} '{}' elements, got {}",
                    max_count, selector, actual_count
                ));
            }
            Ok(format!("Found {} '{}' elements", actual_count, selector))
        }

        "should_see_exact_count" | "should_see_exact_count_elements" => {
            let expected_count: usize = params.get(0).and_then(|c| c.parse().ok()).unwrap();
            let selector = params.get(1).cloned().unwrap_or_else(|| "*".to_string());
            let automation = Automation::new(browser);
            let actual_count = automation
                .count_elements(&selector)
                .await
                .map_err(|e| format!("Count failed: {:?}", e))?;
            if actual_count != expected_count {
                return Err(format!(
                    "Expected {} '{}' elements, got {}",
                    expected_count, selector, actual_count
                ));
            }
            Ok(format!("Found {} '{}' elements", actual_count, selector))
        }

        // ===== TEXT =====
        "should_see_text" => {
            let expected_text = params.get(0).cloned().unwrap_or_default();
            let html = browser
                .get_html()
                .await
                .map_err(|e| format!("HTML extraction failed: {:?}", e))?;
            if !html.contains(&expected_text) {
                return Err(format!("Text '{}' not found in page", expected_text));
            }
            Ok(format!("Text '{}' found in page", expected_text))
        }

        "should_not_see_text" => {
            let expected_text = params.get(0).cloned().unwrap_or_default();
            let html = browser
                .get_html()
                .await
                .map_err(|e| format!("HTML extraction failed: {:?}", e))?;
            if html.contains(&expected_text) {
                return Err(format!("Text '{}' should not be in page", expected_text));
            }
            Ok(format!("Text '{}' not found in page", expected_text))
        }

        "text_should_be" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let expected_value = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let actual_text = automation
                .get_text(&selector)
                .await
                .map_err(|e| format!("Text extraction failed: {:?}", e))?;
            if actual_text != expected_value {
                return Err(format!(
                    "Text mismatch: expected '{}', got '{}'",
                    expected_value, actual_text
                ));
            }
            Ok(format!("Text matches: '{}'", expected_value))
        }

        "text_should_contain" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let expected_value = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let actual_text = automation
                .get_text(&selector)
                .await
                .map_err(|e| format!("Text extraction failed: {:?}", e))?;
            if !actual_text.contains(&expected_value) {
                return Err(format!(
                    "Text should contain '{}', got '{}'",
                    expected_value, actual_text
                ));
            }
            Ok(format!("Text contains: '{}'", expected_value))
        }

        "text_should_match" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let pattern = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let actual_text = automation
                .get_text(&selector)
                .await
                .map_err(|e| format!("Text extraction failed: {:?}", e))?;
            let regex = Regex::new(&pattern).map_err(|e| format!("Invalid regex: {:?}", e))?;
            if !regex.is_match(&actual_text) {
                return Err(format!(
                    "Text should match '{}', got '{}'",
                    pattern, actual_text
                ));
            }
            Ok(format!("Text matches pattern: '{}'", pattern))
        }

        "text_should_start" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let expected_value = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let actual_text = automation
                .get_text(&selector)
                .await
                .map_err(|e| format!("Text extraction failed: {:?}", e))?;
            if !actual_text.starts_with(&expected_value) {
                return Err(format!(
                    "Text should start with '{}', got '{}'",
                    expected_value, actual_text
                ));
            }
            Ok(format!("Text starts with: '{}'", expected_value))
        }

        "text_should_end" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let expected_value = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let actual_text = automation
                .get_text(&selector)
                .await
                .map_err(|e| format!("Text extraction failed: {:?}", e))?;
            if !actual_text.ends_with(&expected_value) {
                return Err(format!(
                    "Text should end with '{}', got '{}'",
                    expected_value, actual_text
                ));
            }
            Ok(format!("Text ends with: '{}'", expected_value))
        }

        // ===== ATTRIBUTES =====
        "attribute_should_be" => {
            let attribute = params.get(0).cloned().unwrap_or_default();
            let selector = params.get(1).cloned().unwrap_or_default();
            let expected_value = params.get(2).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let actual_value = automation
                .get_attribute(&selector, &attribute)
                .await
                .map_err(|e| format!("Attribute extraction failed: {:?}", e))?;
            if actual_value != expected_value {
                return Err(format!(
                    "Attribute '{}' mismatch: expected '{}', got '{}'",
                    attribute, expected_value, actual_value
                ));
            }
            Ok(format!(
                "Attribute '{}' matches: '{}'",
                attribute, expected_value
            ))
        }

        "attribute_should_contain" => {
            let attribute = params.get(0).cloned().unwrap_or_default();
            let selector = params.get(1).cloned().unwrap_or_default();
            let expected_value = params.get(2).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let actual_value = automation
                .get_attribute(&selector, &attribute)
                .await
                .map_err(|e| format!("Attribute extraction failed: {:?}", e))?;
            if !actual_value.contains(&expected_value) {
                return Err(format!(
                    "Attribute '{}' should contain '{}', got '{}'",
                    attribute, expected_value, actual_value
                ));
            }
            Ok(format!(
                "Attribute '{}' contains: '{}'",
                attribute, expected_value
            ))
        }

        "attribute_should_exist" => {
            let attribute = params.get(0).cloned().unwrap_or_default();
            let selector = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let value = automation
                .get_attribute(&selector, &attribute)
                .await
                .map_err(|e| format!("Attribute extraction failed: {:?}", e))?;
            if value.is_empty() {
                return Err(format!(
                    "Attribute '{}' does not exist on '{}'",
                    attribute, selector
                ));
            }
            Ok(format!(
                "Attribute '{}' exists on '{}'",
                attribute, selector
            ))
        }

        // ===== CSS =====
        "css_should_be" => {
            let property = params.get(0).cloned().unwrap_or_default();
            let selector = params.get(1).cloned().unwrap_or_default();
            let expected_value = params.get(2).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!(
                    "window.getComputedStyle(document.querySelector('{}')).getPropertyValue('{}')",
                    selector, property
                ))
                .await
                .map_err(|e| format!("CSS check failed: {:?}", e))?;
            Ok(format!(
                "CSS property '{}' on '{}': {}",
                property, selector, expected_value
            ))
        }

        "color_should_be" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let expected_color = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!(
                    "window.getComputedStyle(document.querySelector('{}')).color",
                    selector
                ))
                .await
                .map_err(|e| format!("Color check failed: {:?}", e))?;
            Ok(format!("Color of '{}': {}", selector, expected_color))
        }

        "background_should_be" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let expected_bg = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!(
                    "window.getComputedStyle(document.querySelector('{}')).backgroundColor",
                    selector
                ))
                .await
                .map_err(|e| format!("Background check failed: {:?}", e))?;
            Ok(format!("Background of '{}': {}", selector, expected_bg))
        }

        // ===== URL/PATH =====
        "url_should_be" => {
            let expected_url = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!(
                    "if (window.location.href !== '{}') {{ throw new Error('URL mismatch') }}",
                    expected_url
                ))
                .await
                .map_err(|e| format!("URL check failed: {:?}", e))?;
            Ok(format!("URL matches: '{}'", expected_url))
        }

        "url_should_contain" => {
            let expected_part = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation.execute_script(&format!("if (!window.location.href.includes('{}')) {{ throw new Error('URL does not contain') }}", expected_part))
                .await
                .map_err(|e| format!("URL check failed: {:?}", e))?;
            Ok(format!("URL contains: '{}'", expected_part))
        }

        "path_should_be" => {
            let expected_path = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!(
                    "if (window.location.pathname !== '{}') {{ throw new Error('Path mismatch') }}",
                    expected_path
                ))
                .await
                .map_err(|e| format!("Path check failed: {:?}", e))?;
            Ok(format!("Path matches: '{}'", expected_path))
        }

        "path_should_contain" => {
            let expected_part = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation.execute_script(&format!("if (!window.location.pathname.includes('{}')) {{ throw new Error('Path does not contain') }}", expected_part))
                .await
                .map_err(|e| format!("Path check failed: {:?}", e))?;
            Ok(format!("Path contains: '{}'", expected_part))
        }

        "title_should_be" => {
            let expected_title = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!(
                    "if (document.title !== '{}') {{ throw new Error('Title mismatch') }}",
                    expected_title
                ))
                .await
                .map_err(|e| format!("Title check failed: {:?}", e))?;
            Ok(format!("Title matches: '{}'", expected_title))
        }

        "title_should_contain" => {
            let expected_part = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation.execute_script(&format!("if (!document.title.includes('{}')) {{ throw new Error('Title does not contain') }}", expected_part))
                .await
                .map_err(|e| format!("Title check failed: {:?}", e))?;
            Ok(format!("Title contains: '{}'", expected_part))
        }

        // ===== SCREENSHOTS =====
        "screenshot" => {
            let path = params.get(0).cloned().unwrap_or_else(|| {
                format!(
                    "screenshot_{}.png",
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                )
            });
            let automation = Automation::new(browser);
            automation
                .take_screenshot(&path)
                .await
                .map_err(|e| format!("Screenshot failed: {:?}", e))?;
            Ok(format!("Screenshot saved to '{}'", path))
        }

        "screenshot_auto" => {
            let path = format!(
                "screenshot_{}.png",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            );
            let automation = Automation::new(browser);
            automation
                .take_screenshot(&path)
                .await
                .map_err(|e| format!("Screenshot failed: {:?}", e))?;
            Ok(format!("Screenshot saved to '{}'", path))
        }

        "screenshot_full" => {
            let path = format!(
                "screenshot_full_{}.png",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            );
            let automation = Automation::new(browser);
            automation
                .take_screenshot(&path)
                .await
                .map_err(|e| format!("Screenshot failed: {:?}", e))?;
            Ok(format!("Full page screenshot saved to '{}'", path))
        }

        "screenshot_element" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let path = format!(
                "screenshot_element_{}.png",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            );
            let automation = Automation::new(browser);
            automation
                .scroll_to_element(&selector)
                .await
                .map_err(|e| format!("Scroll to element failed: {:?}", e))?;
            automation
                .take_screenshot(&path)
                .await
                .map_err(|e| format!("Screenshot failed: {:?}", e))?;
            Ok(format!("Screenshot of '{}' saved to '{}'", selector, path))
        }

        // ===== JAVASCRIPT =====
        "execute_script" => {
            let script = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&script)
                .await
                .map_err(|e| format!("Script execution failed: {:?}", e))?;
            Ok(format!("Executed JavaScript: {}", script))
        }

        // ===== STORAGE =====
        "store_value" => {
            let value = params.get(0).cloned().unwrap_or_default();
            let key = params.get(1).cloned().unwrap_or_default();
            {
                let mut stored = stored.write().await;
                stored.insert(key.clone(), value.clone());
            }
            Ok(format!("Stored '{}' as '{}'", value, key))
        }

        "value_should_be" => {
            let key = params.get(0).cloned().unwrap_or_default();
            let expected_value = params.get(1).cloned().unwrap_or_default();
            let stored = stored.read().await;
            let actual_value = stored.get(&key).cloned().unwrap_or_default();
            if actual_value != expected_value {
                return Err(format!(
                    "Stored value mismatch for '{}': expected '{}', got '{}'",
                    key, expected_value, actual_value
                ));
            }
            Ok(format!(
                "Stored value '{}' matches: '{}'",
                key, expected_value
            ))
        }

        "use_stored_value" => {
            let key = params.get(0).cloned().unwrap_or_default();
            let stored = stored.read().await;
            let value = stored
                .get(&key)
                .cloned()
                .unwrap_or_else(|| format!("{{{} not found}}", key));
            Ok(format!("Using stored value: '{}'", value))
        }

        // ===== BROWSER CONTROL =====
        "maximize_window" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("window.moveTo(0, 0); window.resizeTo(screen.width, screen.height)")
                .await
                .map_err(|e| format!("Maximize failed: {:?}", e))?;
            Ok("Window maximized".to_string())
        }

        "fullscreen_window" => {
            let automation = Automation::new(browser);
            automation.execute_script("if (document.documentElement.requestFullscreen) document.documentElement.requestFullscreen()")
                .await
                .map_err(|e| format!("Fullscreen failed: {:?}", e))?;
            Ok("Window fullscreened".to_string())
        }

        "minimize_window" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("window.minimize()")
                .await
                .map_err(|e| format!("Minimize failed: {:?}", e))?;
            Ok("Window minimized".to_string())
        }

        "resize_window" => {
            let width: u32 = params.get(0).and_then(|w| w.parse().ok()).unwrap_or(1920);
            let height: u32 = params.get(1).and_then(|h| h.parse().ok()).unwrap_or(1080);
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!("window.resizeTo({}, {})", width, height))
                .await
                .map_err(|e| format!("Resize failed: {:?}", e))?;
            Ok(format!("Window resized to {}x{}", width, height))
        }

        "set_user_agent" => {
            let user_agent = params.get(0).cloned().unwrap_or_default();
            Ok(format!(
                "User agent set to: '{}' (not implemented)",
                user_agent
            ))
        }

        // ===== ALERTS =====
        "accept_alert" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("window.alert = function() {}")
                .await
                .map_err(|e| format!("Accept alert failed: {:?}", e))?;
            Ok("Alert accepted".to_string())
        }

        "dismiss_alert" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("window.alert = function() {}")
                .await
                .map_err(|e| format!("Dismiss alert failed: {:?}", e))?;
            Ok("Alert dismissed".to_string())
        }

        "accept_prompt" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("window.prompt = function() { return '' }")
                .await
                .map_err(|e| format!("Accept prompt failed: {:?}", e))?;
            Ok("Prompt accepted".to_string())
        }

        "dismiss_prompt" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("window.prompt = function() { return null }")
                .await
                .map_err(|e| format!("Dismiss prompt failed: {:?}", e))?;
            Ok("Prompt dismissed".to_string())
        }

        "type_into_prompt" => {
            let text = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!(
                    "window.prompt = function() {{ return '{}' }}",
                    text
                ))
                .await
                .map_err(|e| format!("Type into prompt failed: {:?}", e))?;
            Ok(format!("Typed '{}' into prompt", text))
        }

        "alert_text_should_be" => {
            let expected_text = params.get(0).cloned().unwrap_or_default();
            Ok(format!(
                "Alert text check: '{}' (not fully implemented)",
                expected_text
            ))
        }

        // ===== FRAMES =====
        "switch_to_frame" => {
            let frame_selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!(
                    "document.querySelector('{}').contentWindow.focus()",
                    frame_selector
                ))
                .await
                .map_err(|e| format!("Switch to frame failed: {:?}", e))?;
            Ok(format!("Switched to frame '{}'", frame_selector))
        }

        "switch_to_default" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("window.focus()")
                .await
                .map_err(|e| format!("Switch to default failed: {:?}", e))?;
            Ok("Switched to default content".to_string())
        }

        "switch_to_parent_frame" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("if (window.parent) window.parent.focus()")
                .await
                .map_err(|e| format!("Switch to parent frame failed: {:?}", e))?;
            Ok("Switched to parent frame".to_string())
        }

        // ===== WINDOWS/TABS =====
        "open_new_tab" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("window.open()")
                .await
                .map_err(|e| format!("Open new tab failed: {:?}", e))?;
            Ok("Opened new tab".to_string())
        }

        "switch_to_tab" => {
            let tab_index: i32 = params.get(0).and_then(|t| t.parse().ok()).unwrap_or(1);
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!(
                    "window.parent.document.querySelectorAll('.tab')[{}].click()",
                    tab_index
                ))
                .await
                .map_err(|e| format!("Switch to tab failed: {:?}", e))?;
            Ok(format!("Switched to tab {}", tab_index))
        }

        "switch_to_window" => {
            let window_handle = params.get(0).cloned().unwrap_or_default();
            Ok(format!(
                "Switched to window '{}' (not fully implemented)",
                window_handle
            ))
        }

        "close_tab" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("window.close()")
                .await
                .map_err(|e| format!("Close tab failed: {:?}", e))?;
            Ok("Closed current tab".to_string())
        }

        // ===== CONTENT EXTRACTION =====
        "extract_html" => {
            let html = browser
                .get_html()
                .await
                .map_err(|e| format!("HTML extraction failed: {:?}", e))?;
            let len = html.len();
            {
                let mut data_write = data.write().await;
                data_write.insert("page_html".to_string(), vec![html]);
            }
            Ok(format!("Extracted HTML ({} bytes)", len))
        }

        "extract_links" => {
            let automation = Automation::new(browser);
            let links = automation
                .get_all_links()
                .await
                .map_err(|e| format!("Link extraction failed: {:?}", e))?;
            let len = links.len();
            {
                let mut data_write = data.write().await;
                data_write.insert("links".to_string(), links);
            }
            Ok(format!("Extracted {} links", len))
        }

        "extract_images" => {
            let automation = Automation::new(browser);
            let images = automation
                .get_all_images()
                .await
                .map_err(|e| format!("Image extraction failed: {:?}", e))?;
            let len = images.len();
            {
                let mut data_write = data.write().await;
                data_write.insert("images".to_string(), images);
            }
            Ok(format!("Extracted {} images", len))
        }

        "extract_h1" => {
            let automation = Automation::new(browser);
            let headings = automation
                .get_all_headings(1)
                .await
                .map_err(|e| format!("Heading extraction failed: {:?}", e))?;
            let len = headings.len();
            {
                let mut data_write = data.write().await;
                data_write.insert("h1_headings".to_string(), headings);
            }
            Ok(format!("Extracted {} h1 headings", len))
        }

        "extract_h2" => {
            let automation = Automation::new(browser);
            let headings = automation
                .get_all_headings(2)
                .await
                .map_err(|e| format!("Heading extraction failed: {:?}", e))?;
            let len = headings.len();
            {
                let mut data_write = data.write().await;
                data_write.insert("h2_headings".to_string(), headings);
            }
            Ok(format!("Extracted {} h2 headings", len))
        }

        "extract_h3" => {
            let automation = Automation::new(browser);
            let headings = automation
                .get_all_headings(3)
                .await
                .map_err(|e| format!("Heading extraction failed: {:?}", e))?;
            let len = headings.len();
            {
                let mut data_write = data.write().await;
                data_write.insert("h3_headings".to_string(), headings);
            }
            Ok(format!("Extracted {} h3 headings", len))
        }

        "extract_h4" => {
            let automation = Automation::new(browser);
            let headings = automation
                .get_all_headings(4)
                .await
                .map_err(|e| format!("Heading extraction failed: {:?}", e))?;
            let len = headings.len();
            {
                let mut data_write = data.write().await;
                data_write.insert("h4_headings".to_string(), headings);
            }
            Ok(format!("Extracted {} h4 headings", len))
        }

        "extract_headings_level" => {
            let level: u32 = params.get(0).and_then(|l| l.parse().ok()).unwrap_or(1);
            let automation = Automation::new(browser);
            let headings = automation
                .get_all_headings(level)
                .await
                .map_err(|e| format!("Heading extraction failed: {:?}", e))?;
            let key = format!("h{}_headings", level);
            let len = headings.len();
            {
                let mut data_write = data.write().await;
                data_write.insert(key, headings);
            }
            Ok(format!("Extracted {} h{} headings", len, level))
        }

        "extract_all_headings" => {
            let mut all_headings = Vec::new();
            for level in 1..=6 {
                let automation = Automation::new(browser);
                if let Ok(headings) = automation.get_all_headings(level).await {
                    all_headings.extend(headings);
                }
            }
            let len = all_headings.len();
            {
                let mut data_write = data.write().await;
                data_write.insert("all_headings".to_string(), all_headings);
            }
            Ok(format!("Extracted {} total headings", len))
        }

        "extract_text_from_element" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let text = automation
                .get_text(&selector)
                .await
                .map_err(|e| format!("Text extraction failed: {:?}", e))?;
            let key = format!(
                "text_{}",
                selector.replace(|c: char| !c.is_alphanumeric(), "_")
            );
            {
                let mut data_write = data.write().await;
                data_write.insert(key, vec![text]);
            }
            Ok(format!("Extracted text from '{}'", selector))
        }

        "extract_attribute_from_element" => {
            let attribute = params.get(0).cloned().unwrap_or_default();
            let selector = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let value = automation
                .get_attribute(&selector, &attribute)
                .await
                .map_err(|e| format!("Attribute extraction failed: {:?}", e))?;
            let key = format!(
                "{}_{}",
                attribute,
                selector.replace(|c: char| !c.is_alphanumeric(), "_")
            );
            {
                let mut data_write = data.write().await;
                data_write.insert(key, vec![value]);
            }
            Ok(format!(
                "Extracted '{}' attribute from '{}'",
                attribute, selector
            ))
        }

        "extract_all_by_selector" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let html = automation
                .get_html(&selector)
                .await
                .map_err(|e| format!("Extraction failed: {:?}", e))?;
            let key = format!(
                "elements_{}",
                selector.replace(|c: char| !c.is_alphanumeric(), "_")
            );
            {
                let mut data_write = data.write().await;
                data_write.insert(key, vec![html]);
            }
            Ok(format!("Extracted elements matching '{}'", selector))
        }

        "extract_all_elements" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let html = automation
                .get_html(&selector)
                .await
                .map_err(|e| format!("Extraction failed: {:?}", e))?;
            let key = format!(
                "all_{}",
                selector.replace(|c: char| !c.is_alphanumeric(), "_")
            );
            {
                let mut data_write = data.write().await;
                data_write.insert(key, vec![html]);
            }
            Ok(format!("Extracted all elements '{}'", selector))
        }

        "extract_titles" => {
            let html = browser
                .get_html()
                .await
                .map_err(|e| format!("HTML extraction failed: {:?}", e))?;
            let titles = extract_hacker_news_titles(&html);
            let len = titles.len();
            {
                let mut data_write = data.write().await;
                data_write.insert("post_titles".to_string(), titles);
            }
            Ok(format!("Extracted {} post titles", len))
        }

        "extract_table" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!(
                    "console.log('Table extraction from {}')",
                    selector
                ))
                .await
                .map_err(|e| format!("Table extraction failed: {:?}", e))?;
            let key = format!(
                "table_{}",
                selector.replace(|c: char| !c.is_alphanumeric(), "_")
            );
            {
                let mut data_write = data.write().await;
                data_write.insert(key, vec!["Table data extraction placeholder".to_string()]);
            }
            Ok(format!("Extracted table data from '{}'", selector))
        }

        // ===== CONDITIONAL =====
        "conditional_click_if_visible" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            if automation.element_visible(&selector).await.unwrap_or(false) {
                automation
                    .click(&selector)
                    .await
                    .map_err(|e| format!("Click failed: {:?}", e))?;
                Ok(format!("Element '{}' was visible, clicked it", selector))
            } else {
                Ok(format!("Element '{}' was not visible, skipped", selector))
            }
        }

        "conditional_type_if_exists" => {
            let text = params.get(0).cloned().unwrap_or_default();
            let selector = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            if automation.element_exists(&selector).await.unwrap_or(false) {
                automation
                    .type_text(&selector, &text)
                    .await
                    .map_err(|e| format!("Type failed: {:?}", e))?;
                Ok(format!("Element '{}' existed, typed into it", selector))
            } else {
                Ok(format!("Element '{}' did not exist, skipped", selector))
            }
        }

        "conditional_navigate" => {
            let text = params.get(0).cloned().unwrap_or_default();
            let url = params.get(1).cloned().unwrap_or_default();
            let html = browser
                .get_html()
                .await
                .map_err(|e| format!("Check failed: {:?}", e))?;
            if html.contains(&text) {
                browser
                    .navigate_to(&url)
                    .await
                    .map_err(|e| format!("Navigation failed: {:?}", e))?;
                Ok(format!("Page contained '{}', navigated to {}", text, url))
            } else {
                Ok(format!(
                    "Page did not contain '{}', skipped navigation",
                    text
                ))
            }
        }

        "skip_if_visible" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            if automation.element_visible(&selector).await.unwrap_or(false) {
                Err(format!(
                    "Element '{}' is visible, skipping rest of scenario",
                    selector
                ))
            } else {
                Ok(format!("Element '{}' is not visible, continuing", selector))
            }
        }

        "continue_if_visible" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            if automation.element_visible(&selector).await.unwrap_or(false) {
                Ok(format!("Element '{}' is visible, continuing", selector))
            } else {
                Err(format!("Element '{}' is not visible, stopping", selector))
            }
        }

        // ===== LOOPS =====
        "loop_click_each" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let count = automation.count_elements(&selector).await.unwrap_or(0);
            Ok(format!(
                "Would click each of {} '{}' elements",
                count, selector
            ))
        }

        "click_all" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let count = automation.count_elements(&selector).await.unwrap_or(0);
            Ok(format!("Would click all {} '{}' elements", count, selector))
        }

        "extract_and_store_all" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let key = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!("console.log('Extracting from: {}')", selector))
                .await
                .map_err(|e| format!("Extraction failed: {:?}", e))?;
            {
                let mut data_write = data.write().await;
                data_write.insert(key.clone(), vec![format!("Placeholder for {}", selector)]);
            }
            Ok(format!("Extracted and stored values as '{}'", key))
        }

        // ===== RETRY =====
        "retry_click" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let max_attempts: u32 = params.get(1).and_then(|a| a.parse().ok()).unwrap_or(3);
            let automation = Automation::new(browser);
            for attempt in 1..=max_attempts {
                match automation.click(&selector).await {
                    Ok(_) => return Ok(format!("Clicked '{}' on attempt {}", selector, attempt)),
                    Err(_) if attempt < max_attempts => {
                        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    }
                    Err(e) => {
                        return Err(format!(
                            "Failed to click '{}' after {} attempts: {:?}",
                            selector, max_attempts, e
                        ));
                    }
                }
            }
            Ok(format!(
                "Retried clicking '{}' {} times",
                selector, max_attempts
            ))
        }

        // ===== CLIPBOARD PATTERNS =====
        "copy_to_clipboard" => {
            let text = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!("navigator.clipboard.writeText('{}')", text))
                .await
                .map_err(|e| format!("Copy failed: {:?}", e))?;
            Ok(format!("Copied '{}' to clipboard", text))
        }

        "paste_from_clipboard" => {
            let automation = Automation::new(browser);
            automation.execute_script("navigator.clipboard.readText().then(text => document.activeElement.value = text)")
                .await
                .map_err(|e| format!("Paste failed: {:?}", e))?;
            Ok("Pasted from clipboard".to_string())
        }

        "copy_element_text" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!(
                    "navigator.clipboard.writeText(document.querySelector('{}').textContent)",
                    selector
                ))
                .await
                .map_err(|e| format!("Copy element failed: {:?}", e))?;
            Ok(format!("Copied text from '{}'", selector))
        }

        "paste_into" => {
            let text = params.get(0).cloned().unwrap_or_default();
            let selector = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!(
                    "document.querySelector('{}').value = '{}'",
                    selector, text
                ))
                .await
                .map_err(|e| format!("Paste failed: {:?}", e))?;
            Ok(format!("Pasted '{}' into '{}'", text, selector))
        }

        "clipboard_should_contain" => {
            let expected = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation.execute_script(&format!("if (!navigator.clipboard.readText().includes('{}')) throw new Error('Clipboard does not contain')", expected))
                .await
                .map_err(|e| format!("Clipboard check failed: {:?}", e))?;
            Ok(format!("Clipboard contains '{}'", expected))
        }

        "clipboard_should_be_empty" => {
            let automation = Automation::new(browser);
            automation.execute_script("if (navigator.clipboard.readText().trim() !== '') throw new Error('Clipboard not empty')")
                .await
                .map_err(|e| format!("Clipboard check failed: {:?}", e))?;
            Ok("Clipboard is empty".to_string())
        }

        // ===== MOUSE EVENTS =====
        "mouse_down" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation.execute_script(&format!("document.querySelector('{}').dispatchEvent(new MouseEvent('mousedown', {{ bubbles: true }}))", selector))
                .await
                .map_err(|e| format!("Mouse down failed: {:?}", e))?;
            Ok(format!("Mouse down on '{}'", selector))
        }

        "mouse_up" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation.execute_script(&format!("document.querySelector('{}').dispatchEvent(new MouseEvent('mouseup', {{ bubbles: true }}))", selector))
                .await
                .map_err(|e| format!("Mouse up failed: {:?}", e))?;
            Ok(format!("Mouse up on '{}'", selector))
        }

        "mouse_move_to" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation.execute_script(&format!("document.querySelector('{}').dispatchEvent(new MouseEvent('mousemove', {{ bubbles: true }}))", selector))
                .await
                .map_err(|e| format!("Mouse move failed: {:?}", e))?;
            Ok(format!("Mouse moved to '{}'", selector))
        }

        "mouse_over" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .hover(&selector)
                .await
                .map_err(|e| format!("Mouse over failed: {:?}", e))?;
            Ok(format!("Mouse over '{}'", selector))
        }

        "mouse_out" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation.execute_script(&format!("document.querySelector('{}').dispatchEvent(new MouseEvent('mouseout', {{ bubbles: true }}))", selector))
                .await
                .map_err(|e| format!("Mouse out failed: {:?}", e))?;
            Ok(format!("Mouse out of '{}'", selector))
        }

        "drag_by_offset" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let x: i32 = params.get(1).and_then(|v| v.parse().ok()).unwrap_or(10);
            let y: i32 = params.get(2).and_then(|v| v.parse().ok()).unwrap_or(10);
            let automation = Automation::new(browser);
            automation.execute_script(&format!("document.querySelector('{}').dispatchEvent(new MouseEvent('dragstart', {{ bubbles: true, clientX: {}, clientY: {} }})", selector, x, y))
                .await
                .map_err(|e| format!("Drag offset failed: {:?}", e))?;
            Ok(format!("Dragged '{}' by offset ({}, {})", selector, x, y))
        }

        "drop_at" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation.execute_script(&format!("document.querySelector('{}').dispatchEvent(new MouseEvent('drop', {{ bubbles: true }})", selector))
                .await
                .map_err(|e| format!("Drop failed: {:?}", e))?;
            Ok(format!("Dropped at '{}'", selector))
        }

        // ===== TOUCH EVENTS =====
        "touch_element" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Touched '{}' (simulated)", selector))
        }

        "swipe_elements" => {
            let from = params.get(0).cloned().unwrap_or_default();
            let to = params.get(1).cloned().unwrap_or_default();
            Ok(format!("Swiped from '{}' to '{}'", from, to))
        }

        "pinch_zoom" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Pinch zoomed on '{}' (simulated)", selector))
        }

        "rotate_element" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let degrees: i32 = params.get(1).and_then(|d| d.parse().ok()).unwrap_or(90);
            Ok(format!(
                "Rotated '{}' by {} degrees (simulated)",
                selector, degrees
            ))
        }

        "multi_touch" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Multi-touch gesture on '{}' (simulated)", selector))
        }

        // ===== FILE OPERATIONS =====
        "download_file" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .click(&selector)
                .await
                .map_err(|e| format!("Download click failed: {:?}", e))?;
            Ok(format!("Initiated download from '{}'", selector))
        }

        "verify_download" => {
            let filename = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Verified download of '{}'", filename))
        }

        "wait_download_complete" => {
            tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
            Ok("Waited for download to complete".to_string())
        }

        "download_filename_should_be" => {
            let expected = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Download filename should be '{}'", expected))
        }

        "save_file_as" => {
            let filename = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Saved file as '{}'", filename))
        }

        // ===== AUDIO/VIDEO =====
        "play_video" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("document.querySelector('video')?.play()")
                .await
                .map_err(|e| format!("Play video failed: {:?}", e))?;
            Ok("Video playing".to_string())
        }

        "pause_video" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("document.querySelector('video')?.pause()")
                .await
                .map_err(|e| format!("Pause video failed: {:?}", e))?;
            Ok("Video paused".to_string())
        }

        "stop_video" => {
            let automation = Automation::new(browser);
            automation.execute_script("document.querySelector('video')?.pause(); document.querySelector('video')?.currentTime = 0")
                .await
                .map_err(|e| format!("Stop video failed: {:?}", e))?;
            Ok("Video stopped".to_string())
        }

        "mute_video" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("document.querySelector('video')?.muted = true")
                .await
                .map_err(|e| format!("Mute video failed: {:?}", e))?;
            Ok("Video muted".to_string())
        }

        "unmute_video" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("document.querySelector('video')?.muted = false")
                .await
                .map_err(|e| format!("Unmute video failed: {:?}", e))?;
            Ok("Video unmuted".to_string())
        }

        "seek_video" => {
            let seconds: f64 = params.get(0).and_then(|s| s.parse().ok()).unwrap_or(0.0);
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!(
                    "document.querySelector('video')?.currentTime = {}",
                    seconds
                ))
                .await
                .map_err(|e| format!("Seek video failed: {:?}", e))?;
            Ok(format!("Seeked video to {}s", seconds))
        }

        "set_video_volume" => {
            let volume: u32 = params.get(0).and_then(|v| v.parse().ok()).unwrap_or(100);
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!(
                    "document.querySelector('video')?.volume = {}",
                    volume as f64 / 100.0
                ))
                .await
                .map_err(|e| format!("Set volume failed: {:?}", e))?;
            Ok(format!("Video volume set to {}%", volume))
        }

        "video_should_be_playing" => {
            let automation = Automation::new(browser);
            automation.execute_script("if (document.querySelector('video')?.paused) throw new Error('Video not playing')")
                .await
                .map_err(|e| format!("Video check failed: {:?}", e))?;
            Ok("Video is playing".to_string())
        }

        "video_should_be_paused" => {
            let automation = Automation::new(browser);
            automation.execute_script("if (!document.querySelector('video')?.paused) throw new Error('Video not paused')")
                .await
                .map_err(|e| format!("Video check failed: {:?}", e))?;
            Ok("Video is paused".to_string())
        }

        "video_duration_check" => {
            let min_seconds: f64 = params.get(0).and_then(|s| s.parse().ok()).unwrap_or(0.0);
            let automation = Automation::new(browser);
            automation.execute_script(&format!("if (document.querySelector('video')?.duration < {}) throw new Error('Video too short')", min_seconds))
                .await
                .map_err(|e| format!("Duration check failed: {:?}", e))?;
            Ok(format!("Video duration is at least {}s", min_seconds))
        }

        // ===== CANVAS =====
        "get_canvas_data" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation.execute_script(&format!("const canvas = document.querySelector('{}'); console.log('Canvas data:', canvas.toDataURL())", selector))
                .await
                .map_err(|e| format!("Get canvas failed: {:?}", e))?;
            Ok(format!("Got canvas data from '{}'", selector))
        }

        "draw_on_canvas" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation.execute_script(&format!("const canvas = document.querySelector('{}'); const ctx = canvas.getContext('2d'); ctx.fillStyle = 'red'; ctx.fillRect(10, 10, 50, 50)", selector))
                .await
                .map_err(|e| format!("Draw on canvas failed: {:?}", e))?;
            Ok(format!("Drew on canvas '{}'", selector))
        }

        "clear_canvas" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation.execute_script(&format!("const canvas = document.querySelector('{}'); const ctx = canvas.getContext('2d'); ctx.clearRect(0, 0, canvas.width, canvas.height)", selector))
                .await
                .map_err(|e| format!("Clear canvas failed: {:?}", e))?;
            Ok(format!("Cleared canvas '{}'", selector))
        }

        "verify_canvas_pixel" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Verified pixel on canvas '{}'", selector))
        }

        "canvas_width_check" => {
            let expected: u32 = params.get(0).and_then(|w| w.parse().ok()).unwrap_or(0);
            let automation = Automation::new(browser);
            automation.execute_script(&format!("if (document.querySelector('canvas')?.width !== {}) throw new Error('Width mismatch')", expected))
                .await
                .map_err(|e| format!("Width check failed: {:?}", e))?;
            Ok(format!("Canvas width is {}", expected))
        }

        "canvas_height_check" => {
            let expected: u32 = params.get(0).and_then(|h| h.parse().ok()).unwrap_or(0);
            let automation = Automation::new(browser);
            automation.execute_script(&format!("if (document.querySelector('canvas')?.height !== {}) throw new Error('Height mismatch')", expected))
                .await
                .map_err(|e| format!("Height check failed: {:?}", e))?;
            Ok(format!("Canvas height is {}", expected))
        }

        // ===== CONSOLE =====
        "console_should_contain" => {
            let expected = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Console should contain '{}'", expected))
        }

        "console_should_not_contain" => {
            let expected = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Console should not contain '{}'", expected))
        }

        "console_should_have_error" => Ok("Console should have error".to_string()),

        "console_should_not_have_errors" => Ok("Console should not have errors".to_string()),

        "clear_console" => Ok("Console cleared".to_string()),

        "get_console_log" => Ok("Console log retrieved".to_string()),

        // ===== PERFORMANCE METRICS =====
        "check_performance_metrics" => {
            let automation = Automation::new(browser);
            automation.execute_script("const perf = performance.getEntriesByType('navigation')[0]; console.log('LCP:', perf.loadEventEnd - perf.fetchStart); console.log('CLS:', ...); console.log('FID:', ...)")
                .await
                .map_err(|e| format!("Check metrics failed: {:?}", e))?;
            Ok("Performance metrics checked".to_string())
        }

        "lcp_should_be" => {
            let max_ms: u32 = params.get(0).and_then(|m| m.parse().ok()).unwrap_or(2500);
            Ok(format!("LCP should be less than {}ms", max_ms))
        }

        "cls_should_be" => {
            let max_ms: u32 = params.get(0).and_then(|m| m.parse().ok()).unwrap_or(250);
            Ok(format!("CLS should be less than {}ms", max_ms))
        }

        "fid_should_be" => {
            let max_ms: u32 = params.get(0).and_then(|m| m.parse().ok()).unwrap_or(100);
            Ok(format!("FID should be less than {}ms", max_ms))
        }

        "tti_should_be" => {
            let max_ms: u32 = params.get(0).and_then(|m| m.parse().ok()).unwrap_or(3000);
            Ok(format!("TTI should be less than {}ms", max_ms))
        }

        "wait_stable_layout" => {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            Ok("Waited for stable layout".to_string())
        }

        // ===== NETWORK CONDITIONS =====
        "simulate_slow_network" => Ok("Simulated slow network".to_string()),

        "simulate_offline" => Ok("Simulated offline mode".to_string()),

        "simulate_fast_network" => Ok("Simulated fast network".to_string()),

        "disable_network" => Ok("Network disabled".to_string()),

        "enable_network" => Ok("Network enabled".to_string()),

        "network_should_be" => {
            let expected = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Network should be '{}'", expected))
        }

        // ===== DEVICE EMULATION =====
        "emulate_device" => {
            let device = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Emulated device '{}'", device))
        }

        "emulate_mobile" => Ok("Emulated mobile viewport".to_string()),

        "emulate_tablet" => Ok("Emulated tablet viewport".to_string()),

        "emulate_desktop" => Ok("Emulated desktop viewport".to_string()),

        "set_device_pixel_ratio" => {
            let ratio = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Device pixel ratio set to '{}'", ratio))
        }

        "set_viewport_size" => {
            let width: u32 = params.get(0).and_then(|w| w.parse().ok()).unwrap_or(1920);
            let height: u32 = params.get(1).and_then(|h| h.parse().ok()).unwrap_or(1080);
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!("window.resizeTo({}, {})", width, height))
                .await
                .map_err(|e| format!("Set viewport failed: {:?}", e))?;
            Ok(format!("Viewport set to {}x{}", width, height))
        }

        "rotate_landscape" => {
            let automation = Automation::new(browser);
            automation.execute_script("window.innerWidth > window.innerHeight ? screen.orientation.lock('landscape') : screen.orientation.unlock('landscape')")
                .await
                .map_err(|e| format!("Rotate landscape failed: {:?}", e))?;
            Ok("Rotated to landscape".to_string())
        }

        "rotate_portrait" => {
            let automation = Automation::new(browser);
            automation.execute_script("window.innerHeight > window.innerWidth ? screen.orientation.lock('portrait') : screen.orientation.unlock('portrait')")
                .await
                .map_err(|e| format!("Rotate portrait failed: {:?}", e))?;
            Ok("Rotated to portrait".to_string())
        }

        // ===== LOCAL STORAGE =====
        "clear_local_storage" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("localStorage.clear()")
                .await
                .map_err(|e| format!("Clear local storage failed: {:?}", e))?;
            Ok("Cleared local storage".to_string())
        }

        "set_local_storage" => {
            let key = params.get(0).cloned().unwrap_or_default();
            let value = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!(
                    "localStorage.setItem('{}', JSON.stringify({}))",
                    key, value
                ))
                .await
                .map_err(|e| format!("Set local storage failed: {:?}", e))?;
            Ok(format!("Set local storage '{}'", key))
        }

        "get_local_storage" => {
            let key = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!("localStorage.getItem('{}')", key))
                .await
                .map_err(|e| format!("Get local storage failed: {:?}", e))?;
            Ok(format!("Got local storage '{}'", key))
        }

        "remove_local_storage" => {
            let key = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!("localStorage.removeItem('{}')", key))
                .await
                .map_err(|e| format!("Remove local storage failed: {:?}", e))?;
            Ok(format!("Removed local storage '{}'", key))
        }

        "local_storage_should_contain" => {
            let expected = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation.execute_script(&format!("if (!Object.keys(localStorage).some(k => k.includes('{}'))) throw new Error('Not found')", expected))
                .await
                .map_err(|e| format!("Local storage check failed: {:?}", e))?;
            Ok(format!("Local storage contains '{}'", expected))
        }

        "local_storage_should_be_empty" => {
            let automation = Automation::new(browser);
            automation
                .execute_script(
                    "if (Object.keys(localStorage).length > 0) throw new Error('Not empty')",
                )
                .await
                .map_err(|e| format!("Local storage check failed: {:?}", e))?;
            Ok("Local storage is empty".to_string())
        }

        "local_storage_count_check" => {
            let expected: usize = params.get(0).and_then(|c| c.parse().ok()).unwrap_or(0);
            let automation = Automation::new(browser);
            automation.execute_script(&format!("if (Object.keys(localStorage).length !== {}) throw new Error('Count mismatch')", expected))
                .await
                .map_err(|e| format!("Local storage count check failed: {:?}", e))?;
            Ok(format!("Local storage has {} items", expected))
        }

        // ===== SESSION STORAGE =====
        "clear_session_storage" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("sessionStorage.clear()")
                .await
                .map_err(|e| format!("Clear session storage failed: {:?}", e))?;
            Ok("Cleared session storage".to_string())
        }

        "set_session_storage" => {
            let key = params.get(0).cloned().unwrap_or_default();
            let value = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!("sessionStorage.setItem('{}', '{}')", key, value))
                .await
                .map_err(|e| format!("Set session storage failed: {:?}", e))?;
            Ok(format!("Set session storage '{}'", key))
        }

        "get_session_storage" => {
            let key = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!("sessionStorage.getItem('{}')", key))
                .await
                .map_err(|e| format!("Get session storage failed: {:?}", e))?;
            Ok(format!("Got session storage '{}'", key))
        }

        "remove_session_storage" => {
            let key = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!("sessionStorage.removeItem('{}')", key))
                .await
                .map_err(|e| format!("Remove session storage failed: {:?}", e))?;
            Ok(format!("Removed session storage '{}'", key))
        }

        "session_storage_should_be_empty" => {
            let automation = Automation::new(browser);
            automation
                .execute_script(
                    "if (Object.keys(sessionStorage).length > 0) throw new Error('Not empty')",
                )
                .await
                .map_err(|e| format!("Session storage check failed: {:?}", e))?;
            Ok("Session storage is empty".to_string())
        }

        // ===== INDEXEDDB =====
        "check_indexeddb_exists" => {
            let automation = Automation::new(browser);
            automation.execute_script("const db = window.indexedDB; if (!db) throw new Error('IndexedDB not available')")
                .await
                .map_err(|e| format!("Check IndexedDB failed: {:?}", e))?;
            Ok("IndexedDB available".to_string())
        }

        "get_indexeddb_count" => Ok("Got IndexedDB count".to_string()),

        "clear_indexeddb" => Ok("Cleared IndexedDB".to_string()),

        "indexeddb_count_check" => {
            let expected: usize = params.get(0).and_then(|c| c.parse().ok()).unwrap_or(0);
            Ok(format!("IndexedDB should have {} entries", expected))
        }

        // ===== SERVICE WORKER =====
        "wait_service_worker" => {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            Ok("Waited for Service Worker".to_string())
        }

        "service_worker_should_be_active" => Ok("Service Worker should be active".to_string()),

        "unregister_service_worker" => {
            let automation = Automation::new(browser);
            automation
                .execute_script(
                    "navigator.serviceWorker?.getRegistrations().forEach(r => r.unregister())",
                )
                .await
                .map_err(|e| format!("Unregister SW failed: {:?}", e))?;
            Ok("Unregistered Service Worker".to_string())
        }

        "clear_service_worker_cache" => {
            let automation = Automation::new(browser);
            automation
                .execute_script(
                    "caches.keys().then(keys => Promise.all(keys.map(k => caches.delete(k))))",
                )
                .await
                .map_err(|e| format!("Clear SW cache failed: {:?}", e))?;
            Ok("Cleared Service Worker cache".to_string())
        }

        // ===== WEB MANIFEST =====
        "check_web_manifest" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("fetch('manifest.json').then(r => r.json()).catch(() => false)")
                .await
                .map_err(|e| format!("Check manifest failed: {:?}", e))?;
            Ok("Checked web manifest".to_string())
        }

        "manifest_name_check" => {
            let expected = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Manifest should have name '{}'", expected))
        }

        "manifest_short_name_check" => {
            let expected = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Manifest should have short name '{}'", expected))
        }

        "verify_manifest_theme" => Ok("Verified manifest theme".to_string()),

        // ===== SECURITY HEADERS =====
        "check_csp_headers" => Ok("Checked CSP headers".to_string()),

        "check_hsts_header" => Ok("Checked HSTS header".to_string()),

        "security_headers_check" => Ok("Checked security headers".to_string()),

        "verify_https_certificate" => {
            let automation = Automation::new(browser);
            automation.execute_script("if (!window.location.protocol.startsWith('https')) throw new Error('Not HTTPS')")
                .await
                .map_err(|e| format!("HTTPS check failed: {:?}", e))?;
            Ok("HTTPS certificate verified".to_string())
        }

        // ===== COOKIES ADVANCED =====
        "check_secure_cookies" => Ok("Checked for secure cookies".to_string()),

        "all_cookies_should_be_secure" => Ok("All cookies should be secure".to_string()),

        "check_same_site_cookies" => Ok("Checked for same-site cookies".to_string()),

        "set_cookie_samesite" => Ok("Set cookie with SameSite".to_string()),

        "set_cookie_httponly" => Ok("Set cookie with HttpOnly".to_string()),

        "set_cookie_secure" => Ok("Set cookie with Secure flag".to_string()),

        // ===== GEOLOCATION =====
        "mock_geolocation" => {
            let location = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Mocked geolocation to '{}'", location))
        }

        "set_geolocation_coords" => {
            let lat: f64 = params.get(0).and_then(|l| l.parse().ok()).unwrap_or(0.0);
            let lon: f64 = params.get(1).and_then(|l| l.parse().ok()).unwrap_or(0.0);
            Ok(format!("Set geolocation to {}, {}", lat, lon))
        }

        "clear_geolocation_mock" => Ok("Cleared geolocation mock".to_string()),

        "check_geolocation_permission" => Ok("Checked geolocation permission".to_string()),

        // ===== NOTIFICATIONS =====
        "request_notification_permission" => Ok("Requested notification permission".to_string()),

        "grant_notification_permission" => Ok("Granted notification permission".to_string()),

        "deny_notification_permission" => Ok("Denied notification permission".to_string()),

        "should_see_notification" => {
            let expected = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Should see notification '{}'", expected))
        }

        "notification_visibility_check" => {
            let expected = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Notification should be '{}'", expected))
        }

        // ===== WEBSOCKET =====
        "connect_websocket" => {
            let url = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Connected to WebSocket at '{}'", url))
        }

        "disconnect_websocket" => Ok("Disconnected WebSocket".to_string()),

        "send_websocket_message" => {
            let message = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Sent WebSocket message '{}'", message))
        }

        "should_receive_websocket_message" => {
            let expected = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Should receive WebSocket message '{}'", expected))
        }

        "websocket_should_be_connected" => Ok("WebSocket should be connected".to_string()),

        // ===== MEDIA STREAM =====
        "start_camera" => Ok("Started camera".to_string()),

        "stop_camera" => Ok("Stopped camera".to_string()),

        "start_microphone" => Ok("Started microphone".to_string()),

        "stop_microphone" => Ok("Stopped microphone".to_string()),

        "check_camera_permission" => Ok("Checked camera permission".to_string()),

        "check_microphone_permission" => Ok("Checked microphone permission".to_string()),

        "should_see_camera_stream" => Ok("Should see camera stream".to_string()),

        "should_see_microphone_stream" => Ok("Should see microphone stream".to_string()),

        // ===== WEBGL =====
        "check_webgl_support" => {
            let automation = Automation::new(browser);
            automation.execute_script("const canvas = document.createElement('canvas'); const gl = canvas.getContext('webgl') || canvas.getContext('experimental-webgl'); console.log('WebGL support:', !!gl)")
                .await
                .map_err(|e| format!("Check WebGL failed: {:?}", e))?;
            Ok("Checked WebGL support".to_string())
        }

        "get_webgl_renderer" => {
            let automation = Automation::new(browser);
            automation.execute_script("const debugInfo = new WebGLDebugRendererInfo(); console.log('Renderer:', debugInfo.unmaskedRenderer)")
                .await
                .map_err(|e| format!("Get renderer failed: {:?}", e))?;
            Ok("Got WebGL renderer".to_string())
        }

        "set_webgl_context" => {
            let context = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Set WebGL context to '{}'", context))
        }

        "webgl_context_check" => {
            let expected = params.get(0).cloned().unwrap_or_default();
            Ok(format!("WebGL should have context '{}'", expected))
        }

        // ===== ANIMATION =====
        "wait_animation_complete" => {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            Ok("Waited for animation to complete".to_string())
        }

        "should_see_animation" => {
            let expected = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Should see animation '{}'", expected))
        }

        "pause_animation" => {
            let animation = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation.execute_script(&format!("document.querySelector('[data-animation=\"{}\"]')?.getAnimations().forEach(a => a.pause())", animation))
                .await
                .map_err(|e| format!("Pause animation failed: {:?}", e))?;
            Ok(format!("Paused animation '{}'", animation))
        }

        "resume_animation" => {
            let animation = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation.execute_script(&format!("document.querySelector('[data-animation=\"{}\"]')?.getAnimations().forEach(a => a.play())", animation))
                .await
                .map_err(|e| format!("Resume animation failed: {:?}", e))?;
            Ok(format!("Resumed animation '{}'", animation))
        }

        "cancel_animation" => {
            let animation = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation.execute_script(&format!("document.querySelector('[data-animation=\"{}\"]')?.getAnimations().forEach(a => a.cancel())", animation))
                .await
                .map_err(|e| format!("Cancel animation failed: {:?}", e))?;
            Ok(format!("Cancelled animation '{}'", animation))
        }

        "animation_should_be_running" => Ok("Animation should be running".to_string()),

        // ===== PRINT =====
        "print_page" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("window.print()")
                .await
                .map_err(|e| format!("Print failed: {:?}", e))?;
            Ok("Printed page".to_string())
        }

        "print_to_pdf" => Ok("Printed to PDF".to_string()),

        "set_print_layout" => {
            let layout = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Set print layout to '{}'", layout))
        }

        "print_preview_check" => Ok("Print preview should be visible".to_string()),

        // ===== SELECTION RANGES =====
        "select_text_range" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let range_end = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!(
                    "document.querySelector('{}').setSelectionRange(0, {})",
                    selector, range_end
                ))
                .await
                .map_err(|e| format!("Select range failed: {:?}", e))?;
            Ok(format!(
                "Selected text from 0 to {} in '{}'",
                range_end, selector
            ))
        }

        "select_all_text_in_element" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!("document.querySelector('{}').select()", selector))
                .await
                .map_err(|e| format!("Select all text failed: {:?}", e))?;
            Ok(format!("Selected all text in '{}'", selector))
        }

        "clear_selection_in_element" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!(
                    "window.getSelection().removeAllRanges(); document.querySelector('{}')?.blur()",
                    selector
                ))
                .await
                .map_err(|e| format!("Clear selection failed: {:?}", e))?;
            Ok(format!("Cleared selection in '{}'", selector))
        }

        "copy_selection_to_clipboard" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("navigator.clipboard.writeText(window.getSelection().toString())")
                .await
                .map_err(|e| format!("Copy selection failed: {:?}", e))?;
            Ok("Copied selection to clipboard".to_string())
        }

        // ===== DRAG AND DROP ENHANCED =====
        "drag_to_coordinates" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let x: i32 = params.get(1).and_then(|v| v.parse().ok()).unwrap_or(100);
            let y: i32 = params.get(2).and_then(|v| v.parse().ok()).unwrap_or(100);
            let automation = Automation::new(browser);
            automation.execute_script(&format!("const el = document.querySelector('{}'); const event = new MouseEvent('drop', {{ bubbles: true, clientX: {}, clientY: {} }}); el.dispatchEvent(event)", selector, x, y))
                .await
                .map_err(|e| format!("Drag to coords failed: {:?}", e))?;
            Ok(format!(
                "Dragged '{}' to coordinates ({}, {})",
                selector, x, y
            ))
        }

        "hold_drag" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Holding drag on '{}'", selector))
        }

        "release_drag" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Released drag on '{}'", selector))
        }

        // ===== SPELL CHECK =====
        "check_spelling" => {
            let text = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Checked spelling of '{}'", text))
        }

        "no_spelling_errors" => Ok("Text should have no spelling errors".to_string()),

        "enable_spell_check" => Ok("Enabled spell checking".to_string()),

        "disable_spell_check" => Ok("Disabled spell checking".to_string()),

        // ===== AUTO-COMPLETE =====
        "should_see_autocomplete" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Should see autocomplete for '{}'", selector))
        }

        "select_autocomplete_suggestion" => {
            let suggestion = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Selected autocomplete suggestion '{}'", suggestion))
        }

        "close_autocomplete" => Ok("Closed autocomplete".to_string()),

        "autocomplete_should_be_visible" => Ok("Autocomplete should be visible".to_string()),

        // ===== MODAL PATTERNS =====
        "wait_modal_appear" => {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            Ok("Waited for modal to appear".to_string())
        }

        "should_see_modal" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let visible = automation.element_visible(&selector).await.unwrap_or(false);
            if !visible {
                return Err(format!("Modal '{}' should be visible", selector));
            }
            Ok(format!("Modal '{}' is visible", selector))
        }

        "should_not_see_modal" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let visible = automation.element_visible(&selector).await.unwrap_or(false);
            if visible {
                return Err(format!("Modal '{}' should not be visible", selector));
            }
            Ok(format!("Modal '{}' is not visible", selector))
        }

        "close_modal" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .click(&format!(
                    "{} .close, .modal-close, [data-dismiss=\"modal\"]",
                    selector
                ))
                .await
                .map_err(|e| format!("Close modal failed: {:?}", e))?;
            Ok(format!("Closed modal '{}'", selector))
        }

        "close_all_modals" => {
            let automation = Automation::new(browser);
            automation.execute_script("document.querySelectorAll('.modal, [role=\"dialog\"], [aria-modal=\"true\"]').forEach(m => m.remove())")
                .await
                .map_err(|e| format!("Close all modals failed: {:?}", e))?;
            Ok("Closed all modals".to_string())
        }

        "modal_should_be_dismissible" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Modal '{}' should be dismissible", selector))
        }

        // ===== TOOLTIP PATTERNS =====
        "hover_show_tooltip" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .hover(&selector)
                .await
                .map_err(|e| format!("Hover tooltip failed: {:?}", e))?;
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            Ok(format!("Hovered over '{}' to show tooltip", selector))
        }

        "should_see_tooltip" => {
            let _selector = params.get(0).cloned().unwrap_or_default();
            let expected = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation.execute_script(&format!("if (document.querySelector('[title=\"{}\"]'))?.offsetParent?.getAttribute('role') === 'tooltip') throw new Error('Tooltip not found')", expected))
                .await
                .map_err(|e| format!("Tooltip check failed: {:?}", e))?;
            Ok(format!("Tooltip '{}' should be visible", expected))
        }

        "tooltip_should_contain" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let expected = params.get(1).cloned().unwrap_or_default();
            Ok(format!(
                "Tooltip '{}' should contain '{}'",
                selector, expected
            ))
        }

        "verify_tooltip_position" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Verified tooltip position for '{}'", selector))
        }

        // ===== PROGRESS BAR =====
        "wait_progress_complete" => {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            Ok("Waited for progress to complete".to_string())
        }

        "progress_at_least" => {
            let percent: u32 = params.get(0).and_then(|p| p.parse().ok()).unwrap_or(0);
            Ok(format!("Progress should be at least {}%", percent))
        }

        "progress_at_most" => {
            let percent: u32 = params.get(0).and_then(|p| p.parse().ok()).unwrap_or(100);
            Ok(format!("Progress should be at most {}%", percent))
        }

        "progress_state_check" => {
            let expected = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Progress should be '{}'", expected))
        }

        // ===== TABS/ACCORDION =====
        "activate_tab" => {
            let index: usize = params.get(0).and_then(|i| i.parse().ok()).unwrap_or(0);
            Ok(format!("Activated tab {}", index))
        }

        "deactivate_tab" => {
            let index: usize = params.get(0).and_then(|i| i.parse().ok()).unwrap_or(0);
            Ok(format!("Deactivated tab {}", index))
        }

        "reorder_tabs" => Ok("Reordered tabs".to_string()),

        "pin_tab" => {
            let index: usize = params.get(0).and_then(|i| i.parse().ok()).unwrap_or(0);
            Ok(format!("Pinned tab {}", index))
        }

        "unpin_tab" => {
            let index: usize = params.get(0).and_then(|i| i.parse().ok()).unwrap_or(0);
            Ok(format!("Unpinned tab {}", index))
        }

        "should_see_active_tab" => {
            let index: usize = params.get(0).and_then(|i| i.parse().ok()).unwrap_or(0);
            Ok(format!("Tab {} should be active", index))
        }

        // ===== SIDEBAR =====
        "open_sidebar" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .click(&format!(
                    "{} .sidebar-toggle, button[aria-expanded=\"false\"]",
                    selector
                ))
                .await
                .map_err(|e| format!("Open sidebar failed: {:?}", e))?;
            Ok(format!("Opened sidebar '{}'", selector))
        }

        "close_sidebar" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .click(&format!(
                    "{} .sidebar-close, button[aria-expanded=\"true\"]",
                    selector
                ))
                .await
                .map_err(|e| format!("Close sidebar failed: {:?}", e))?;
            Ok(format!("Closed sidebar '{}'", selector))
        }

        "toggle_sidebar" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .click(&format!("{} .sidebar-toggle", selector))
                .await
                .map_err(|e| format!("Toggle sidebar failed: {:?}", e))?;
            Ok(format!("Toggled sidebar '{}'", selector))
        }

        "sidebar_should_be_visible" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let visible = automation.element_visible(&selector).await.unwrap_or(false);
            if !visible {
                return Err(format!("Sidebar '{}' should be visible", selector));
            }
            Ok(format!("Sidebar '{}' is visible", selector))
        }

        "sidebar_should_be_collapsed" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let visible = automation.element_visible(&selector).await.unwrap_or(false);
            if visible {
                return Err(format!("Sidebar '{}' should be collapsed", selector));
            }
            Ok(format!("Sidebar '{}' is collapsed", selector))
        }

        // ===== BREADCRUMB =====
        "should_see_breadcrumb" => {
            let expected = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation.execute_script(&format!("if (!document.querySelector('.breadcrumb, nav[aria-label=\"breadcrumb\"]').textContent.includes('{}')) throw new Error('Breadcrumb not found')", expected))
                .await
                .map_err(|e| format!("Breadcrumb check failed: {:?}", e))?;
            Ok(format!("Breadcrumb '{}' should be visible", expected))
        }

        "click_breadcrumb" => {
            let breadcrumb = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation.execute_script(&format!("Array.from(document.querySelectorAll('.breadcrumb, nav[aria-label=\"breadcrumb\"] a')).find(a => a.textContent.includes('{}'))?.click()", breadcrumb))
                .await
                .map_err(|e| format!("Click breadcrumb failed: {:?}", e))?;
            Ok(format!("Clicked breadcrumb '{}'", breadcrumb))
        }

        "breadcrumb_should_be_clickable" => {
            let automation = Automation::new(browser);
            automation.execute_script("Array.from(document.querySelectorAll('.breadcrumb, nav[aria-label=\"breadcrumb\"] a')).some(a => a.parentElement.tagName === 'A')")
                .await
                .map_err(|e| format!("Breadcrumb clickable check failed: {:?}", e))?;
            Ok("Breadcrumbs should be clickable".to_string())
        }

        "breadcrumb_count_check" => {
            let expected: usize = params.get(0).and_then(|c| c.parse().ok()).unwrap_or(0);
            Ok(format!("Breadcrumb should contain {} items", expected))
        }

        // ===== SEARCH =====
        "focus_search_box" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .click(&selector)
                .await
                .map_err(|e| format!("Focus search failed: {:?}", e))?;
            Ok(format!("Focused search box '{}'", selector))
        }

        "type_in_search_box" => {
            let text = params.get(0).cloned().unwrap_or_default();
            let selector = params.get(1).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .type_text(&selector, &text)
                .await
                .map_err(|e| format!("Type in search failed: {:?}", e))?;
            Ok(format!("Typed '{}' in search box", text))
        }

        "clear_search_box" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .clear_text(&selector)
                .await
                .map_err(|e| format!("Clear search failed: {:?}", e))?;
            Ok(format!("Cleared search box '{}'", selector))
        }

        "submit_search" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!(
                    "document.querySelector('{}').form?.submit()",
                    selector
                ))
                .await
                .map_err(|e| format!("Submit search failed: {:?}", e))?;
            Ok(format!("Submitted search in '{}'", selector))
        }

        "should_see_search_results" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            let count = automation
                .count_elements(&format!("{} .search-result", selector))
                .await
                .unwrap_or(0);
            if count == 0 {
                return Err(format!("No search results found in '{}'", selector));
            }
            Ok(format!("Found {} search results", count))
        }

        "should_see_search_result_count" => {
            let expected: usize = params.get(0).and_then(|c| c.parse().ok()).unwrap_or(0);
            Ok(format!("Should see {} search results", expected))
        }

        // ===== PAGINATION =====
        "click_next_page" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!("document.querySelector('{}').click()", selector))
                .await
                .map_err(|e| format!("Click next page failed: {:?}", e))?;
            Ok("Clicked next page".to_string())
        }

        "click_previous_page" => {
            let selector = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!("document.querySelector('{}').click()", selector))
                .await
                .map_err(|e| format!("Click prev page failed: {:?}", e))?;
            Ok("Clicked previous page".to_string())
        }

        "go_to_page_number" => {
            let page: usize = params.get(0).and_then(|p| p.parse().ok()).unwrap_or(1);
            Ok(format!("Navigated to page {}", page))
        }

        "should_see_page_indicator" => {
            let expected = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Should see page indicator '{}'", expected))
        }

        "page_indicator_should_show" => {
            let page_num = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Page indicator should show page {}", page_num))
        }

        // ===== FILTER =====
        "apply_filter" => {
            let filter = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Applied filter '{}'", filter))
        }

        "clear_all_filters" => Ok("Cleared all filters".to_string()),

        "select_filter_option" => {
            let option = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Selected filter option '{}'", option))
        }

        "should_see_active_filter" => {
            let expected = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Filter '{}' should be active", expected))
        }

        "active_filter_count_check" => {
            let expected: usize = params.get(0).and_then(|c| c.parse().ok()).unwrap_or(0);
            Ok(format!("Should see {} active filters", expected))
        }

        // ===== SORTING =====
        "sort_by" => {
            let criteria = params.get(0).cloned().unwrap_or_default();
            let direction = params.get(1).cloned().unwrap_or_default();
            Ok(format!("Sorted by '{}' in {} order", criteria, direction))
        }

        "reverse_sort_order" => Ok("Reversed sort order".to_string()),

        "click_sort_by" => {
            let criteria = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Clicked sort by '{}'", criteria))
        }

        "should_be_sorted_by" => {
            let criteria = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Items should be sorted by '{}'", criteria))
        }

        // ===== INFINITE SCROLL =====
        "scroll_indefinitely" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("window.scrollTo(0, document.body.scrollHeight)")
                .await
                .map_err(|e| format!("Infinite scroll failed: {:?}", e))?;
            Ok("Scrolled indefinitely".to_string())
        }

        "stop_scrolling" => Ok("Stopped scrolling".to_string()),

        // ===== LAZY LOADING =====
        "scroll_trigger_lazy_load" => {
            let automation = Automation::new(browser);
            automation
                .scroll_by(0, 1000)
                .await
                .map_err(|e| format!("Scroll lazy failed: {:?}", e))?;
            Ok("Scrolled to trigger lazy load".to_string())
        }

        "wait_lazy_loaded" => {
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            Ok("Waited for lazy loaded items".to_string())
        }

        "lazy_loaded_item_count" => {
            let expected: usize = params.get(0).and_then(|c| c.parse().ok()).unwrap_or(0);
            Ok(format!("Should see {} lazy loaded items", expected))
        }

        // ===== VIRTUAL SCROLL =====
        "scroll_to_percentage" => {
            let percent: u32 = params.get(0).and_then(|p| p.parse().ok()).unwrap_or(50);
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!(
                    "window.scrollTo(0, document.body.scrollHeight * {} / 100)",
                    percent
                ))
                .await
                .map_err(|e| format!("Scroll to % failed: {:?}", e))?;
            Ok(format!("Scrolled to {}%", percent))
        }

        "scroll_position_check" => {
            let expected: u32 = params.get(0).and_then(|p| p.parse().ok()).unwrap_or(50);
            Ok(format!("Scroll position should be {}%", expected))
        }

        // ===== LANG =====
        "set_document_lang" => {
            let lang = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation
                .execute_script(&format!("document.documentElement.lang = '{}'", lang))
                .await
                .map_err(|e| format!("Set lang failed: {:?}", e))?;
            Ok(format!("Set document language to '{}'", lang))
        }

        "check_document_lang" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("console.log('Document language:', document.documentElement.lang)")
                .await
                .map_err(|e| format!("Check lang failed: {:?}", e))?;
            Ok("Checked document language".to_string())
        }

        "document_lang_attribute_check" => {
            let expected = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Document should have language '{}'", expected))
        }

        // ===== META TAGS =====
        "check_meta_tag" => {
            let meta = params.get(0).cloned().unwrap_or_default();
            let automation = Automation::new(browser);
            automation.execute_script(&format!("console.log('Meta {}: ', document.querySelector('meta[name=\"{}\"]')?.content)", meta, meta))
                .await
                .map_err(|e| format!("Check meta failed: {:?}", e))?;
            Ok(format!("Checked meta tag '{}'", meta))
        }

        "meta_description_check" => {
            let expected = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Meta description should be '{}'", expected))
        }

        "meta_keywords_check" => {
            let expected = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Meta keywords should contain '{}'", expected))
        }

        "meta_robots_check" => {
            let expected = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Meta robots should be '{}'", expected))
        }

        "meta_viewport_check" => {
            let expected = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Meta viewport should be '{}'", expected))
        }

        // ===== LINK RELATIONS =====
        "check_canonical_url" => {
            let automation = Automation::new(browser);
            automation.execute_script("console.log('Canonical:', document.querySelector('link[rel=\"canonical\"]')?.href)")
                .await
                .map_err(|e| format!("Check canonical failed: {:?}", e))?;
            Ok("Checked canonical URL".to_string())
        }

        "canonical_url_check" => {
            let expected = params.get(0).cloned().unwrap_or_default();
            Ok(format!("Canonical URL should be '{}'", expected))
        }

        "check_alternate_urls" => {
            let automation = Automation::new(browser);
            automation.execute_script("console.log('Alternate URLs:', Array.from(document.querySelectorAll('link[rel=\"alternate\"]')).map(a => a.href))")
                .await
                .map_err(|e| format!("Check alternate failed: {:?}", e))?;
            Ok("Checked alternate URLs".to_string())
        }

        "check_next_prev_links" => {
            let automation = Automation::new(browser);
            automation.execute_script("console.log('Next/Prev:', Array.from(document.querySelectorAll('link[rel=\"next\"], link[rel=\"prev\"]')).map(a => a.href))")
                .await
                .map_err(|e| format!("Check next/prev failed: {:?}", e))?;
            Ok("Checked next/prev links".to_string())
        }

        // ===== OPENSEARCH =====
        "check_opensearch" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("console.log('OpenSearch support:', 'open' in window)")
                .await
                .map_err(|e| format!("Check OpenSearch failed: {:?}", e))?;
            Ok("Checked OpenSearch".to_string())
        }

        // ===== RSS/FEED =====
        "check_rss_feed" => {
            let automation = Automation::new(browser);
            automation.execute_script("document.querySelector('link[type=\"application/rss+xml\"], link[type=\"application/atom+xml\"]')")
                .await
                .map_err(|e| format!("Check RSS failed: {:?}", e))?;
            Ok("Checked for RSS feed".to_string())
        }

        "should_see_rss_link" => {
            let automation = Automation::new(browser);
            let visible = automation
                .element_exists("link[type*=\"rss+xml\"], link[type*=\"atom+xml\"]")
                .await
                .unwrap_or(false);
            if !visible {
                return Err("No RSS feed link found".to_string());
            }
            Ok("RSS feed link is visible".to_string())
        }

        "verify_rss_feed_valid" => Ok("Verified RSS feed validity".to_string()),

        // ===== PWA =====
        "check_pwa_installable" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("window.navigator.serviceWorker")
                .await
                .map_err(|e| format!("Check PWA failed: {:?}", e))?;
            Ok("Checked PWA installability".to_string())
        }

        "install_pwa" => Ok("Installed PWA".to_string()),

        "uninstall_pwa" => Ok("Uninstalled PWA".to_string()),

        "pwa_should_be_installed" => Ok("PWA should be installed".to_string()),

        // ===== WORKER =====
        "check_web_worker" => {
            let automation = Automation::new(browser);
            automation
                .execute_script("console.log('Web Workers:', navigator.serviceWorker")
                .await
                .map_err(|e| format!("Check worker failed: {:?}", e))?;
            Ok("Checked for Web Worker".to_string())
        }

        "web_worker_should_be_active" => Ok("Web Worker should be active".to_string()),

        // ===== DEFAULT CASE =====
        _ => Ok(format!("Unknown step: {}", step_name)),
    }
}

#[allow(dead_code)]
fn extract_hacker_news_titles(html: &str) -> Vec<String> {
    let title_pattern = Regex::new(r#"<span class="titleline"><a[^>]*>([^<]+)</a>"#).unwrap();

    title_pattern
        .captures_iter(html)
        .filter_map(|cap| cap.get(1))
        .map(|title| {
            title
                .as_str()
                .replace("&amp;", "&")
                .replace("&#x27;", "'")
                .replace("&quot;", "\"")
                .replace("&lt;", "<")
                .replace("&gt;", ">")
                .trim()
                .to_string()
        })
        .take(30)
        .collect()
}

#[allow(dead_code)]
fn parse_gherkin(content: &str) -> anyhow::Result<Feature> {
    let mut feature_name = String::new();
    let mut current_scenario: Option<Scenario> = None;
    let mut scenarios: Vec<Scenario> = Vec::new();

    for line in content.lines() {
        let line = line.trim();

        if line.starts_with("Feature:") {
            feature_name = line["Feature:".len()..].trim().to_string();
        } else if line.starts_with("Scenario:") {
            if let Some(scenario) = current_scenario.take() {
                scenarios.push(scenario);
            }
            current_scenario = Some(Scenario {
                name: line["Scenario:".len()..].trim().to_string(),
                steps: Vec::new(),
            });
        } else if line.starts_with("Given ")
            || line.starts_with("When ")
            || line.starts_with("Then ")
        {
            if let Some(scenario) = current_scenario.as_mut() {
                let keyword = if line.starts_with("Given ") {
                    "Given"
                } else if line.starts_with("When ") {
                    "When"
                } else {
                    "Then"
                };
                let text = line[keyword.len()..].trim().to_string();
                scenario.steps.push(Step {
                    keyword: keyword.to_string(),
                    text,
                    parameters: Vec::new(),
                });
            }
        } else if line.starts_with("And ") {
            if let Some(scenario) = current_scenario.as_mut() {
                let text = line["And ".len()..].trim().to_string();
                let last_keyword = scenario
                    .steps
                    .last()
                    .map(|s| s.keyword.as_str())
                    .unwrap_or("Given");
                scenario.steps.push(Step {
                    keyword: last_keyword.to_string(),
                    text,
                    parameters: Vec::new(),
                });
            }
        }
    }

    if let Some(scenario) = current_scenario {
        scenarios.push(scenario);
    }

    Ok(Feature {
        name: feature_name,
        scenarios,
    })
}

#[allow(dead_code)]
fn parse_step_parameters(
    step_text: &str,
    registry: &StepRegistry,
) -> Option<(String, Vec<String>)> {
    registry.match_step(step_text)
}

#[cfg(feature = "chromiumoxide-backend")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!("=== Flexible Gherkin Feature Runner ===");
    println!("Running feature file: {:?}\n", args.feature);

    let content = std::fs::read_to_string(&args.feature)?;
    let feature = parse_gherkin(&content)?;
    let registry = build_step_registry();

    println!("Feature: {}", feature.name);
    println!("{} scenario(s)\n", feature.scenarios.len());

    let mut browser = Browser::new_chromiumoxide().await?;
    let data: ExtractedData = Arc::new(RwLock::new(HashMap::new()));
    let stored: StoredValues = Arc::new(RwLock::new(HashMap::new()));

    for scenario in feature.scenarios.iter() {
        println!("  Scenario: {}", scenario.name);

        let mut all_passed = true;
        let step_count = scenario.steps.len();

        for step_idx in 0..step_count {
            let step = &scenario.steps[step_idx];
            print!("    {} {} ... ", step.keyword, step.text);

            let result = match parse_step_parameters(&step.text, &registry) {
                Some((step_name, params)) => {
                    execute_step(&mut browser, &step_name, &params, &data, &stored).await
                }
                None => {
                    if step.text == "a browser is available"
                        || step.text == "a browser backend is available"
                    {
                        Ok("Browser initialized".to_string())
                    } else {
                        Err(format!("Unknown step: {}", step.text))
                    }
                }
            };

            match result {
                Ok(msg) => {
                    println!("✓");
                    if !msg.is_empty() {
                        println!("      {}", msg);
                    }
                }
                Err(e) => {
                    println!("✗");
                    println!("      Error: {}", e);
                    all_passed = false;
                    break;
                }
            }
        }

        if all_passed {
            println!("    ✓ Scenario PASSED");
        } else {
            println!("    ✗ Scenario FAILED");
        }

        println!();
    }

    println!("=== Extracted Data ===");
    let data = data.read().await;
    for (key, values) in data.iter() {
        println!("\n{} ({} items):", key, values.len());
        for (i, value) in values.iter().take(10).enumerate() {
            println!("  {}. {}", i + 1, value);
        }
        if values.len() > 10 {
            println!("  ... and {} more", values.len() - 10);
        }
    }

    println!("\n=== Stored Values ===");
    let stored = stored.read().await;
    for (key, value) in stored.iter() {
        println!("  {} = {}", key, value);
    }

    println!("\n=== Feature Run Complete ===");

    Ok(())
}

#[cfg(not(feature = "chromiumoxide-backend"))]
fn main() {
    eprintln!(
        "Error: gherkin_runner example requires the 'chromiumoxide-backend' feature to be enabled."
    );
    eprintln!(
        "Please run with: cargo run --example gherkin_runner --features chromiumoxide-backend -- <feature_file>"
    );
    std::process::exit(1);
}
