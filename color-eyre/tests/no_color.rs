use color_eyre::{Section, eyre::Report};

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
struct TestError(&'static str);

#[test]
#[cfg(not(miri))]
fn report_has_no_ansi_codes_when_colors_are_unsupported() {
    // Hooks are installed once per process, so this test lives in its own file
    unsafe {
        std::env::set_var("NO_COLOR", "1");
        std::env::remove_var("CLICOLOR_FORCE");
    }

    #[cfg(feature = "capture-spantrace")]
    {
        use tracing_subscriber::prelude::*;
        tracing_subscriber::registry()
            .with(tracing_error::ErrorLayer::default())
            .init();
    }

    color_eyre::install().unwrap();

    let report = {
        let _span = tracing::info_span!("no_color_span", field = 1).entered();
        Report::msg("test")
            .note("note")
            .warning("warning")
            .suggestion("suggestion")
            .error(TestError("error"))
    };
    let output = format!("{report:?}");

    assert!(output.contains("suggestion"), "{output}");
    assert!(!output.contains('\x1b'), "{output:?}");
}
