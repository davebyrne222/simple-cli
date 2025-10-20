use crate::commands::filters::param_filter::ParamFilter;
use crate::config::{CommandDef, UserParams};
use std::collections::HashMap;
use tera::{Context, Tera};

pub fn render_cmd(
    cmd: &CommandDef,
    params: &UserParams,
    args: &HashMap<String, String>,
) -> Result<String, tera::Error> {
    let mut tera = Tera::default();
    tera.add_raw_template("cmd", cmd.exec.as_str())?;
    tera.register_filter("i_param", ParamFilter::new(args));

    // create context
    let mut context = Context::new();

    // Expose generic user params directly as `config` for templates: {{ config.<key> }}
    context.insert("params", &params.fields);

    for (k, v) in args.iter() {
        context.insert(k, v);
    }

    // Render command
    let rendered = tera.render("cmd", &context)?;
    Ok(rendered)
}
