use crate::headermap;
use crate::TwelveClient;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProductCall {
    #[serde(rename = "lstProducts")]
    pub lst_products: Vec<LstProduct>,
}

/// The sctruct that parses all products returned from Twelve.
/// Beaware that it implements a default value if a field is missing
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LstProduct {
    pub id: i64,
    #[serde(rename = "subClientId")]
    #[serde(default)]
    pub sub_client_id: i64,
    #[serde(rename = "nameOnTerminal")]
    pub name_on_terminal: String,
    #[serde(rename = "nameOnReport")]
    pub name_on_report: String,
    pub price: f64,
    #[serde(rename = "priceForAccountGroup")]
    pub price_for_account_group: f64,
    #[serde(rename = "hasOpenPrice")]
    pub has_open_price: bool,
    #[serde(rename = "stockPrice")]
    pub stock_price: f64,
    #[serde(rename = "stockPriceNet")]
    pub stock_price_net: f64,
    #[serde(rename = "isSupplement")]
    pub is_supplement: bool,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "coinValue")]
    pub coin_value: f64,
    #[serde(rename = "VATId")]
    pub vatid: f64,
    #[serde(rename = "VATPercentage")]
    pub vatpercentage: f64,
    #[serde(rename = "DepositId")]
    pub deposit_id: f64,
    #[serde(rename = "defaultPrintKitchen")]
    pub default_print_kitchen: bool,
    #[serde(rename = "defaultPrintBar")]
    pub default_print_bar: bool,
    #[serde(rename = "minimumAge")]
    pub minimum_age: f64,
    #[serde(rename = "productParentId")]
    pub product_parent_id: f64,
    #[serde(rename = "accountingGroupId")]
    pub accounting_group_id: f64,
    #[serde(rename = "externalId")]
    pub external_id: String,
    #[serde(rename = "opensSupplementCategoryId")]
    pub opens_supplement_category_id: f64,
    #[serde(rename = "opensSupplementScreenAutomatically")]
    pub opens_supplement_screen_automatically: bool,
    #[serde(rename = "automaticSupplements")]
    pub automatic_supplements: Vec<AutomaticSupplement>,
    #[serde(rename = "isDiscount")]
    pub is_discount: bool,
    #[serde(rename = "isNoSale")]
    pub is_no_sale: bool,
    #[serde(rename = "discountPercentage")]
    pub discount_percentage: f64,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "accountGroupId")]
    pub account_group_id: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutomaticSupplement {
    #[serde(rename = "productId")]
    pub product_id: i64,
    pub count: i64,
}

/// getproducts fetches all products from the get products api call: https://clientapi.twelve.eu/api/v1/Products.
/// It requires the Client, and has optionally deletedlines and subclient as values. The latter two are inserted in the query path.
pub async fn getproducts(
    client: TwelveClient,
    deletedlines: Option<bool>,
    subclientid: Option<i32>,
) -> Result<GetProductCall, std::io::Error> {
    let urlpath = if deletedlines.is_some() && subclientid.is_some() {
        format!("https://clientapi.twelve.eu/api/v1/Products?filterSubClientId={}&includeDeletedLines={}", subclientid.unwrap(), deletedlines.unwrap())
    } else if deletedlines.is_some() {
        format!(
            "https://clientapi.twelve.eu/api/v1/Products?includeDeletedLines={}",
            deletedlines.unwrap()
        )
    } else if subclientid.is_some() {
        format!(
            "https://clientapi.twelve.eu/api/v1/Products?filterSubClientId={}",
            subclientid.unwrap(),
        )
    } else {
        String::from("https://clientapi.twelve.eu/api/v1/Products")
    };

    let getproductsheadermap = headermap(String::from("/api/v1/Products"), client)
        .await
        .unwrap();

    let reqwestclient = reqwest::Client::new();
    let apicall = reqwestclient
        .get(urlpath)
        .headers(getproductsheadermap)
        .send()
        .await
        .expect("Couldn't request api");

    if !apicall.status().is_success() {
        panic!("ripperoni")
    }

    //println!("{:#?}", apicall.text().await);

    let apicallformat = match apicall.json::<GetProductCall>().await {
        Ok(val) => val,
        Err(err) => {
            println!("There was an error while parsing the apicall into the Root struct\n{err}");
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err));
        }
    };

    Ok(apicallformat)
}
