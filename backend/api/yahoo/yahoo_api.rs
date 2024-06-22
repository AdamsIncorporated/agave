mod YahooAPI {

    #[derive(Clone, Serialize, Copy)]
    struct Data {
        time: u64,
        close: f64,
    }
    
    pub async fn fetch_data(ticker: &str, interval: &str) -> Option<Data> {
        let provider = yahoo::YahooConnector::new();
        let response = tokio_test::block_on(provider.get_latest_quotes(ticker, interval)).unwrap();
        let quote = response.last_quote().unwrap();
        let data = Data {
            time: quote.timestamp,
            close: quote.close,
        };

        Some(data)
    }
}
