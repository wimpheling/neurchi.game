import React, { useState } from "react";
import logo from "./logo.svg";
import "./App.css";
import { Game } from "./engine/build";

function App({ game }: { game: Game }) {
  const updateLog = () => {
    setLog(JSON.stringify(game.get_positions()));
  };
  const addPosition = () => {
    game.add_position(x, y);
    setX(0);
    setY(0);
    updateLog();
  };
  const [x, setX] = useState(0);
  const [y, setY] = useState(0);
  const [log, setLog] = useState("");

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <div>
          <h2>Add position</h2>X :{" "}
          <input
            type="text"
            value={x}
            onChange={(e) => setX(parseInt(e.target.value))}
          />
          <br />Y :{" "}
          <input
            type="text"
            value={y}
            onChange={(e) => setY(parseInt(e.target.value))}
          />
          <br />
          <button onClick={addPosition}>Add position</button>
        </div>
        <div>
          <div>{log}</div>
        </div>
      </header>
    </div>
  );
}

export default App;
