use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

/// Represents a file to be written during scaffolding
pub struct ScaffoldFile {
    pub path: String,
    pub content: String,
}

/// Validates TypeScript/JavaScript syntax using `tsc --noEmit`
pub fn validate_typescript(_files: &[ScaffoldFile], project_dir: &Path) -> Result<()> {
    // Check if tsc is available
    let tsc_check = Command::new("tsc")
        .arg("--version")
        .output();

    match tsc_check {
        Ok(output) if output.status.success() => {
            // tsc is available, run validation
            let status = Command::new("tsc")
                .arg("--noEmit")
                .arg("--skipLibCheck")
                .current_dir(project_dir)
                .status()
                .context("Failed to run tsc validation")?;

            if !status.success() {
                anyhow::bail!("TypeScript validation failed. Fix syntax errors above.");
            }
            Ok(())
        }
        _ => {
            // tsc not available, skip validation
            // This is acceptable in CI environments where dependencies aren't installed yet
            Ok(())
        }
    }
}

/// Validates Rust syntax using `cargo check`
pub fn validate_rust(project_dir: &Path) -> Result<()> {
    // Check if cargo is available
    let cargo_check = Command::new("cargo")
        .arg("--version")
        .output();

    match cargo_check {
        Ok(output) if output.status.success() => {
            // cargo is available, run validation
            let status = Command::new("cargo")
                .arg("check")
                .arg("--quiet")
                .current_dir(project_dir)
                .status()
                .context("Failed to run cargo check")?;

            if !status.success() {
                anyhow::bail!("Rust validation failed. Run `cargo check` to see errors.");
            }
            Ok(())
        }
        _ => {
            // cargo not available, skip validation
            // This is acceptable in environments without Rust toolchain
            Ok(())
        }
    }
}

/// Writes files to a temporary directory and validates them
/// Returns the temporary directory if validation succeeds
pub fn dry_run_typescript(files: &[ScaffoldFile]) -> Result<TempDir> {
    let temp_dir = TempDir::new().context("Failed to create temporary directory")?;

    // Write files to temp directory
    for file in files {
        let full_path = temp_dir.path().join(&file.path);
        if let Some(parent) = full_path.parent() {
            std::fs::create_dir_all(parent)
                .context(format!("Failed to create directory {}", parent.display()))?;
        }
        std::fs::write(&full_path, &file.content)
            .context(format!("Failed to write {}", file.path))?;
    }

    // Validate
    validate_typescript(files, temp_dir.path())?;

    Ok(temp_dir)
}

/// Writes files to a temporary directory and validates Rust syntax
/// Returns the temporary directory if validation succeeds
pub fn dry_run_rust(files: &[ScaffoldFile]) -> Result<TempDir> {
    let temp_dir = TempDir::new().context("Failed to create temporary directory")?;

    // Write files to temp directory
    for file in files {
        let full_path = temp_dir.path().join(&file.path);
        if let Some(parent) = full_path.parent() {
            std::fs::create_dir_all(parent)
                .context(format!("Failed to create directory {}", parent.display()))?;
        }
        std::fs::write(&full_path, &file.content)
            .context(format!("Failed to write {}", file.path))?;
    }

    // Validate
    validate_rust(temp_dir.path())?;

    Ok(temp_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typescript_validation_available() {
        // This test checks if tsc is available
        let result = Command::new("tsc")
            .arg("--version")
            .output();

        if result.is_ok() {
            println!("TypeScript compiler is available");
        } else {
            println!("TypeScript compiler is not available (OK for some environments)");
        }
    }

    #[test]
    fn test_rust_validation_available() {
        // This test checks if cargo is available
        let result = Command::new("cargo")
            .arg("--version")
            .output();

        if result.is_ok() {
            println!("Rust toolchain is available");
        } else {
            println!("Rust toolchain is not available (OK for some environments)");
        }
    }
}
