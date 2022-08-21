use crate::quote;

#[test]
fn extract_call_stack() {
    let s = r#"
        
AddRef:
f0
f2
f3

RelRef:
f1
f2
f3

        "#;
    use super::*;
    let s1 = quote::TrimExactMatch::new("AddRef:".to_string());
    let s2 = quote::TrimExactMatch::new("RelRef:".to_string());
    let e1 = quote::TrimExactMatch::new("\n".to_string());
    let css = CallStacks::from_string(s, &[&s1, &s2], &[&e1]);
    assert_eq!(css.len(), 2);
}

#[test]
fn output() {
    let s = r#"
        Callstack:
        f1
        f2
        Callstack end
        dfhdiadfad
        
        Callstack:
        f0
        f2
        Callstack end"#;
    use super::*;
    let s1 = quote::TrimExactMatch::new("Callstack:".to_string());
    let e1 = quote::TrimExactMatch::new("Callstack end".to_string());
    let css = CallStacks::from_string(s, &[&s1], &[&e1]);
    assert_eq!(css.len(), 2);
    let cs = CallStack::from_string(
        r"
        f1
        f2",
    );
    assert!(css.has(&cs));
    let cs = CallStack::from_string(
        r"
        f0
        f2",
    );
    assert!(css.has(&cs));
    let cs = CallStack::from_string(
        r"
        f0
        f2
        f3",
    );
    assert!(!css.has(&cs));
}
