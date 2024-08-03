import init, { Csv } from "./pkg/daff_wasm";

import { useEffect, useState } from "react";

import "./App.css";

function App() {
  const [a, setA] = useState("a,b,c\n1,1,1");
  const [b, setB] = useState("a,b,c\n2,1,1");
  const [diff, setDiff] = useState("");

  useEffect(() => {
    async function initAsync() {
      await init(); // Initialize WASM module

      const csvA = new Csv(a);
      const csvB = new Csv(b);

      setDiff(csvB.compare(csvA));
    }

    initAsync().catch(console.error);
  }, []);

  return (
    <>
      <div className="card">{a}</div>
      <div className="card">{b}</div>
      <div className="card">{diff}</div>
    </>
  );
}

export default App;
