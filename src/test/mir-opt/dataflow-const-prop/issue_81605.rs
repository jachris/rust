// unit-test: DataflowConstProp

// EMIT_MIR issue_81605.main.DataflowConstProp.diff
fn main() {
    let x = 1 + if true { 1 } else { 2 };
    std::process::exit(x);
}
