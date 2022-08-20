use calltrace::calls::CallStacks;
use calltrace::quote;
use calltrace::folder;
use anyhow::Result;
fn main() -> Result<()>{
    let f = "test_data/gts_callstack.txt";
    let mut callstack_start:Vec<&dyn quote::MatchQuote> = vec![];
    let m =  quote::TrimPrefixMatch::new("Write of size".to_string());
    callstack_start.push(&m);  
    let e1 = quote::TrimExactMatch::new("\n".to_string());
    let css = CallStacks::from_file(f, &callstack_start, &[&e1])?;

    let folder = folder::Folder::new(&css);
    println!("size = {}", folder.len());
    println!("{}", folder);    
    Ok(())
}
