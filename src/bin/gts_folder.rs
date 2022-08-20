use calltrace::calls::CallStacks;
use calltrace::quote;
use calltrace::folder;
use anyhow::Result;
fn main() -> Result<()>{
    let f = "test_data/gts_callstack.txt";
    let mut matches:Vec<&dyn quote::MatchQuote> = vec![];
    let m =  quote::TrimPrefixMatch::new("Write of size".to_string());
    matches.push(&m);  
    let e1 = quote::TrimExactMatch::new("\n".to_string());
    let css = CallStacks::from_file(f, &matches, &[&e1])?;

    let folder = folder::Folder::new(&css);
    println!("size = {}", folder.size());
    println!("{}", folder);    
    Ok(())
}
