#![allow(dead_code, unused_imports, unused_must_use)]
pub mod quickjs_sys;

fn args_parse() -> Vec<String> {
    use argparse::ArgumentParser;
    let mut res_args: Vec<String> = vec![];
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut res_args)
            .add_argument("arg", argparse::List, "arg");
        ap.parse_args_or_exit();
    }
    res_args
}

fn main() {
    use quickjs_sys as q;
    let mut ctx = q::Context::new();
    // include js code
    let code = include_str!("../example_js/demo.js");
    // get args and set into quickjs
    let mut res_args = args_parse();
    res_args.insert(0, "<embedded_no_filename>".to_string());
    ctx.put_args(res_args);
    // run js code
    ctx.eval_str(code, "");
}
