/// Prints the error then
/// exits proccess using `std::process::exit(1)`.
#[macro_export]
macro_rules! bail {
    ($report:expr) => {{
        let report: miette::Report = $report.into();
        println!("{report:?}");
        std::process::exit(1)
    }};
}

/// Prints the error !without! using `std::process::exit(1)` then
#[macro_export]
macro_rules! emit {
    ($report:expr) => {{
        let report: miette::Report = $report.into();
        println!("{report:?}");
    }};
}

/// Prints bug error then
/// exits proccess using `std::process::exit(1)`.
#[macro_export]
macro_rules! bug {
    ($text:expr) => {{
        println!("{:?}", miette::miette!("hello"));
        std::process::exit(1)
    }};
}
