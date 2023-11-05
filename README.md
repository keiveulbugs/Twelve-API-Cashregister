# A wrapper around the Twelve api
The goal of this crate is to form a wrapper around the Twelve API. This to make the use of the API easier as the current API of Twelve is just not that ergonomic to work with. The API calls added will be based on what I personally need first. Especially as there are 36 different API endpoints with almost all multiple functions (get, post, patch delete).

I have currently not yet decided on how to implement the structure of the crate, *and there will be breaking changes*

The original Twelve API documentation can be found [here](https://clientapi.twelve.eu/index.html). You can request your publickey, privatekey and clientid from Twelve by emailing them.

**NOTE:** If you use the API without this crate, please remember that the API keys should be in pure UPPERCASE. Otherwise you will get an authentication error.



## ToDo
*This that still need to be done to make the wrapper support the api 100%*
- [x] create client
- [x] create headermap
- [ ] accountgroups
- [ ] accountinggroups
- [ ] accounts
- [ ] billing
- [ ] colours
- [ ] deposits
- [ ] eventcalendar
- [ ] eventproductcategory
- [ ] eventproductpricerules
- [ ] kiosks
- [ ] logins
- [ ] paymenttransactions
- [ ] printticketclass
- [ ] printticketdesigns
- [ ] printticketgroup
- [ ] productbuttons
- [ ] productcategories
- [ ] productgroups
- [ ] products
- [ ] registration
- [ ] revenuetransactions
- [ ] revenuetransactionsbookkeeping
- [ ] subclients
- [ ] tabs
- [ ] tabtransactions
- [ ] testservice
- [ ] tokens
- [ ] topuptransactions
- [ ] transactioncosttypes
- [ ] transactiontypes
- [ ] users
- [ ] vat
- [ ] virtualdevciereportgroups
- [ ] virtualdevices
- [ ] realtimestock
- [ ] reportrevenue
- [ ] reportrevenuebyproductgroup