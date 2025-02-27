use nu_test_support::{nu, pipeline};

#[test]
fn adds_a_row_to_the_end() {
    let actual = nu!(
        cwd: ".", pipeline(
        r#"
                echo  [ "Andrés N. Robalino", "Jonathan Turner", "Yehuda Katz" ]
                | append "pollo loco"
                | get 3
        "#
    ));

    assert_eq!(actual.out, "pollo loco");
}

#[test]
fn fail_on_non_iterator() {
    let actual = nu!(cwd: ".", pipeline("1 | append 3"));

    assert!(actual.err.contains("only_supports_this_input_type"));
}
