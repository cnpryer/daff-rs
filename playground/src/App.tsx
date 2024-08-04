import init, { Text } from "./pkg/daff_wasm";

import { useEffect, useState } from "react";
import { TextareaAutosize } from "@mui/base";

import "./App.css";

function App() {
  const [a, setA] = useState("a,b,c\n1,2,3");
  const [b, setB] = useState("a,b,c\n1,2,4");
  const [changes, setChanges] = useState("");

  useEffect(() => {
    async function initAsync() {
      await init(); // Initialize WASM module

      const textA = new Text(a);
      const textB = new Text(b);

      setChanges(textA.compare(textB).toString());
    }

    initAsync().catch(console.error);
  }, [a, b]);

  return (
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
      <div className="w-full h-full p-2 border border-gray-300">{changes}</div>
    </div>
  );
}

export default App;
