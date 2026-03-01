/// Prints the error then
/// exits proccess using `std::process::exit(1)`.
#[macro_export]
macro_rules! bail {
    ($report:expr) => {{
        let report: miette::Report = $report.into();
        panic!("{report:?}");
    }};
}

/// Prints bug error then
/// exits proccess using `std::process::exit(1)`.
#[macro_export]
macro_rules! bug {
    ($text:expr) => {{
        panic!("{:?}", miette::miette!("hello"));
    }};
}
