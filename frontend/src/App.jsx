import React from "react";
import DataDisplay from "./component/api";
import "./App.css";

const App = () => {
  return (
    <div className="App">
      <header className="App-header">
        <div>
          <DataDisplay />
        </div>
      </header>
    </div>
  );
};

export default App;
