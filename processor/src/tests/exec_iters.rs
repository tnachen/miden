use super::super::build_test;

// EXEC ITER TESTS

#[test]
fn test_memory() {
    let source = "begin popw.mem.1 popw.mem.2 end";

    let test = build_test!(source, &[1, 2, 3, 4, 5, 6, 7, 8]);
    let traces = test.execute_iter();
    assert_eq!(1, traces.len());
}
