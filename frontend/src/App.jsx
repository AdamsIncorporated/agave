import React, { useState, useEffect } from "react";
import DataFetcher from "./component/DataFetcher.jsx";
import "./App.css";

function App() {
  const [message, setMessage] = useState();
  const [url, setUrl] = useState(null);
  const handleFetchData = () => {
    setUrl("/api/");
  };

  return (
    <div className="App">
      <header className="App-header">
        <div>
          <button className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" onClick={handleFetchData}>
            Fetch Data
          </button>
          {url && <DataFetcher url={url} />}
        </div>
      </header>
    </div>
  );
}

export default App;
