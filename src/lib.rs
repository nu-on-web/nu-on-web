mod utils;

use nu_cmd_extra::ToHtml;
use nu_cmd_lang::{create_default_context, eval_block, parse};
use nu_command::{Find, FromCsv, FromJson, Help, Sort, ToCsv, ToJson};
use nu_protocol::{engine::StateWorkingSet, PipelineData, Value};
use std::path::Path;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
mod ls;
mod open;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn consolelog(s: &str);
}

#[wasm_bindgen]
pub fn run_code(code: String) -> String {
    set_panic_hook();
    let mut ctx = create_default_context();
    let mut s = StateWorkingSet::new(&ctx);
    macro_rules! bind_command {
        ( $( $command:expr ),* $(,)? ) => {
            $( s.add_decl(Box::new($command)); )*
        };
    }
    bind_command! {
        Sort,
        ToHtml,
        FromJson,
        ToJson,
        FromCsv,
        ToCsv,
        Find,
        Help,
        ls::Ls,
        open::Open,
    }
    ctx.merge_delta(s.delta).expect("Failed to merge delte");

    let (block, _) = parse(&code, &ctx);
    let val = eval_block(block, PipelineData::empty(), Path::new("/tmp"), &ctx);
    return serde_json::to_string(&val).expect("Failed serizling to string!");
}
