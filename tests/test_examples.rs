use pretty_assertions::assert_eq;

#[test]
fn test_example_complex_app() {
    mod complex_app {
        include!("../docs/examples/complex_app.rs");
    }

    assert_eq!(
        clap_markdown_dfir::help_markdown::<complex_app::Cli>(),
        include_str!("../docs/examples/complex-app.md")
    );
}
