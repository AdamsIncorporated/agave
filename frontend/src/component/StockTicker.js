import React, { useState, useEffect } from 'react';
import './StockTicker.css'; // Import your CSS file

const StockTicker = () => {
  const [ticker, setTicker] = useState('');
  const [stockData, setStockData] = useState(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState(null);

  const apiKey = 'YOUR_API_KEY'; // Replace with your actual API key

  const fetchData = async () => {
    setIsLoading(true);
    setError(null);

    try {
      const response = await fetch(`https://api.polygon.io/v2/aggs/ticker/${ticker}/prev?apiKey=${apiKey}`);
      if (!response.ok) {
        throw new Error(`Error fetching data for ${ticker}`);
      }
      const data = await response.json();
      setStockData(data.results[0]);
    } catch (error) {
      setError(error.message);
    } finally {
      setIsLoading(false);
    }
  };

  const handleSubmit = (event) => {
    event.preventDefault();
    fetchData();
  };

  useEffect(() => {
    // Clear any previous errors on ticker change
    setError(null);
  }, [ticker]);

  return (
    <div className="stock-ticker">
      <h1>Stock Ticker</h1>
      <form onSubmit={handleSubmit}>
        <label htmlFor="ticker">Enter Stock Ticker:</label>
        <input
          type="text"
          id="ticker"
          value={ticker}
          onChange={(event) => setTicker(event.target.value)}
          required
        />
        <button type="submit" disabled={isLoading}>
          {isLoading ? 'Loading...' : 'Get Stock Info'}
        </button>
      </form>

      {error && <p className="error">{error}</p>}

      {stockData && (
        <div className="stock-info">
          <h2>{stockData.c} ({stockData.o} - {stockData.h} - {stockData.l})</h2>
          <p>
            Company: {stockData.t} ({stockData.n})
          </p>
          <p>Change: {stockData.c - stockData.o} ({((stockData.c - stockData.o) / stockData.o) * 100}%)</p>
        </div>
      )}
    </div>
  );
};

export default StockTicker;
