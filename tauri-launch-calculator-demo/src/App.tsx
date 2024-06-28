// src/App.tsx
import React from 'react';
import './App.css';
import { invoke } from '@tauri-apps/api/tauri';

function App() {
    const launchCalculator = async () => {
        await invoke('launch_calculator');
    };

    return (
        <div className="App">
            <header className="App-header">
                <h1>Launch Calculator</h1>
                <button onClick={launchCalculator}>Launch Calculator</button>
            </header>
        </div>
    );
}

export default App;
