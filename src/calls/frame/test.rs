
use super::*;
#[test]
fn test_parse() {
    let inner =
        InnerFrame::parse("Z:\\Server\\xxx\\xx\\x\\zzzz.cpp( 6688 ): MIS_64.dll!zzzzz::ProcessH");
    assert!(inner.is_some());
    let inner = inner.unwrap();
    assert_eq!(inner.get_func().unwrap(), "MIS_64.dll!zzzzz::ProcessH");
    assert_eq!(
        inner.get_file().unwrap(),
        "Z:\\Server\\xxx\\xx\\x\\zzzz.cpp"
    );
    assert_eq!(inner.get_line().unwrap(), "6688");
}
