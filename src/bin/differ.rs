use calltrace::calls::CallStacks;
use calltrace::calls::quote;
use calltrace::differ;
use anyhow::Result;
fn main() ->Result<()> {
    let base_file = "test_data/No_LeakRefLeakStacks.txt";
    let target_file = "test_data/RefLeakStacks.txt";
    let m1 = quote::TrimExtractMatch::new("AddRef:".to_string());

    let m2 = quote::TrimExtractMatch::new("RelRef:".to_string());
    let e1 = quote::TrimExtractMatch::new("\n".to_string());
    let cs_base = CallStacks::from_file(base_file, &[&m1, &m2], &[&e1])?;
    let cs_target = CallStacks::from_file(target_file, &[&m1, &m2], &[&e1])?;
    
    let diff = differ::Differ::not_in(&cs_target, &cs_base);

    println!("size = {}:", diff.data.len());
    println!("{}", diff);
    Ok(())
}
