import init, { Text, Csv } from "./pkg/daff_wasm";

import { useEffect, useState } from "react";
import { Button, TextareaAutosize } from "@mui/base";

import "./App.css";

function App() {
  const [a, setA] = useState("a,b,c\n1,2,3");
  const [b, setB] = useState("a,b,c\n1,2,4");
  const [diffType, setDiffType] = useState("text");
  const [changes, setChanges] = useState("");

  useEffect(() => {
    async function initAsync() {
      await init(); // Initialize WASM module

      handleChanges();
    }

    initAsync().catch(console.error);
  }, [a, b, diffType]);

  const handleChanges = () => {
    if (diffType === "text") {
      const aObj = new Text(a);
      const bObj = new Text(b);
      setChanges(aObj.compare(bObj).toString());
    } else if (diffType === "csv") {
      const aObj = new Csv(a);
      const bObj = new Csv(b);
      setChanges(aObj.compare(bObj).toString());
    }
  };

  return (
    <>
      <select
        value={diffType}
        onChange={(e) => setDiffType(e.target.value)}
        className="block appearance-none border border-gray-400 hover:border-gray-500 px-4 py-2 pr-8 rounded shadow leading-tight focus:outline-none focus:shadow-outline"
      >
        <option value="text">Text</option>
        <option value="csv">CSV</option>
      </select>
      <div className="grid grid-cols-3 gap-4 min-h-screen">
        <div className="w-full h-full">
          <TextareaAutosize
            className="resize-none w-full min-h-full p-2 border border-gray-300"
            value={a}
            onChange={(e) => setA(e.target.value)}
          />
        </div>
        <div className="w-full h-full">
          <TextareaAutosize
            className="resize-none w-full min-h-full p-2 border border-gray-300"
            value={b}
            onChange={(e) => setB(e.target.value)}
          />
        </div>
        <div className="w-full h-full p-2 border border-gray-300">
          {changes}
        </div>
      </div>
      );
    </>
  );
}

export default App;
