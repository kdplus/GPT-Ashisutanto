/*global chrome*/
import logo from './logo.svg';
import './App.css';

function App() {
  const saveApiKey = () => {
    const apiKey = document.getElementById('apiKey').value;
    chrome.storage.local.set({ apiKey: apiKey }, function() {
      console.log('API key saved');
      console.log(apiKey);
    });
  };

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <input type="text" id="apiKey" placeholder="Enter API key" />
        <button onClick={saveApiKey}>Save</button>
      </header>
    </div>
  );
}


export default App;
