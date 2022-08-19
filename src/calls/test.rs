
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
    let sep = Quotes {
        start: &["AddRef:", "RelRef:"],
        end: &["\n"],
    };
    let css = CallStacks::from_string(s, &sep);
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
    let sep = Quotes {
        start: &["Callstack:"],
        end: &["Callstack end"],
    };
    let css = CallStacks::from_string(s, &sep);
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
