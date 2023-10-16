mod nu_ron;
use nu_plugin::{serve_plugin, EvaluatedCall, LabeledError, MsgPackSerializer, Plugin};
use nu_protocol::{Category, PluginExample, PluginSignature, Spanned, Value};

struct Ron;

impl Ron {
    fn new() -> Self {
        Self {}
    }
}

impl Plugin for Ron {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("ron")
            .usage("View ron results")
            //.required("path", SyntaxShape::String, "path to ron input file")
            .category(Category::Experimental)
            .plugin_examples(vec![PluginExample {
                description: "This is the example descripion".into(),
                example: "some pipeline involving ron".into(),
                result: None,
            }]),
            PluginSignature::build("from ron")
            .usage("convert ron to record")
            .category(Category::Experimental)
            .plugin_examples(vec![PluginExample {
                description: "form example description".into(),
                example: "open file.ron -r | from ron".into(),
                result: None,
            }])]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        match name {
            "ron" => {let param: Option<Spanned<String>> = call.opt(0)?;

                let ret_val = match input {
                    Value::String { val, internal_span } => {
                        crate::nu_ron::ron_do_something(param, val, *internal_span)?
                    }
                    v => {
                        return Err(LabeledError {
                            label: "Expected something from pipeline".into(),
                            msg: format!("requires some input, got {}", v.get_type()),
                            span: Some(call.head),
                        });
                    }
                };
        
                Ok(ret_val)}
                "from ron" => {let param: Option<Spanned<String>> = call.opt(0)?;
                    match input {
                        Value::String { val, internal_span } => {
                            crate::nu_ron::from_ron(param, val, *internal_span)
                        }
                        v => {
                            return Err(LabeledError { 
                                label: format!("incorrect type {}", v.get_type()), 
                                msg: "failed".into(), 
                                span: Some(call.head) });
                        }
                    }
                    
                }
                _ => return Err(LabeledError { 
                    label: "Not suported ron action".into(), 
                    msg: "current suported options are \"ron\" and \"from ron\" \n see usage for more info".into(), 
                    span: Some(call.head) })
        }
        
    }
}

fn main() {
    serve_plugin(&mut Ron::new(), MsgPackSerializer);
}
