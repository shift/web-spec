//! Complete step catalog with all registered patterns

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
    pub required: bool,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepInfo {
    pub id: String,
    pub pattern: String,
    pub aliases: Vec<String>,
    pub category: String,
    pub description: String,
    pub parameters: Vec<ParameterInfo>,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepCatalog {
    pub steps: Vec<StepInfo>,
    pub categories: Vec<String>,
}

impl StepCatalog {
    pub fn new() -> Self {
        StepCatalog {
            steps: Vec::new(),
            categories: Vec::new(),
        }
    }

    pub fn add_step(&mut self, step: StepInfo) {
        if !self.categories.contains(&step.category) {
            self.categories.push(step.category.clone());
        }
        self.categories.sort();
        self.steps.push(step);
    }

    pub fn find_by_id(&self, id: &str) -> Option<&StepInfo> {
        self.steps.iter().find(|s| s.id == id)
    }

    pub fn find_by_category(&self, category: &str) -> Vec<&StepInfo> {
        self.steps
            .iter()
            .filter(|s| s.category == category)
            .collect()
    }

    pub fn all_steps(&self) -> &[StepInfo] {
        &self.steps
    }

    pub fn total_steps(&self) -> usize {
        self.steps.len()
    }
}

impl Default for StepCatalog {
    fn default() -> Self {
        Self::new()
    }
}

pub fn build_step_catalog() -> StepCatalog {
    let mut catalog = StepCatalog::new();

    catalog.add_step(StepInfo {
        id: "activate_tab".to_string(),
        pattern: r#"I activate tab "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: activate_tab".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "alert_text_should_be".to_string(),
        pattern: r#"the alert text should be "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: alert_text_should_be".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "apply_filter".to_string(),
        pattern: r#"I apply filter "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: apply_filter".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "attribute_should_be".to_string(),
        pattern: r#"the "([^"]+)" attribute of "([^"]+)" should be "([^"]+)""#.to_string(),
        aliases: vec![
            r#"the element "([^"]+)" should have "([^"]+)" attribute set to "([^"]+)""#.to_string(),
        ],
        category: "Verification".to_string(),
        description: "Verify element attribute value".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "attribute_should_contain".to_string(),
        pattern: r#"the "([^"]+)" attribute of "([^"]+)" should contain "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Verification".to_string(),
        description: "Verify element attribute contains value".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "attribute_should_exist".to_string(),
        pattern: r#"the "([^"]+)" attribute of "([^"]+)" should exist"#.to_string(),
        aliases: vec![r#"the element "([^"]+)" should have "([^"]+)" attribute"#.to_string()],
        category: "Verification".to_string(),
        description: "Verify element attribute exists".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "background_should_be".to_string(),
        pattern: r#"the element "([^"]+)" should have background "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Verification".to_string(),
        description: "Verify element background".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "cancel_animation".to_string(),
        pattern: r#"I cancel animation "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: cancel_animation".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "canonical_url_check".to_string(),
        pattern: r#"the canonical URL should be "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: canonical_url_check".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "check".to_string(),
        pattern: r#"I check "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Input".to_string(),
        description: "Check a checkbox".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "check_meta_tag".to_string(),
        pattern: r#"I check for meta "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: check_meta_tag".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "check_spelling".to_string(),
        pattern: r#"I check spelling of "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: check_spelling".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "clear_canvas".to_string(),
        pattern: r#"I clear canvas "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: clear_canvas".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "clear_selection_in_element".to_string(),
        pattern: r#"I clear selection in "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: clear_selection_in_element".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "clear_text".to_string(),
        pattern: r#"I clear "([^"]+)""#.to_string(),
        aliases: vec![
            r#"I clear the input "([^"]+)""#.to_string(),
            r#"I clear the field "([^"]+)""#.to_string(),
        ],
        category: "Input".to_string(),
        description: "Clear the contents of an input field".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "click".to_string(),
        pattern: r#"I click on "([^"]+)""#.to_string(),
        aliases: vec![
            r#"I click "([^"]+)""#.to_string(),
            r#"I press "([^"]+)""#.to_string(),
            r#"I tap "([^"]+)""#.to_string(),
        ],
        category: "Interaction".to_string(),
        description: "Click on an element".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "click_all".to_string(),
        pattern: r#"I click all "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: click_all".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "click_breadcrumb".to_string(),
        pattern: r#"I click breadcrumb "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: click_breadcrumb".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "click_button".to_string(),
        pattern: r#"I click the "([^"]+)" button"#.to_string(),
        aliases: vec![],
        category: "Interaction".to_string(),
        description: "Click a button element".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "click_button_or_link".to_string(),
        pattern: r#"I click the "([^"]+)" (button|link)"#.to_string(),
        aliases: vec![],
        category: "Interaction".to_string(),
        description: "Click a button or link".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "click_link".to_string(),
        pattern: r#"I click the "([^"]+)" link"#.to_string(),
        aliases: vec![],
        category: "Interaction".to_string(),
        description: "Click a link element".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "click_sort_by".to_string(),
        pattern: r#"I click sort by "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: click_sort_by".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "clipboard_should_contain".to_string(),
        pattern: r#"the clipboard should contain "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: clipboard_should_contain".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "color_should_be".to_string(),
        pattern: r#"the element "([^"]+)" should have color "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Verification".to_string(),
        description: "Verify element color".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "conditional_click_if_visible".to_string(),
        pattern: r#"if "([^"]+)" is visible, I click it"#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: conditional_click_if_visible".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "conditional_navigate".to_string(),
        pattern: r#"if the page contains "([^"]+)", I navigate to "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: conditional_navigate".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "conditional_type_if_exists".to_string(),
        pattern: r#"if "([^"]+)" exists, I type "([^"]+)" into it"#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: conditional_type_if_exists".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "connect_websocket".to_string(),
        pattern: r#"I connect to WebSocket at "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: connect_websocket".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "console_should_contain".to_string(),
        pattern: r#"I should see console message "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: console_should_contain".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "console_should_not_contain".to_string(),
        pattern: r#"I should not see console message "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: console_should_not_contain".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "continue_if_visible".to_string(),
        pattern: r#"continue only if "([^"]+)" is (visible|present)"#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: continue_if_visible".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "copy_element_text".to_string(),
        pattern: r#"I copy the text of "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: copy_element_text".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "copy_to_clipboard".to_string(),
        pattern: r#"I copy "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: copy_to_clipboard".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "css_should_be".to_string(),
        pattern: r#"the "([^"]+)" CSS property of "([^"]+)" should be "([^"]+)""#.to_string(),
        aliases: vec![
            r#"the element "([^"]+)" should have "([^"]+)" CSS value of "([^"]+)""#.to_string(),
        ],
        category: "Verification".to_string(),
        description: "Verify CSS property value".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "deactivate_tab".to_string(),
        pattern: r#"I deactivate tab "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: deactivate_tab".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "deselect_all".to_string(),
        pattern: r#"I deselect all from "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Input".to_string(),
        description: "Deselect all options in a multi-select".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "double_click".to_string(),
        pattern: r#"I double click on "([^"]+)""#.to_string(),
        aliases: vec![r#"I double-click "([^"]+)""#.to_string()],
        category: "Interaction".to_string(),
        description: "Double-click an element".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "download_file".to_string(),
        pattern: r#"I download file from "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: download_file".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "download_filename_should_be".to_string(),
        pattern: r#"the downloaded file should be named "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: download_filename_should_be".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "drag_and_drop".to_string(),
        pattern: r#"I drag "([^"]+)" to "([^"]+)""#.to_string(),
        aliases: vec![r#"I drag element "([^"]+)" and drop it on "([^"]+)""#.to_string()],
        category: "Interaction".to_string(),
        description: "Drag an element and drop it on another".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "drag_by_offset".to_string(),
        pattern: r#"I drag "([^"]+)" by offset (-?\d+),(-?\d+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: drag_by_offset".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "drag_to_coordinates".to_string(),
        pattern: r#"I drag "([^"]+)" to coordinates (\d+),(\d+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: drag_to_coordinates".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "draw_on_canvas".to_string(),
        pattern: r#"I draw on canvas "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: draw_on_canvas".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "drop_at".to_string(),
        pattern: r#"I drop "([^"]+)" at "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: drop_at".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "execute_script".to_string(),
        pattern: r#"I execute JavaScript "([^"]+)""#.to_string(),
        aliases: vec![
            r#"I execute script "([^"]+)""#.to_string(),
            r#"I run JavaScript "([^"]+)""#.to_string(),
            r#"I run script "([^"]+)""#.to_string(),
            r#"I evaluate "([^"]+)""#.to_string(),
        ],
        category: "Other".to_string(),
        description: "Step: execute_script".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "extract_all_by_selector".to_string(),
        pattern: r#"I extract all "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: extract_all_by_selector".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "extract_all_elements".to_string(),
        pattern: r#"I extract all "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: extract_all_elements".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "extract_and_store_all".to_string(),
        pattern: r#"I extract all "([^"]+)" and store them as "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: extract_and_store_all".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "extract_attribute_from_element".to_string(),
        pattern: r#"I extract "([^"]+)" from "([^"]+)""#.to_string(),
        aliases: vec![r#"I extract the "([^"]+)" attribute from "([^"]+)""#.to_string()],
        category: "Other".to_string(),
        description: "Step: extract_attribute_from_element".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "extract_headings_level".to_string(),
        pattern: r#"I extract all h(\d) headings"#.to_string(),
        aliases: vec![],
        category: "Extraction".to_string(),
        description: "Extract headings by level".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "extract_table".to_string(),
        pattern: r#"I extract table data from "([^"]+)""#.to_string(),
        aliases: vec![r#"I extract the table "([^"]+)""#.to_string()],
        category: "Other".to_string(),
        description: "Step: extract_table".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "extract_text_from_element".to_string(),
        pattern: r#"I extract text from "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: extract_text_from_element".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "get_canvas_data".to_string(),
        pattern: r#"I get canvas data from "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: get_canvas_data".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "get_local_storage".to_string(),
        pattern: r#"I get local storage item "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "State".to_string(),
        description: "Get local storage value".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "get_session_storage".to_string(),
        pattern: r#"I get session storage item "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "State".to_string(),
        description: "Get session storage value".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "hold_drag".to_string(),
        pattern: r#"I hold drag on "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: hold_drag".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "hover".to_string(),
        pattern: r#"I hover over "([^"]+)""#.to_string(),
        aliases: vec![
            r#"I hover "([^"]+)""#.to_string(),
            r#"I move mouse to "([^"]+)""#.to_string(),
        ],
        category: "Interaction".to_string(),
        description: "Hover over an element".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "hover_show_tooltip".to_string(),
        pattern: r#"I hover over "([^"]+)" to show tooltip"#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: hover_show_tooltip".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "local_storage_should_contain".to_string(),
        pattern: r#"the local storage should contain "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: local_storage_should_contain".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "loop_click_each".to_string(),
        pattern: r#"for each "([^"]+)", I click it"#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: loop_click_each".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "manifest_name_check".to_string(),
        pattern: r#"the manifest should have name "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: manifest_name_check".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "manifest_short_name_check".to_string(),
        pattern: r#"the manifest should have short name "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: manifest_short_name_check".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "meta_description_check".to_string(),
        pattern: r#"the meta description should be "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: meta_description_check".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "meta_keywords_check".to_string(),
        pattern: r#"the meta keywords should contain "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: meta_keywords_check".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "meta_robots_check".to_string(),
        pattern: r#"the meta robots should be "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: meta_robots_check".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "meta_viewport_check".to_string(),
        pattern: r#"the meta viewport should be "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: meta_viewport_check".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "mock_geolocation".to_string(),
        pattern: r#"I mock geolocation to "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: mock_geolocation".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "mouse_down".to_string(),
        pattern: r#"I mouse down on "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: mouse_down".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "mouse_move_to".to_string(),
        pattern: r#"I mouse move to "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: mouse_move_to".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "mouse_out".to_string(),
        pattern: r#"I mouse out of "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: mouse_out".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "mouse_over".to_string(),
        pattern: r#"I mouse over "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: mouse_over".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "mouse_up".to_string(),
        pattern: r#"I mouse up on "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: mouse_up".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "multi_touch".to_string(),
        pattern: r#"I perform multi-touch gesture on "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: multi_touch".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "navigate_to".to_string(),
        pattern: r#"I navigate to "([^"]+)""#.to_string(),
        aliases: vec![
            r#"I go to "([^"]+)""#.to_string(),
            r#"I open "([^"]+)""#.to_string(),
            r#"I visit "([^"]+)""#.to_string(),
        ],
        category: "Navigation".to_string(),
        description: "Navigate to a specified URL".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "paste_into".to_string(),
        pattern: r#"I paste "([^"]+)" into "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: paste_into".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "path_should_be".to_string(),
        pattern: r#"the path should be "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Verification".to_string(),
        description: "Verify current path".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "path_should_contain".to_string(),
        pattern: r#"the path should contain "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Verification".to_string(),
        description: "Verify path contains value".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "pause_animation".to_string(),
        pattern: r#"I pause animation "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: pause_animation".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "pinch_zoom".to_string(),
        pattern: r#"I pinch to zoom in on "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: pinch_zoom".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "press_enter".to_string(),
        pattern: r#"I press the Enter key"#.to_string(),
        aliases: vec![r#"I press Enter"#.to_string()],
        category: "Input".to_string(),
        description: "Press the Enter key".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "press_escape".to_string(),
        pattern: r#"I press Escape key"#.to_string(),
        aliases: vec![r#"I press Escape"#.to_string()],
        category: "Input".to_string(),
        description: "Press the Escape key".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "press_key".to_string(),
        pattern: r#"I press "([^"]+)" key"#.to_string(),
        aliases: vec![],
        category: "Input".to_string(),
        description: "Press a keyboard key".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "press_tab".to_string(),
        pattern: r#"I press Tab key"#.to_string(),
        aliases: vec![r#"I press Tab"#.to_string()],
        category: "Input".to_string(),
        description: "Press the Tab key".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "release_drag".to_string(),
        pattern: r#"I release drag on "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: release_drag".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "remove_local_storage".to_string(),
        pattern: r#"I remove local storage item "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: remove_local_storage".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "remove_session_storage".to_string(),
        pattern: r#"I remove session storage item "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: remove_session_storage".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "resume_animation".to_string(),
        pattern: r#"I resume animation "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: resume_animation".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "retry_click".to_string(),
        pattern: r#"I retry clicking "([^"]+)" up to (\d+) times"#.to_string(),
        aliases: vec![r#"I retry "([^"]+)" (\d+) times"#.to_string()],
        category: "Other".to_string(),
        description: "Step: retry_click".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "right_click".to_string(),
        pattern: r#"I right click on "([^"]+)""#.to_string(),
        aliases: vec![
            r#"I right-click "([^"]+)""#.to_string(),
            r#"I click the right mouse button on "([^"]+)""#.to_string(),
        ],
        category: "Interaction".to_string(),
        description: "Right-click an element".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "rotate_element".to_string(),
        pattern: r#"I rotate "([^"]+)" by (\d+) degrees?"#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: rotate_element".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "save_file_as".to_string(),
        pattern: r#"I save file as "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: save_file_as".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "screenshot".to_string(),
        pattern: r#"I take a screenshot "([^"]+)""#.to_string(),
        aliases: vec![r#"I capture screenshot "([^"]+)""#.to_string()],
        category: "Extraction".to_string(),
        description: "Take a screenshot".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "screenshot_element".to_string(),
        pattern: r#"I take screenshot of "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: screenshot_element".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "scroll_position_check".to_string(),
        pattern: r#"I should see scroll position (\d+)%"#.to_string(),
        aliases: vec![],
        category: "Scrolling".to_string(),
        description: "Check scroll position".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "scroll_to_element".to_string(),
        pattern: r#"I scroll to "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Scrolling".to_string(),
        description: "Scroll to a specific element".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "scroll_to_percentage".to_string(),
        pattern: r#"I scroll to position (\d+)%"#.to_string(),
        aliases: vec![],
        category: "Scrolling".to_string(),
        description: "Scroll to percentage".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "select_all".to_string(),
        pattern: r#"I select multiple options from "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Input".to_string(),
        description: "Select all options in a multi-select".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "select_all_text_in_element".to_string(),
        pattern: r#"I select all text in "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: select_all_text_in_element".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "select_autocomplete_suggestion".to_string(),
        pattern: r#"I select autocomplete suggestion "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: select_autocomplete_suggestion".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "select_filter_option".to_string(),
        pattern: r#"I select filter option "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: select_filter_option".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "select_multiple".to_string(),
        pattern: r#"I select "([^"]+)", "([^"]+)", and "([^"]+)" from "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Input".to_string(),
        description: "Select multiple options".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "select_option".to_string(),
        pattern: r#"I select "([^"]+)" from "([^"]+)""#.to_string(),
        aliases: vec![
            r#"I choose "([^"]+)" from "([^"]+)""#.to_string(),
            r#"I pick "([^"]+)" from "([^"]+)""#.to_string(),
        ],
        category: "Input".to_string(),
        description: "Select an option from a dropdown".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "select_radio".to_string(),
        pattern: r#"I select the "([^"]+)" radio (?:button|option)"#.to_string(),
        aliases: vec![],
        category: "Input".to_string(),
        description: "Select a radio button option".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "select_text_range".to_string(),
        pattern: r#"I select text from "([^"]+)" to "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: select_text_range".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "send_websocket_message".to_string(),
        pattern: r#"I send WebSocket message "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: send_websocket_message".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "set_document_lang".to_string(),
        pattern: r#"I set document language to "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: set_document_lang".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "set_local_storage".to_string(),
        pattern: r#"I set local storage item "([^"]+)" to "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "State".to_string(),
        description: "Set local storage value".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "set_print_layout".to_string(),
        pattern: r#"I set print layout to "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: set_print_layout".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "set_session_storage".to_string(),
        pattern: r#"I set session storage item "([^"]+)" to "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "State".to_string(),
        description: "Set session storage value".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "set_user_agent".to_string(),
        pattern: r#"I set user agent to "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: set_user_agent".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "set_webgl_context".to_string(),
        pattern: r#"I set WebGL context to "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: set_webgl_context".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "should_be_checked".to_string(),
        pattern: r#"the element "([^"]+)" should be checked"#.to_string(),
        aliases: vec![],
        category: "Verification".to_string(),
        description: "Verify checkbox is checked".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "should_be_disabled".to_string(),
        pattern: r#"the element "([^"]+)" should be disabled"#.to_string(),
        aliases: vec![],
        category: "Verification".to_string(),
        description: "Verify element is disabled".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "should_be_enabled".to_string(),
        pattern: r#"the element "([^"]+)" should be enabled"#.to_string(),
        aliases: vec![],
        category: "Verification".to_string(),
        description: "Verify element is enabled".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "should_be_focused".to_string(),
        pattern: r#"the element "([^"]+)" should be focused"#.to_string(),
        aliases: vec![],
        category: "Verification".to_string(),
        description: "Verify element has focus".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "should_be_selected".to_string(),
        pattern: r#"the element "([^"]+)" should be selected"#.to_string(),
        aliases: vec![],
        category: "Verification".to_string(),
        description: "Verify element is selected".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "should_be_sorted_by".to_string(),
        pattern: r#"the items should be sorted by "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: should_be_sorted_by".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "should_be_visible".to_string(),
        pattern: r#"the element "([^"]+)" should be visible"#.to_string(),
        aliases: vec![r#""([^"]+)" should be visible"#.to_string()],
        category: "Verification".to_string(),
        description: "Verify element is visible".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "should_contain_text".to_string(),
        pattern: r#"the element "([^"]+)" should contain "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Verification".to_string(),
        description: "Verify element contains text".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "should_exist".to_string(),
        pattern: r#"the element "([^"]+)" should exist"#.to_string(),
        aliases: vec![],
        category: "Verification".to_string(),
        description: "Verify element exists".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "should_not_be_checked".to_string(),
        pattern: r#"the element "([^"]+)" should not be checked"#.to_string(),
        aliases: vec![],
        category: "Verification".to_string(),
        description: "Verify checkbox is not checked".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "should_not_be_visible".to_string(),
        pattern: r#"the element "([^"]+)" should not be visible"#.to_string(),
        aliases: vec![r#""([^"]+)" should not be visible"#.to_string()],
        category: "Verification".to_string(),
        description: "Verify element is not visible".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "should_not_contain_text".to_string(),
        pattern: r#"the element "([^"]+)" should not contain "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Verification".to_string(),
        description: "Verify element does not contain text".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "should_not_exist".to_string(),
        pattern: r#"the element "([^"]+)" should not exist"#.to_string(),
        aliases: vec![],
        category: "Verification".to_string(),
        description: "Verify element does not exist".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "should_not_see".to_string(),
        pattern: r#"I should not see "([^"]+)""#.to_string(),
        aliases: vec![r#"I should not see the element "([^"]+)""#.to_string()],
        category: "Verification".to_string(),
        description: "Verify an element is not visible".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "should_not_see_text".to_string(),
        pattern: r#"I should not see text "([^"]+)""#.to_string(),
        aliases: vec![r#"the page should not contain "([^"]+)""#.to_string()],
        category: "Verification".to_string(),
        description: "Verify text is not on page".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "should_receive_websocket_message".to_string(),
        pattern: r#"I should receive WebSocket message "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: should_receive_websocket_message".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "should_see".to_string(),
        pattern: r#"I should see "([^"]+)""#.to_string(),
        aliases: vec![r#"I should see the element "([^"]+)""#.to_string()],
        category: "Verification".to_string(),
        description: "Verify an element is visible".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "should_see_active_filter".to_string(),
        pattern: r#"I should see active filter "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: should_see_active_filter".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "should_see_active_tab".to_string(),
        pattern: r#"I should see active tab "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: should_see_active_tab".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "should_see_animation".to_string(),
        pattern: r#"I should see animation "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: should_see_animation".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "should_see_autocomplete".to_string(),
        pattern: r#"I should see autocomplete suggestions for "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: should_see_autocomplete".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "should_see_breadcrumb".to_string(),
        pattern: r#"I should see breadcrumb "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: should_see_breadcrumb".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "should_see_exact_count_elements".to_string(),
        pattern: r#"there should be (\d+) "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Verification".to_string(),
        description: "Verify exact count of specific elements".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "should_see_notification".to_string(),
        pattern: r#"I should see notification "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: should_see_notification".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "should_see_text".to_string(),
        pattern: r#"I should see text "([^"]+)""#.to_string(),
        aliases: vec![r#"the page should contain "([^"]+)""#.to_string()],
        category: "Verification".to_string(),
        description: "Verify text is present on page".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "should_see_tooltip".to_string(),
        pattern: r#"I should see tooltip "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: should_see_tooltip".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "skip_if_visible".to_string(),
        pattern: r#"skip the rest of the scenario if "([^"]+)" is (visible|present)"#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: skip_if_visible".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "sort_by".to_string(),
        pattern: r#"I sort by "([^"]+)" in (?:ascending|descending) order"#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: sort_by".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "store_value".to_string(),
        pattern: r#"I set "([^"]+)" to "([^"]+)""#.to_string(),
        aliases: vec![
            r#"I save "([^"]+)" as "([^"]+)""#.to_string(),
            r#"I store "([^"]+)" as "([^"]+)""#.to_string(),
        ],
        category: "State".to_string(),
        description: "Store value in variable".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "submit_form".to_string(),
        pattern: r#"I submit the form "([^"]+)""#.to_string(),
        aliases: vec![r#"I submit "([^"]+)""#.to_string()],
        category: "Input".to_string(),
        description: "Submit a form".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "swipe_elements".to_string(),
        pattern: r#"I swipe "([^"]+)" to "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: swipe_elements".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "switch_to_frame".to_string(),
        pattern: r#"I switch to frame "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Navigation".to_string(),
        description: "Switch to an iframe".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "switch_to_window".to_string(),
        pattern: r#"I switch to window "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: switch_to_window".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "text_should_be".to_string(),
        pattern: r#"the text of "([^"]+)" should be "([^"]+)""#.to_string(),
        aliases: vec![r#"the text of "([^"]+)" should equal "([^"]+)""#.to_string()],
        category: "Verification".to_string(),
        description: "Verify element text equals value".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "text_should_contain".to_string(),
        pattern: r#"the text of "([^"]+)" should contain "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Verification".to_string(),
        description: "Verify element text contains value".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "text_should_end".to_string(),
        pattern: r#"the text of "([^"]+)" should end with "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Verification".to_string(),
        description: "Verify element text ends with value".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "text_should_match".to_string(),
        pattern: r#"the text of "([^"]+)" should match "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Verification".to_string(),
        description: "Verify element text matches pattern".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "text_should_start".to_string(),
        pattern: r#"the text of "([^"]+)" should start with "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Verification".to_string(),
        description: "Verify element text starts with value".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "title_should_be".to_string(),
        pattern: r#"the title should be "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Verification".to_string(),
        description: "Verify page title".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "title_should_contain".to_string(),
        pattern: r#"the title should contain "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Verification".to_string(),
        description: "Verify page title contains value".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "toggle".to_string(),
        pattern: r#"I toggle "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Input".to_string(),
        description: "Toggle a checkbox or switch".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "tooltip_should_contain".to_string(),
        pattern: r#"the tooltip should contain "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: tooltip_should_contain".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "touch_element".to_string(),
        pattern: r#"I touch "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: touch_element".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "type_in_search_box".to_string(),
        pattern: r#"I type in search box "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: type_in_search_box".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "type_into".to_string(),
        pattern: r#"I press "([^"]+)" in "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Input".to_string(),
        description: "Type into a specific element".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "type_into_prompt".to_string(),
        pattern: r#"I type "([^"]+)" into the prompt"#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: type_into_prompt".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "type_text".to_string(),
        pattern: r#"I type "([^"]+)" into "([^"]+)""#.to_string(),
        aliases: vec![
            r#"I enter "([^"]+)" into "([^"]+)""#.to_string(),
            r#"I type "([^"]+)" (?:in|into) "([^"]+)""#.to_string(),
            r#"I enter "([^"]+)" (?:in|into) "([^"]+)""#.to_string(),
            r#"I fill "([^"]+)" with "([^"]+)""#.to_string(),
            r#"I fill in "([^"]+)" with "([^"]+)""#.to_string(),
            r#"I input "([^"]+)" (?:in|into) "([^"]+)""#.to_string(),
        ],
        category: "Input".to_string(),
        description: "Type text into an input field".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "uncheck".to_string(),
        pattern: r#"I uncheck "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Input".to_string(),
        description: "Uncheck a checkbox".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "upload_file".to_string(),
        pattern: r#"I upload file "([^"]+)" to "([^"]+)""#.to_string(),
        aliases: vec![
            r#"I attach "([^"]+)" to "([^"]+)""#.to_string(),
            r#"I upload file "([^"]+)" to "([^"]+)""#.to_string(),
        ],
        category: "Input".to_string(),
        description: "Upload a file".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "url_should_be".to_string(),
        pattern: r#"the URL should be "([^"]+)""#.to_string(),
        aliases: vec![r#"the current URL should be "([^"]+)""#.to_string()],
        category: "Verification".to_string(),
        description: "Verify current URL".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "url_should_contain".to_string(),
        pattern: r#"the URL should contain "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Verification".to_string(),
        description: "Verify URL contains value".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "use_stored_value".to_string(),
        pattern: r#"I use the stored value "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: use_stored_value".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "value_should_be".to_string(),
        pattern: r#"the value "([^"]+)" should be "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: value_should_be".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "verify_canvas_pixel".to_string(),
        pattern: r#"I verify canvas pixel at "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: verify_canvas_pixel".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "verify_download".to_string(),
        pattern: r#"I verify file "([^"]+)" was downloaded"#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: verify_download".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "wait_appear".to_string(),
        pattern: r#"I wait for element "([^"]+)" to appear"#.to_string(),
        aliases: vec![
            r#"I wait for "([^"]+)" to appear"#.to_string(),
            r#"I wait until "([^"]+)" appears"#.to_string(),
        ],
        category: "Waiting".to_string(),
        description: "Wait for an element to appear on the page".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "wait_clickable".to_string(),
        pattern: r#"I wait for element "([^"]+)" to be (?:clickable|enabled)"#.to_string(),
        aliases: vec![],
        category: "Waiting".to_string(),
        description: "Wait for element to be clickable".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "wait_for_element_text".to_string(),
        pattern: r#"I wait for element "([^"]+)" to contain "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Waiting".to_string(),
        description: "Wait for element to contain text".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "wait_for_text".to_string(),
        pattern: r#"I wait for text "([^"]+)" to appear"#.to_string(),
        aliases: vec![],
        category: "Waiting".to_string(),
        description: "Wait for text to appear".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "wait_hidden".to_string(),
        pattern: r#"I wait for element "([^"]+)" to be hidden"#.to_string(),
        aliases: vec![r#"I wait for "([^"]+)" to disappear"#.to_string()],
        category: "Waiting".to_string(),
        description: "Wait for an element to become hidden".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "wait_visible".to_string(),
        pattern: r#"I wait for element "([^"]+)" to be visible"#.to_string(),
        aliases: vec![
            r#"I wait for "([^"]+)" to be visible"#.to_string(),
            r#"I wait until "([^"]+)" is visible"#.to_string(),
        ],
        category: "Waiting".to_string(),
        description: "Wait for an element to become visible".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "wait_with_timeout".to_string(),
        pattern: r#"I wait for "([^"]+)" with timeout of (\d+) seconds"#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: wait_with_timeout".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog.add_step(StepInfo {
        id: "webgl_context_check".to_string(),
        pattern: r#"the WebGL should have context "([^"]+)""#.to_string(),
        aliases: vec![],
        category: "Other".to_string(),
        description: "Step: webgl_context_check".to_string(),
        parameters: vec![],
        examples: vec![],
    });

    catalog
}
