use clap::{App, AppSettings, Arg, ArgMatches};

mod boolean;
pub use boolean::clap_app as boolean;
pub use boolean::cli as boolean_cli;
mod clear;
pub use clear::clap_app as clear;
pub use clear::cli as clear_cli;
mod fit;
pub use fit::clap_app as fit_to_points;
pub use fit::cli as fit_to_points_cli;
mod refigure;
pub use refigure::clap_app as refigure;
pub use refigure::cli as refigure_cli;
