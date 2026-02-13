/// Imports
use crate::assert_sema;

// Note: should bail
#[test]
fn test_sema_1() {
    assert_sema!(
        r#"
        break;
        "#
    )
}

// Note: should bail
#[test]
fn test_sema_2() {
    assert_sema!(
        r#"
        continue;
        "#
    )
}

// Note: should bail
#[test]
fn test_sema_3() {
    assert_sema!(
        r#"
        return;
        "#
    )
}
