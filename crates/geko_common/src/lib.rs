/// Prints error, and then
/// exits proccess using `std::process::exit(1)`.
#[macro_export]
macro_rules! bail {
    ($report:expr) => {{
        let report: miette::Report = $report.into();
        panic!("{report:?}");
    }};
}
