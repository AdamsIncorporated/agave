import React, { useState } from "react";

const DataDisplay = () => {
  const [data, setData] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  const fetchData = () => {
    setLoading(true);
    setError(null);
    fetch("/api")
      .then((response) => {
        if (!response.ok) {
          throw new Error("Network response was not ok");
        }
        return response.json();
      })
      .then((data) => {
        setData(data);
        setLoading(false);
      })
      .catch((error) => {
        setError(error);
        setLoading(false);
      });
  };

  return (
    <div>
      <h1>Data from Rust Endpoint</h1>
      <button
        className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
        onClick={fetchData}
      >
        Fetch Data
      </button>
      {loading && <div>Loading...</div>}
      {error && <div>Error: {error.message}</div>}
      <table>
        <thead>
          <tr>
          </tr>
        </thead>
      <tbody>
        <tr>

        </tr>
      </tbody>
      </table>
    </div>
  );
};

export default DataDisplay;
