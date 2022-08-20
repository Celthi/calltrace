use calltrace::calls::CallStacks;
use calltrace::calls::quote;
use anyhow::Result;
fn main() -> Result<()>{
    let f = "test_data/gts_callstack.txt";
    let mut matches:Vec<&dyn quote::MatchQuote> = vec![];
    //let m1 =  quote::TrimPrefixMatch::new("Thread".to_string());
    let m2 =  quote::TrimPrefixMatch::new("Write of size".to_string());
    matches.push(&m2);
    
    let e1 = quote::TrimExtractMatch::new("\n".to_string());

    let css = CallStacks::from_file(f, &matches, &[&e1])?;
    println!("size = {}", css.data.len());
    println!("{}", css);    
    Ok(())
}
