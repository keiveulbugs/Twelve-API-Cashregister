pub mod products;

#[derive(Debug, PartialEq)]
pub struct TwelveClient {
    privatekey: String,
    publickey: String,
    clientid: i32,
}

impl TwelveClient {
    /// Build a new client containing a private key, a public key and clientid.
    pub fn new(privatekey: String, publickey: String, clientid: i32) -> TwelveClient {
        TwelveClient {
            privatekey: privatekey,
            publickey: publickey,
            clientid,
        }
    }
    pub async fn header(self) -> Result<HeaderMap, Error> {
        headermap("/group".to_string(), self).await
    }

    /// Get a list of all products
    /// deletedlines: Show deleted products as well when set to true
    /// subclientid: Filter based on subclients
    pub async fn getproducts(self, deletedlines: Option<bool>, subclientid: Option<i32>) -> GetProductCall{
        products::getproducts::getproducts(self, deletedlines, subclientid)
            .await
            .unwrap()
    }
}

#[cfg(test)]
mod clienttest {
    use super::*;

    #[test]
    fn buildingaclientworks() {
        let client = TwelveClient::new("PRIVATEKEY".to_string(), "PUBLICKEY".to_string(), 6404);
        assert_eq!(
            client,
            TwelveClient {
                privatekey: "PRIVATEKEY".to_string(),
                publickey: "PUBLICKEY".to_string(),
                clientid: 6404
            }
        );
    }
    #[tokio::test]
    async fn headermaptest() {
        let result = headermap(
            "/test".to_string(),
            TwelveClient {
                privatekey: "privatekey".to_string(),
                publickey: "publickey".to_string(),
                clientid: 6404,
            },
        )
        .await;
        assert_eq!(true, result.is_ok());
    }

    #[tokio::test]
    async fn getproductstest() {
        let privatekey = dotenvreader("privatekey".to_string()).unwrap();
        let publickey = dotenvreader("publickey".to_string()).unwrap();
        let clientid = dotenvreader("clientid".to_string())
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let client = TwelveClient::new(privatekey, publickey, clientid);
        let test = TwelveClient::getproducts(client, Some(true), None).await;
    }

    /// Reads the .env file and tries to find the .env variable.
    ///
    /// # Example
    /// ```rust
    /// use macro_env::dotenvreader;
    ///
    /// let envvariable :String = dotenvreader("OS".to_string()).unwrap();
    /// ```
    fn dotenvreader(envvariablename: String) -> Result<String, std::io::Error> {
        let mut token = String::new();

        let content = include_str!("../.env");

        for line in content.lines() {
            if line.clone().starts_with('#') {
                continue;
            };
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2
                && parts[0] == envvariablename
                && !parts[1].is_empty()
                && !line.starts_with('#')
            {
                token = parts[1].to_string();
                break;
            } else {
                continue;
            }
        }

        if token.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "There doesn't seem to be a variable in the .env",
            ));
        }

        if token.ends_with('"') && token.starts_with('"') {
            token.pop();
            token.remove(0);
        };

        Ok(token)
    }
}

use chrono::Utc;
use products::getproducts::GetProductCall;
use rand::distributions::Alphanumeric;
use rand::Rng;
use reqwest::header::{HeaderMap, ACCEPT};
use std::io::Error;
/// Create a headermap
/// We are parsing the data, which returns a possible error.
/// It should never return an error, but if it does,
/// it should exit the function immediately to avoid the function pushing invalid data to Twelve
/// Each call needs a few basic Headers:
/// ```json
/// -H 'accept: text/plain' \
/// -H 'PublicAPIKey: ....' \
/// -H 'RequestToken: ....' \
/// -H 'RequestSignature: ...' \
/// -H 'ClientId: ....' \
/// -H 'Content-Type: application/json' \
/// ```
/// - Public api key is obvious, I hope.
/// - Request token is a token that starts with the current date followed by a random string of characters which needs to be unique every time!.
/// - Request signature is a SHA256 Hash existing out of the endpoint path formatted as `/api/v1/tokens`, the requesttoken, and your privatekey. Just stitch those 3 strings together and hash them. **It is very important to make these ASCII UPPERCASE characaters, otherwise it won't work.**
/// - The ClientId.
pub async fn headermap(path: String, client: TwelveClient) -> Result<HeaderMap, Error> {
    // create time in yyyymmdd (i.e. 20230808)
    let date = Utc::now().format("%Y%d%m");

    // generate a string of 30 random characters
    let randomstring: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    // Combine date and randomstring to get a requestkey
    let requesttoken = format!("{date}{randomstring}");
    // generate a SHA256 hash from path+requestkey+privatekey
    let hash =
        sha256::digest(format!("{path}{requesttoken}{}", client.privatekey)).to_ascii_uppercase();

    // Create a Headermap
    // We are parsing the data, which returns a possible error.
    // It should never return an error, but if it does,
    // it should exit the function immediately to avoid the function pushing invalid data to Twelve
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        match "text/plain".parse() {
            Ok(val) => val,
            Err(err) => {
                println!("There was an error while parsing data\n{err}");
                return Err(Error::new(std::io::ErrorKind::InvalidData, err));
            }
        },
    );
    headers.insert(
        "PublicAPIKey",
        match client.publickey.parse() {
            Ok(val) => val,
            Err(err) => {
                println!("There was an error while parsing data\n{err}");
                return Err(Error::new(std::io::ErrorKind::InvalidData, err));
            }
        },
    );
    headers.insert(
        "RequestToken",
        match requesttoken.parse() {
            Ok(val) => val,
            Err(err) => {
                println!("There was an error while parsing data\n{err}");
                return Err(Error::new(std::io::ErrorKind::InvalidData, err));
            }
        },
    );
    headers.insert(
        "RequestSignature",
        match hash.parse() {
            Ok(val) => val,
            Err(err) => {
                println!("There was an error while parsing data\n{err}");
                return Err(Error::new(std::io::ErrorKind::InvalidData, err));
            }
        },
    );
    headers.insert("ClientId", client.clientid.into());
    Ok(headers)
}
