use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize, Default)]
#[serde(default, rename_all = "PascalCase")]
pub struct DataSet {
    pub header: Header,
    pub body: Body,
}

#[derive(Debug, PartialEq, Deserialize, Default)]
#[serde(default, rename_all = "PascalCase")]
pub struct Header {
    pub publisher: String,
    pub publishing_date: NaiveDate,
    pub message_type: String,
}

#[derive(Debug, PartialEq, Deserialize, Default)]
#[serde(default, rename_all = "PascalCase")]
pub struct Body {
    pub subject: String,
    pub orig_currency: String,
    pub cube: Vec<Cube>,
}

#[derive(Debug, PartialEq, Deserialize, Default)]
#[serde(default, rename_all = "PascalCase")]
pub struct Cube {
    #[serde(rename = "@date")]
    date: NaiveDate,
    #[serde(rename = "Rate")]
    rates: Vec<Rate>,
}

#[derive(Debug, PartialEq, Deserialize, Default)]
#[serde(default, rename_all = "PascalCase")]
pub struct Rate {
    #[serde(rename = "@currency")]
    pub currency: String,
    #[serde(rename = "$text")]
    pub value: f64,
    #[serde(rename = "@multiplier")]
    pub multiplier: Option<u8>,
}

#[cfg(test)]
mod test {
    use crate::response::DataSet;

    const XML: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<DataSet xmlns="http://www.bnr.ro/xsd" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://www.bnr.ro/xsd nbrfxrates.xsd">
	<Header>
		<Publisher>National Bank of Romania</Publisher>
		<PublishingDate>2023-11-17</PublishingDate>
		<MessageType>DR</MessageType>
	</Header>
	<Body>
		<Subject>Reference rates</Subject>
		<OrigCurrency>RON</OrigCurrency>
		<Cube date="2023-11-17">
			<Rate currency="AED">1.2456</Rate>
			<Rate currency="AUD">2.9772</Rate>
			<Rate currency="BGN">2.5417</Rate>
			<Rate currency="BRL">0.9407</Rate>
			<Rate currency="CAD">3.3340</Rate>
			<Rate currency="CHF">5.1573</Rate>
			<Rate currency="CNY">0.6348</Rate>
			<Rate currency="CZK">0.2034</Rate>
			<Rate currency="DKK">0.6665</Rate>
			<Rate currency="EGP">0.1479</Rate>
			<Rate currency="EUR">4.9711</Rate>
			<Rate currency="GBP">5.6865</Rate>
			<Rate currency="HUF" multiplier="100">1.3180</Rate>
			<Rate currency="INR">0.0550</Rate>
			<Rate currency="JPY" multiplier="100">3.0623</Rate>
			<Rate currency="KRW" multiplier="100">0.3543</Rate>
			<Rate currency="MDL">0.2559</Rate>
			<Rate currency="MXN">0.2662</Rate>
			<Rate currency="NOK">0.4205</Rate>
			<Rate currency="NZD">2.7415</Rate>
			<Rate currency="PLN">1.1342</Rate>
			<Rate currency="RSD">0.0424</Rate>
			<Rate currency="RUB">0.0514</Rate>
			<Rate currency="SEK">0.4338</Rate>
			<Rate currency="THB">0.1306</Rate>
			<Rate currency="TRY">0.1597</Rate>
			<Rate currency="UAH">0.1267</Rate>
			<Rate currency="USD">4.5751</Rate>
			<Rate currency="XAU">292.9384</Rate>
			<Rate currency="XDR">6.0728</Rate>
			<Rate currency="ZAR">0.2505</Rate>
		</Cube>
	</Body>
</DataSet>"#;

    #[test]
    fn it_works() {
        let t = quick_xml::de::from_str::<DataSet>(XML);

        assert!(t.is_ok());
    }
}
