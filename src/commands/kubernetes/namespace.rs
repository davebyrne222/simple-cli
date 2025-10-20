use std::process::Command;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use serde_json::Value;

pub fn get_user_namespace_choice() -> Result<String, String> {

    // get namespaces
    let output = Command::new("kubectl")
        .args(["get", "namespaces", "-o", "json"])
        .output()
        .map_err(|e| format!("Failed to run kubectl: {}", e))?; // handle OS-level command errors

    if !output.status.success() {
        return Err(format!(
            "kubectl command failed: status={:?}, stderr={}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let json: Value = serde_json::from_slice(&output.stdout)
        .expect("Failed to parse kubectl output as JSON");

    let items = json["items"].as_array().expect("Invalid JSON: no items field");

    let mut namespaces: Vec<String> = items
        .iter()
        .filter_map(|item| item["metadata"]["name"].as_str().map(|s| s.to_string()))
        .collect();

    if namespaces.is_empty() {
        eprintln!("No namespaces found.");
    }

    // Build selection list
    namespaces.insert(0, "context".to_string());
    namespaces.insert(1, "all".to_string());

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a Kubernetes namespace")
        .default(0)
        .items(&namespaces)
        .interact()
        .expect("Dialoguer selection failed");

    let namespace_arg = match selection {
        0 => "".to_string(),
        1 => "--all-namespaces".to_string(),
        n => format!(
            "-n {}",
            namespaces
                .get(n)
                .ok_or_else(|| format!("Invalid selection: {}", n))?
        ),
    };

    Ok(namespace_arg)
}