/** Switchers for subscription and namespace (placeholders). */
use crate::config::models::GlobalContext;
use crate::config::context::set_last_used;

/** Switch current group and persist last-used value. */
#[allow(dead_code)]
pub fn switch_subscription(ctx: &mut GlobalContext, name: &str) {
    ctx.current_group = Some(name.to_string());
    set_last_used("group", name);
}