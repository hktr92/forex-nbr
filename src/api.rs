use reqwest::Client;

use crate::{endpoint::NbrApiEndpoint, DataSet};

pub type NbrApiResponse = anyhow::Result<DataSet>;

/// API Client for National Bank of Romania
///
/// Usage:
/// ```rust
/// let api = forex_nbr::NbrApi::default();
///
/// // to fetch the latest available rates
/// let latest = api.latest().await?;
///
/// // to fetch the last 10 days rates
/// let last_10_days = api.last_10_days().await?;
///
/// // to fetch historical rates for a given year
/// let historical = api.historical(2019).await?;
///
/// ```
pub struct NbrApi {
    client: Client,
}

/// Implementing default for NbrApi is more than enough for most use cases.
/// It's not like they'll change the hostname or something.
impl Default for NbrApi {
    fn default() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

/// Public API implementation
impl NbrApi {
    /// Fetches the latest available rates.
    ///
    /// The data are updated in real time, shortly after 13:00, every banking day.
    ///
    /// In order to minimize the traffic we recommend to store information locally.
    pub async fn latest(&self) -> NbrApiResponse {
        self.call(NbrApiEndpoint::Latest).await
    }

    /// Fetches the last 10 days rates.
    pub async fn last_10_days(&self) -> NbrApiResponse {
        self.call(NbrApiEndpoint::Last10Days).await
    }

    /// Fetches the historical rates for a given year.
    pub async fn historical(&self, year: usize) -> NbrApiResponse {
        if year < 2005 {
            return Err(anyhow::anyhow!("Year must be greater than 2005"));
        }

        self.call(NbrApiEndpoint::Historical(year)).await
    }
}

impl NbrApi {
    /// Performs an HTTP GET request to the given endpoint, and returns the response as a DataSet.
    async fn call(&self, endpoint: NbrApiEndpoint) -> NbrApiResponse {
        let body = self
            .client
            .get(endpoint.to_string())
            .send()
            .await?
            .text()
            .await?;

        Ok(quick_xml::de::from_str::<DataSet>(&body)?)
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test_it_works() {
        let api = super::NbrApi::default();

        assert!(api.latest().await.is_ok());
    }
}
