use dialoguer::{Select, console::Term};
use colored::Colorize;
use crate::config::{CommandDef, Config, GlobalContext};
use crate::interactive::switchers::switch_subscription;

pub fn handle_switch_subscription(cfg: &Config, ctx: &mut GlobalContext) {
    if cfg.groups.is_empty() {
        println!("No groups configured.");
        return;
    }

    let mut names: Vec<String> = cfg.groups.keys().cloned().collect();
    names.sort();

    let default_idx = ctx
        .current_group
        .as_ref()
        .and_then(|cur| names.iter().position(|n| n == cur))
        .unwrap_or(0);

    if let Ok(choice) = Select::new()
        .with_prompt("Choose group")
        .items(&names)
        .default(default_idx)
        .interact()
    {
        let selected = &names[choice];
        switch_subscription(ctx, selected);
        println!("Switched group to {}", selected);
    }
}

/** Build ARGS column for a command, wrapping optional args in [brackets]. */
pub fn args_column(cmd: &CommandDef) -> String {
    if cmd.args.is_empty() {
        return "-".to_string();
    }
    let mut parts: Vec<String> = Vec::new();
    for a in &cmd.args {
        if a.optional {
            parts.push(format!("[{}]", a.name));
        } else {
            parts.push(a.name.clone());
        }
    }
    parts.join(", ")
}

/** Normalize a name: lowercase and remove all whitespace. */
fn normalize_name(s: &str) -> String {
    s.to_lowercase().chars().filter(|c| !c.is_whitespace()).collect()
}

/** Render a formatted table (NAME, DESCRIPTION, ARGS) for a slice of commands, with a prefixed name.
    The printed NAME is category[.subcategory].name, where the prefix part is styled bright_black()
    but the width/padding is computed from the plain (uncolored) text so alignment is preserved. */
pub fn render_table(cmds: &[CommandDef], prefix: &str) -> Vec<String> {
    // Terminal width (fallback to 80 if unknown)
    let term_cols: usize = {
        let (_rows, cols) = Term::stdout().size();
        let c = cols as usize;
        if c == 0 { 80 } else { c }
    };

    // Normalize the prefix for consistent, copyable names
    let plain_prefix = normalize_name(prefix.trim());

    // Fixed NAME width: longest full name (plain), at least 12
    let min_name_w = "NAME".len().max(12);
    let name_w = cmds
        .iter()
        .map(|c| {
            if plain_prefix.is_empty() { c.name.len() } else { plain_prefix.len() + 1 + c.name.len() }
        })
        .max()
        .unwrap_or(min_name_w)
        .max(min_name_w);

    // Layout constraints
    let inter_col_spaces = 2 /*name-desc*/ + 2 /*desc-args*/;
    let min_desc_w = "DESCRIPTION".len().max(16);
    let min_args_w = "ARGS".len().max(12);

    // Allocate remaining width between DESCRIPTION and ARGS (bias to DESCRIPTION)
    let remaining = term_cols.saturating_sub(name_w).saturating_sub(inter_col_spaces);
    let mut args_w = (remaining / 3).max(min_args_w).min(30);
    let mut desc_w = remaining.saturating_sub(args_w);

    // Ensure DESCRIPTION minimum by borrowing from ARGS only
    if desc_w < min_desc_w {
        let need = min_desc_w - desc_w;
        let can_take = args_w.saturating_sub(min_args_w);
        let take = need.min(can_take);
        args_w -= take;
        desc_w += take;
    }

    // Cap DESCRIPTION at 200; reflow to ARGS if needed
    let desc_w_cap = 100usize;
    if desc_w > desc_w_cap {
        desc_w = desc_w_cap;
        let new_remaining = term_cols.saturating_sub(name_w).saturating_sub(inter_col_spaces);
        args_w = new_remaining.saturating_sub(desc_w).max(min_args_w);
    }

    // Simple word-wrap helper (used for DESCRIPTION and ARGS)
    fn wrap_text(s: &str, width: usize) -> Vec<String> {
        if width == 0 { return vec![String::new()]; }
        let mut lines = Vec::new();
        let mut line = String::new();
        for word in s.split_whitespace() {
            let pending = if line.is_empty() { word.chars().count() } else { 1 + word.chars().count() };
            if line.chars().count() + pending <= width {
                if !line.is_empty() { line.push(' '); }
                line.push_str(word);
            } else {
                if !line.is_empty() { lines.push(line); line = String::new(); }
                // place word on new line; hard-wrap if single word > width
                if word.chars().count() <= width {
                    line.push_str(word);
                } else {
                    let mut buf = String::new();
                    for ch in word.chars() {
                        buf.push(ch);
                        if buf.chars().count() == width {
                            lines.push(buf);
                            buf = String::new();
                        }
                    }
                    line.push_str(&buf);
                }
            }
        }
        if !line.is_empty() || lines.is_empty() {
            lines.push(line);
        }
        lines
    }

    // Pad a colored string to the desired visible width (counting only non-ANSI chars)
    fn pad_ansi(colored: String, visible_width: usize, total_width: usize) -> String {
        let pad_spaces = total_width.saturating_sub(visible_width);
        if pad_spaces == 0 { colored } else { format!("{}{}", colored, " ".repeat(pad_spaces)) }
    }

    // Header and underline
    let header = format!("{:<name_w$}  {:<desc_w$}  {}", "NAME", "DESCRIPTION", "ARGS", name_w = name_w, desc_w = desc_w);
    let underline = format!("{:<name_w$}  {:<desc_w$}  {}", "-".repeat(name_w), "-".repeat(desc_w), "-".repeat("ARGS".len()), name_w = name_w, desc_w = desc_w);

    let mut rows: Vec<String> = vec![header, underline];

    // Rows with fixed NAME width (first line shows colored full name; continuations are blank in NAME)
    for c in cmds {
        let plain_full = if plain_prefix.is_empty() {
            c.name.clone()
        } else {
            format!("{}.{}", plain_prefix, c.name)
        };
        let colored_full = if plain_prefix.is_empty() {
            c.name.clone()
        } else {
            format!("{}.{}", plain_prefix.bright_black(), c.name)
        };

        let desc_lines = wrap_text(&c.description, desc_w);
        let args_lines = wrap_text(&args_column(c), args_w);
        let max_lines = desc_lines.len().max(args_lines.len()).max(1);

        for i in 0..max_lines {
            let name_cell = if i == 0 {
                pad_ansi(colored_full.clone(), plain_full.len(), name_w)
            } else {
                " ".repeat(name_w)
            };
            let desc_cell = desc_lines.get(i).map(String::as_str).unwrap_or("");
            let args_cell = args_lines.get(i).map(String::as_str).unwrap_or("");
            rows.push(format!("{}  {:<desc_w$}  {}", name_cell, desc_cell, args_cell, desc_w = desc_w));
        }
    }

    rows
}

/**
Simple command listing grouped by categories and subcategories, using the same
NAME/DESCRIPTION/ARGS table format as interactive mode.
*/
pub fn list_commands(cfg: &Config) {
    for cat in &cfg.categories {
        println!("\n{}", cat.category.bold().on_blue());

        let cat_prefix = normalize_name(&cat.category);
        if !cat.commands.is_empty() {
            for line in render_table(&cat.commands, &cat_prefix) {
                println!("  {}", line);
            }
        }

        for sub in &cat.subcategories {
            println!("\n  {}", sub.name.blue());
            if !sub.commands.is_empty() {
                let sub_prefix = format!("{}.{}", cat_prefix, normalize_name(&sub.name));
                for line in render_table(&sub.commands, &sub_prefix) {
                    println!("  {}", line);
                }
            }
        }
    }
}