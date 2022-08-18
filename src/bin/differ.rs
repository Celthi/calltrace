use calltrace::callstack;
use calltrace::callstack::Separate;
use calltrace::differ;
use anyhow::Result;
fn main() ->Result<()> {
    let base_file = "test/No_LeakRefLeakStacks.txt";
    let target_file = "test/RefLeakStacks.txt";
    let sep = Separate {
        start: &["AddRef:", "RelRef:"],
        end: &["\n"],
    };
    let cs_base = callstack::CallStacks::from_file(base_file, &sep)?;
    let cs_target = callstack::CallStacks::from_file(target_file, &sep)?;
    
    let diff = differ::Differ::not_in(&cs_target, &cs_base);

    println!("size = {}:", diff.data.len());
    println!("{}", diff);
    Ok(())
}
