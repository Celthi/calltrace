use anyhow::Result;
use calltrace::calls::CallStacks;
use calltrace::filter;
use calltrace::quote;
fn main() -> Result<()> {
    let f = "test_data/gts_callstack.txt";
    let m = quote::TrimPrefixMatch::new("Write of size".to_string());
    let e1 = quote::TrimExactMatch::new("\n".to_string());
    let callstack_begin:Vec<&dyn quote::MatchQuote> = vec![&m];
    let callstack_end:Vec<&dyn quote::MatchQuote> = vec![&e1];
    let css = CallStacks::from_file(f, &callstack_begin, &callstack_end)?;

    let m = quote::ExcludeKeywordMatch::new("ATL::CComPtr<ICDSSSource>::Release()".to_string());
    let filter_condition = filter::TopFrameFilter::new(&m);
    let css = filter::filter(&css, &filter_condition);

    println!("size = {}", css.len());
    println!("{}", css);
    Ok(())
}
