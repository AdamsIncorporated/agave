import { useState } from 'react';

export const useFetchData = () => {
  const [data, setData] = useState(null);

  const fetchData = async () => {
    try {
      const response = await fetch("/");
      const result = await response;
      setData(result);
    } catch (error) {
      console.error("Error fetching data:", error);
    }
  };

  return { data, fetchData };
};