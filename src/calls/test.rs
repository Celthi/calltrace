
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
    let s1 = quote::TrimExtractMatch::new("AddRef:".to_string());
    let s2 = quote::TrimExtractMatch::new("RelRef:".to_string());
    let e1 = quote::TrimExtractMatch::new("\n".to_string());
    let css = CallStacks::from_string(s, &[&s1, &s2], &[&e1]);
    assert_eq!(css.size(), 2);
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
    let s1 = quote::TrimExtractMatch::new("Callstack:".to_string());
    let e1 = quote::TrimExtractMatch::new("Callstack end".to_string());
    let css = CallStacks::from_string(s, &[&s1], &[&e1]);
    assert_eq!(css.size(), 2);
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
