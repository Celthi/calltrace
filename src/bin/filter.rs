use anyhow::Result;
use calltrace::calls::CallStacks;
use calltrace::filter;
use calltrace::quote;
fn main() -> Result<()> {
    let f = "test_data/gts_callstack.txt";
    let mut matches: Vec<&dyn quote::MatchQuote> = vec![];
    //let m1 =  quote::TrimPrefixMatch::new("Thread".to_string());
    let m2 = quote::TrimPrefixMatch::new("Write of size".to_string());
    matches.push(&m2);

    let e1 = quote::TrimExactMatch::new("\n".to_string());

    let css = CallStacks::from_file(f, &matches, &[&e1])?;
    let m = quote::ExcludeKeywordMatch::new("ATL::CComPtr<ICDSSSource>::Release()".to_string());

    let filter_condition = filter::TopFrameFilter::new(&m);
    let css = filter::filter(&css, &filter_condition);
    println!("size = {}", css.data.len());
    println!("{}", css);
    Ok(())
}
