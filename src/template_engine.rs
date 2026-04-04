use anyhow::Result;
use handlebars::Handlebars;
use serde_json::{json, Value};
use std::collections::BTreeMap;

/// Template rendering engine using Handlebars
/// This replaces ad-hoc string replacement with a proper templating system
pub struct TemplateEngine {
    handlebars: Handlebars<'static>,
}

impl TemplateEngine {
    pub fn new() -> Result<Self> {
        let handlebars = Handlebars::new();
        Ok(TemplateEngine { handlebars })
    }

    /// Register a template from a string
    pub fn register_template(&mut self, name: &str, template: &str) -> Result<()> {
        self.handlebars
            .register_template_string(name, template)
            .map_err(|e| anyhow::anyhow!("Failed to register template '{}': {}", name, e))
    }

    /// Render a template with context
    pub fn render(&self, template_name: &str, context: &Value) -> Result<String> {
        self.handlebars
            .render(template_name, context)
            .map_err(|e| anyhow::anyhow!("Failed to render template '{}': {}", template_name, e))
    }

    /// Render inline template with context (without registration)
    pub fn render_inline(&self, template_str: &str, context: &Value) -> Result<String> {
        self.handlebars
            .render_template(template_str, context)
            .map_err(|e| anyhow::anyhow!("Failed to render inline template: {}", e))
    }
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create template engine")
    }
}

/// Helper to build context as JSON
pub struct ContextBuilder {
    values: BTreeMap<String, Value>,
}

impl ContextBuilder {
    pub fn new() -> Self {
        ContextBuilder {
            values: BTreeMap::new(),
        }
    }

    pub fn set(mut self, key: &str, value: impl Into<Value>) -> Self {
        self.values.insert(key.to_string(), value.into());
        self
    }

    pub fn array(mut self, key: &str, items: Vec<impl Into<Value>>) -> Self {
        let array: Vec<Value> = items.into_iter().map(|v| v.into()).collect();
        self.values.insert(key.to_string(), Value::Array(array));
        self
    }

    pub fn object(mut self, key: &str, map: BTreeMap<String, Value>) -> Self {
        let mut json_map = serde_json::Map::new();
        for (k, v) in map {
            json_map.insert(k, v);
        }
        self.values
            .insert(key.to_string(), Value::Object(json_map));
        self
    }

    pub fn build(self) -> Value {
        json!(self.values)
    }
}

impl Default for ContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_template() {
        let mut engine = TemplateEngine::new().unwrap();
        engine
            .register_template("hello", "Hello, {{name}}!")
            .unwrap();

        let context = ContextBuilder::new().set("name", "World").build();
        let result = engine.render("hello", &context).unwrap();

        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_context_builder() {
        let context = ContextBuilder::new()
            .set("title", "My App")
            .set("version", "1.0.0")
            .array(
                "items",
                vec![
                    json!({"name": "item1"}),
                    json!({"name": "item2"}),
                ],
            )
            .build();

        assert_eq!(context["title"], "My App");
        assert_eq!(context["version"], "1.0.0");
        assert_eq!(context["items"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_inline_render() {
        let engine = TemplateEngine::new().unwrap();
        let context = ContextBuilder::new()
            .set("project_name", "my-app")
            .set("version", "0.1.0")
            .build();

        let result = engine
            .render_inline(
                r#"{
  "name": "{{project_name}}",
  "version": "{{version}}"
}"#,
                &context,
            )
            .unwrap();

        assert!(result.contains("\"name\": \"my-app\""));
        assert!(result.contains("\"version\": \"0.1.0\""));
    }
}
