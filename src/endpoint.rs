use std::fmt::Display;

use crate::BASE_URL;

/// Enum representing the available endpoints.
pub(crate) enum NbrApiEndpoint {
    /// Corresponds to the latest available rates.
    Latest,
    /// Corresponds to the last 10 days rates.
    Last10Days,
    /// Corresponds to the historical rates for a given year.
    Historical(usize),
}

impl Display for NbrApiEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{}",
            BASE_URL,
            match self {
                NbrApiEndpoint::Latest => "nbrfxrates.xml".to_owned(),
                NbrApiEndpoint::Last10Days => "nbrfxrates10days.xml".to_owned(),
                NbrApiEndpoint::Historical(year) =>
                    format!("files/xml/years/nbrfxrates{}.xml", year),
            }
        )
    }
}
