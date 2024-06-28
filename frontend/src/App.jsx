import React from "react";
import { useFetchData } from './component/api';
import "./App.css";

const App = () => {
  const { data, fetchData } = useFetchData();

  return (
    <div className="App">
      <header className="App-header">
        <div>
          <button onClick={fetchData}>Fetch Data</button>
          {data && <pre>{JSON.stringify(data, null, 2)}</pre>}
        </div>
      </header>
    </div>
  );
}

export default App;
