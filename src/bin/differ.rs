use calltrace::calls::CallStacks;
use calltrace::quote;
use calltrace::differ;
use anyhow::Result;
fn main() ->Result<()> {
    let base_file = "test_data/No_LeakRefLeakStacks.txt";
    let target_file = "test_data/RefLeakStacks.txt";
    let m1 = quote::TrimExactMatch::new("AddRef:".to_string());
    let m2 = quote::TrimExactMatch::new("RelRef:".to_string());
    let e1 = quote::TrimExactMatch::new("\n".to_string());
    let callstack_begin:Vec<&dyn quote::MatchQuote> = vec![&m1, &m2];
    let callstack_end:Vec<&dyn quote::MatchQuote> = vec![&e1];
    let cs_base = CallStacks::from_file(base_file, &callstack_begin, &callstack_end)?;
    let cs_target = CallStacks::from_file(target_file, &callstack_begin, &callstack_end)?;
    
    let diff = differ::Differ::not_in(&cs_target, &cs_base);

    println!("size = {}:", diff.map.len());
    println!("{}", diff);
    Ok(())
}
