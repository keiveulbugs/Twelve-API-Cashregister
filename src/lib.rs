pub fn add(left: usize, right: usize) -> usize {
    left + right
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[derive(Debug, PartialEq)]
struct Client {
    privatekey: String,
    publickey: String,
    clientid: u64,
  }
  
impl Client {
    /// Build a new client containing a private key, a public key and clientid.
    pub fn new(privatekey:String, publickey:String, clientid:u64) -> Client {
        Client { 
            privatekey : privatekey.to_ascii_lowercase(),
            publickey : publickey.to_ascii_lowercase(),
            clientid 
        }
    }
    pub async fn header(self) -> Result<HeaderMap, Error> {headermap("/group".to_string(), self).await}
    pub async fn accountgroeps(self) {

    }
}

use chrono::Utc;
use rand::distributions::Alphanumeric;
use rand::Rng;
use reqwest::header::{HeaderMap, ACCEPT};
use std::io::Error;





#[cfg(test)]
mod clienttest {
    use super::*;

    #[test]
    fn buildingaclientworks() {
        let client =  Client::new("privatekey".to_string(), "publickey".to_string(), 6404);
        assert_eq!(client, Client {privatekey: "privatekey".to_string(), publickey: "publickey".to_string(), clientid: 6404});
    }
    #[tokio::test]
    async fn headermaptest() {
        let result = headermap("/test".to_string(), Client {privatekey: "privatekey".to_string(), publickey: "publickey".to_string(), clientid: 6404}).await;
        assert_eq!(true, result.is_ok());
    }
}





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
async fn headermap(path: String, client :Client) -> Result<HeaderMap, Error> {


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
    let hash = sha256::digest(format!("{path}{requesttoken}{}", client.privatekey)).to_ascii_uppercase();

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
    headers.insert(
        "ClientId",
        client.clientid.into(),
    );
    Ok(headers)
}