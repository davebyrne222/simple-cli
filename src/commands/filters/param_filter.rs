use std::collections::HashMap;
use tera::{Value, Filter, Result as TeraResult, Error};
use crate::config::GlobalContext;
use crate::commands::kubernetes::namespace::get_user_namespace_choice;

pub struct ParamFilter {
    ctx: GlobalContext,
    arg_overrides: HashMap<String, String>,
}

impl ParamFilter {
    pub fn new(ctx: &GlobalContext, args: &HashMap<String, String>) -> Self {
        Self {
            ctx: ctx.clone(),
            arg_overrides: args.clone(),
        }
    }
}

impl Filter for ParamFilter {
    fn filter(&self, value: &Value, _args: &HashMap<String, Value>) -> TeraResult<Value> {
        // If an override is present, return it
        if let Some(arg_override) = self.arg_overrides.get(value.as_str().unwrap()) {
            return Ok(Value::from(arg_override.clone()));
        }

        // Else use context
        match tera::try_get_value!("param", "value", String, value).as_str() {
            "namespace" => {
                let ns_arg = get_user_namespace_choice(&self.ctx)
                    .map_err(|e| Error::msg(e))?;
                Ok(Value::from(ns_arg))
            },
            other => {
                let msg = format!("Unknown param: {}", other);
                eprintln!("{}", msg);
                Err(Error::msg(msg))
            }
        }
    }
}