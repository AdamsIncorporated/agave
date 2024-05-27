import React, { useState, useEffect } from 'react';
import DataFetcher from './component/DataFetcher.jsx';
import logo from './logo.svg';
import './App.css';

function App() {
  const [message, setMessage] = useState();
  const [url, setUrl] = useState(null);
  const handleFetchData = () => {
    setUrl('/api/yahoo-send-to-consent/');
  };

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>{message || 'Loading...'}</p>
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
        <div>
          <button onClick={handleFetchData}>Fetch Data</button>
          {url && <DataFetcher url={url} />}
        </div>
      </header>
    </div>
  );
}

export default App;
