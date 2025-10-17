use crate::commands::filters::param_filter::ParamFilter;
use crate::config::{CommandDef, GlobalContext, Subscription};
use std::collections::HashMap;
use tera::{Context, Tera};

pub fn render_cmd(
    cmd: &CommandDef,
    config: &Subscription,
    ctx: &GlobalContext,
    args: &HashMap<String, String>,
) -> Result<String, tera::Error> {
    let mut tera = Tera::default();
    tera.add_raw_template("cmd", cmd.exec.as_str())?;
    tera.register_filter("i_param", ParamFilter::new(ctx, args));

    // create context
    let mut context = Context::new();

    context.insert("config", config);

    for (k, v) in args.iter() {
        context.insert(k, v);
    }

    // Render command
    let rendered = tera.render("cmd", &context)?;
    Ok(rendered)
}
