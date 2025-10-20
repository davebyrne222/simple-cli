/** Interactive menu runner. */
use std::collections::HashMap;
use crate::config::models::{Config, GlobalContext, CommandDef};
use crate::commands::{find_command, run_command};
use crate::commands::arguments::collect_arguments;
use dialoguer::{Select, console::Term};

#[derive(Clone, Debug)]
enum MenuLevel {
    Root,
    Category(usize),
    SubCategory(usize, usize),
}

#[derive(Clone, Debug)]
enum MenuItem {
    NavigateCategory(usize),
    NavigateSubCategory(usize, usize),
    ExecuteCommand(usize, Option<usize>, usize),
    GoBack,
    Exit,
    Header, // Non-interactive header row
}

enum MenuAction {
    Navigate(MenuLevel),
    Exit,
    Stay,
}

struct Menu {
    labels: Vec<String>,
    items: Vec<MenuItem>,
}

impl Menu {
    fn new() -> Self {
        Self {
            labels: Vec::new(),
            items: Vec::new(),
        }
    }

    fn add(&mut self, label: String, item: MenuItem) {
        self.labels.push(label);
        self.items.push(item);
    }

    fn add_header(&mut self, label: String) {
        self.add(label, MenuItem::Header);
    }

    fn add_navigation_actions(&mut self, at_root: bool) {
        if !at_root {
            self.add("◀ Go Back".to_string(), MenuItem::GoBack);
        }
        self.add("◀ Exit".to_string(), MenuItem::Exit);
    }
}

/**
Run interactive mode with hierarchical navigation.
*/
pub fn run_interactive(
    cfg: &Config,
    ctx: &mut GlobalContext,
    arg_overrides: &HashMap<String, String>,
) {
    let mut current_level = MenuLevel::Root;

    loop {
        let menu = build_menu_for_level(&current_level, cfg);
        let prompt = create_prompt_for_level(&current_level, cfg);

        match present_menu_and_get_selection(&menu, &prompt) {
            Ok((selected_item, prompt_lines, items_lines)) => {
                // Clear the just-printed menu immediately after selection,
                // so headers don't linger and previously printed command/output stay intact.
                let _ = Term::stdout().clear_last_lines(prompt_lines + items_lines);

                let action = handle_menu_selection(selected_item, cfg, ctx, arg_overrides, &current_level);
                match action {
                    MenuAction::Navigate(new_level) => {
                        current_level = new_level;
                    }
                    MenuAction::Stay => {
                        continue;
                    }
                    MenuAction::Exit => {
                        break;
                    }
                }
            }
            Err(_) => break,
        }
    }
}

/// Build menu items for the current navigation level
fn build_menu_for_level(level: &MenuLevel, cfg: &Config) -> Menu {
    match level {
        MenuLevel::Root => build_root_menu(cfg),
        MenuLevel::Category(ci) => build_category_menu(*ci, cfg),
        MenuLevel::SubCategory(ci, si) => build_subcategory_menu(*ci, *si, cfg),
    }
}

fn build_root_menu(cfg: &Config) -> Menu {
    let mut menu = Menu::new();

    // Add categories
    for (idx, category) in cfg.categories.iter().enumerate() {
        menu.add(category.category.clone(), MenuItem::NavigateCategory(idx));
    }

    if menu.labels.is_empty() {
        println!("No categories defined in commands.yaml");
    }

    // Add actions
    menu.add_navigation_actions(true);

    menu
}

fn build_category_menu(category_idx: usize, cfg: &Config) -> Menu {
    let mut menu = Menu::new();
    let category = &cfg.categories[category_idx];

    // Add subcategories
    for (idx, subcategory) in category.subcategories.iter().enumerate() {
        menu.add(
            subcategory.name.clone(),
            MenuItem::NavigateSubCategory(category_idx, idx),
        );
    }

    // Add commands in this category
    if !category.commands.is_empty() {
        add_commands_to_menu(&mut menu, &category.commands, category_idx, None);
    }

    menu.add_navigation_actions(false);
    menu
}

fn build_subcategory_menu(category_idx: usize, subcategory_idx: usize, cfg: &Config) -> Menu {
    let mut menu = Menu::new();
    let subcategory = &cfg.categories[category_idx].subcategories[subcategory_idx];

    // Add commands
    if !subcategory.commands.is_empty() {
        add_commands_to_menu(&mut menu, &subcategory.commands, category_idx, Some(subcategory_idx));
    }

    menu.add_navigation_actions(false);
    menu
}

fn add_commands_to_menu(
    menu: &mut Menu,
    commands: &[CommandDef],
    category_idx: usize,
    subcategory_idx: Option<usize>,
) {
    let (table_rows, max_name_width, max_desc_width) = format_commands_table(commands);

    // Add table header
    let header = format!(
        "{:<name$}  {:<desc$}  {}",
        "NAME",
        "DESCRIPTION",
        "ARGS",
        name = max_name_width,
        desc = max_desc_width
    );
    menu.add_header(header);

    // Add separator
    let separator = format!(
        "{:<name$}  {:<desc$}  {}",
        "-".repeat(max_name_width),
        "-".repeat(max_desc_width),
        "-".repeat(4),
        name = max_name_width,
        desc = max_desc_width
    );
    menu.add_header(separator);

    // Add command rows
    for (idx, row) in table_rows.iter().enumerate() {
        menu.add(
            row.clone(),
            MenuItem::ExecuteCommand(category_idx, subcategory_idx, idx),
        );
    }
}

fn format_commands_table(commands: &[CommandDef]) -> (Vec<String>, usize, usize) {
    // Determine terminal width; default to 80 if unknown.
    let term_cols: usize = {
        #[allow(unused_imports)]
        use dialoguer::console::Term;
        let (_rows, cols) = Term::stdout().size();
        let c = cols as usize;
        if c == 0 { 80 } else { c }
    };

    // Simple word-wrapping within a fixed width. Falls back to hard-wrap for long words.
    fn wrap_text(s: &str, width: usize) -> Vec<String> {
        if width == 0 { return vec![String::new()]; }
        let mut lines = Vec::new();
        let mut line = String::new();

        for word in s.split_whitespace() {
            if line.is_empty() {
                if word.chars().count() <= width {
                    line.push_str(word);
                } else {
                    // Hard-wrap long word
                    let mut buf = String::new();
                    for ch in word.chars() {
                        buf.push(ch);
                        if buf.chars().count() == width {
                            lines.push(buf);
                            buf = String::new();
                        }
                    }
                    if !buf.is_empty() {
                        line.push_str(&buf);
                    } else if line.is_empty() {
                        // already pushed exact-width chunks; start new line
                        line.clear();
                    }
                }
            } else {
                let pending = if line.is_empty() { word.chars().count() } else { 1 + word.chars().count() };
                if line.chars().count() + pending <= width {
                    if !line.is_empty() { line.push(' '); }
                    line.push_str(word);
                } else {
                    lines.push(line);
                    line = String::new();
                    // place word on new line (hard-wrap if needed)
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
        }
        if !line.is_empty() || lines.is_empty() {
            lines.push(line);
        }
        lines
    }

    // Intrinsic desired widths
    let intrinsic_name_w = commands.iter()
        .map(|c| c.name.len())
        .max()
        .unwrap_or(4)
        .max("NAME".len());

    // Column layout heuristic
    let inter_col_spaces = 2 /*between name & desc*/ + 2 /*between desc & args*/;

    // Keep NAME width consistent across all rows: at least the longest name
    let min_name_w = "NAME".len().max(12);
    let name_w = intrinsic_name_w.max(min_name_w);

    let min_desc_w = "DESCRIPTION".len().max(16);
    let min_args_w = "ARGS".len().max(12);

    // Initial allocation: split remaining width between desc and args with bias to desc
    let remaining = term_cols.saturating_sub(name_w).saturating_sub(inter_col_spaces);
    let mut args_w = (remaining / 3).max(min_args_w).min(30);
    let mut desc_w = remaining.saturating_sub(args_w);

    // Ensure DESCRIPTION meets its minimum by borrowing from ARGS only (NAME stays fixed)
    if desc_w < min_desc_w {
        let need = min_desc_w - desc_w;
        let can_take = args_w.saturating_sub(min_args_w);
        let take = need.min(can_take);
        args_w -= take;
        desc_w += take;
        // If still too small due to extremely narrow terminal, leave as-is (best effort)
    }

    // Enforce a maximum width for the DESCRIPTION column
    let desc_w_cap = 80usize;
    if desc_w > desc_w_cap {
        desc_w = desc_w_cap;
        // Reflow remaining width to ARGS if possible
        let new_remaining = term_cols.saturating_sub(name_w).saturating_sub(inter_col_spaces);
        args_w = new_remaining.saturating_sub(desc_w).max(min_args_w);
    }

    // Helper to clip text to a given width with an ellipsis when needed (used for NAME only).
    fn clip_text(s: &str, width: usize) -> String {
        if width == 0 { return String::new(); }
        let len = s.chars().count();
        if len <= width {
            s.to_string()
        } else if width <= 1 {
            "…".to_string()
        } else {
            let mut out = String::with_capacity(width);
            for (i, ch) in s.chars().enumerate() {
                if i >= width - 1 { break; }
                out.push(ch);
            }
            out.push('…');
            out
        }
    }

    // Build wrapped rows
    let rows: Vec<String> = commands.iter()
        .map(|cmd| {
            // NAME: do not wrap; clip to fixed width for consistency
            let name_disp = clip_text(&cmd.name, name_w);
            // DESCRIPTION and ARGS: wrap within their column widths
            let desc_lines = wrap_text(&cmd.description, desc_w);
            let args_lines = wrap_text(&format_command_args(cmd), args_w);
            let max_lines = 1usize.max(desc_lines.len()).max(args_lines.len());

            let mut out_lines = Vec::with_capacity(max_lines);
            for i in 0..max_lines {
                let n = if i == 0 { name_disp.as_str() } else { "" };
                let d = desc_lines.get(i).map(String::as_str).unwrap_or("");
                let a = args_lines.get(i).map(String::as_str).unwrap_or("");
                out_lines.push(format!("{:<name$}  {:<desc$}  {}", n, d, a, name = name_w, desc = desc_w));
            }
            out_lines.join("\n")
        })
        .collect();

    (rows, name_w, desc_w)
}

fn format_command_args(cmd: &CommandDef) -> String {
    if cmd.args.is_empty() {
        return "-".to_string();
    }

    cmd.args.iter()
        .map(|arg| {
            if arg.optional {
                format!("[{}]", arg.name)
            } else {
                arg.name.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn create_prompt_for_level(level: &MenuLevel, cfg: &Config) -> String {
    match level {
        MenuLevel::Root => "Select a category or action".to_string(),
        MenuLevel::Category(ci) => {
            format!("Category: {} — choose subcategory or command", cfg.categories[*ci].category)
        }
        MenuLevel::SubCategory(ci, si) => {
            format!(
                "Subcategory: {} / {} — choose a command",
                cfg.categories[*ci].category,
                cfg.categories[*ci].subcategories[*si].name
            )
        }
    }
}

fn present_menu_and_get_selection(menu: &Menu, prompt: &str) -> Result<(MenuItem, usize, usize), ()> {
    // Split headers (non-selectable) from real items (selectable)
    let mut header_lines: Vec<String> = Vec::new();
    let mut display_labels: Vec<String> = Vec::new();
    let mut display_items: Vec<MenuItem> = Vec::new();

    for (label, item) in menu.labels.iter().zip(menu.items.iter()) {
        match item {
            MenuItem::Header => header_lines.push(label.clone()),
            _ => {
                display_labels.push(label.clone());
                display_items.push(item.clone());
            }
        }
    }

    // Compose a multi-line prompt with header/separator above the choices
    let composed_prompt = if header_lines.is_empty() {
        prompt.to_string()
    } else {
        format!("{}\n{}", prompt, header_lines.join("\n"))
    };

    // Present the menu and capture the result
    let selection_opt = Select::new()
        .with_prompt(composed_prompt.as_str())
        .items(&display_labels)
        .default(0)
        .interact_opt()
        .map_err(|_| ())?;

    // Count printed lines (prompt + items). Caller decides whether to clear.
    let prompt_lines = composed_prompt.lines().count();
    let items_lines: usize = display_labels.iter().map(|s| s.lines().count()).sum();

    // Map selection to the corresponding item
    let selection_idx = selection_opt.ok_or(())?;
    Ok((display_items[selection_idx].clone(), prompt_lines, items_lines))
}

fn handle_menu_selection(
    item: MenuItem,
    cfg: &Config,
    ctx: &mut GlobalContext,
    arg_overrides: &HashMap<String, String>,
    current_level: &MenuLevel,
) -> MenuAction {
    match item {
        MenuItem::Header => MenuAction::Stay,
        MenuItem::Exit => MenuAction::Exit,
        MenuItem::GoBack => MenuAction::Navigate(navigate_back(current_level)),
        MenuItem::NavigateCategory(idx) => MenuAction::Navigate(MenuLevel::Category(idx)),
        MenuItem::NavigateSubCategory(ci, si) => MenuAction::Navigate(MenuLevel::SubCategory(ci, si)),
        MenuItem::ExecuteCommand(ci, maybe_si, ki) => {
            handle_command_selection(cfg, ctx, arg_overrides, ci, maybe_si, ki);
            MenuAction::Stay
        }
    }
}

fn navigate_back(current_level: &MenuLevel) -> MenuLevel {
    match current_level {
        MenuLevel::Root => MenuLevel::Root,
        MenuLevel::Category(_) => MenuLevel::Root,
        MenuLevel::SubCategory(ci, _) => MenuLevel::Category(*ci),
    }
}

fn handle_command_selection(
    cfg: &Config,
    ctx: &mut GlobalContext,
    arg_overrides: &HashMap<String, String>,
    category_idx: usize,
    subcategory_idx: Option<usize>,
    command_idx: usize,
) {
    let cmd = match subcategory_idx {
        Some(si) => &cfg.categories[category_idx].subcategories[si].commands[command_idx],
        None => &cfg.categories[category_idx].commands[command_idx],
    };

    // Execute pre-command if specified
    if let Some(pre_cmd_name) = &cmd.pre_command {
        match find_command(cfg.categories.as_slice(), pre_cmd_name) {
            Some(pre_cmd) => {
                println!("Running pre-command: {} (`{}`)", pre_cmd.name, pre_cmd.exec);
                handle_command_execution(cfg, ctx, pre_cmd, arg_overrides)
            }
            None => eprintln!("Pre-command '{}' not found", pre_cmd_name),
        }
    }

    // Execute the main command
    handle_command_execution(cfg, ctx, cmd, arg_overrides);
    // Spacer between command output and the next interactive menu
    println!();
}

fn handle_command_execution(
    cfg: &Config,
    ctx: &mut GlobalContext,
    cmd: &CommandDef,
    arg_overrides: &HashMap<String, String>,
){
    let args = collect_arguments(cmd, arg_overrides);
    if let Err(e) = run_command(cmd, cfg, ctx, &args){
        eprintln!("Failed to execute command: {}", e)
    }
}