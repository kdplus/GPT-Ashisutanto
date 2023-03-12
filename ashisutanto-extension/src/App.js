/*global chrome*/
import logo from './logo.svg';
import './App.css';
// import ApiCalendar from "react-google-calendar-api";
// import { useEffect } from 'react';

function App() {

  // useEffect(() => {
  //   chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  //     if (message.type === 'createEvent') {
  //       const event = message.data;
  //       // console.log("Event from app", event);

  //       window.gapi.load('auth2', () => {
  //         const client_id = '';
  //         const auth2 = window.gapi.auth2.init({ client_id })
  //         auth2.signIn()
  //           .then(console.log)
  //           .then(() => {
  //             // Call the Google Calendar API to create the event
  //             window.gapi.client.calendar.events.insert({
  //               calendarId: 'primary',
  //               resource: event,
  //             }).then((response) => {
  //               console.log('Event created: ' + response.htmlLink);
  //             }).catch((error) => {
  //               console.error('Error creating event:', error);
  //             });
  //           });
  //       })
  //     }
  //   });
  //   // console.log("hook on");
  //   return () => {
  //     chrome.runtime.onMessage.removeListener();
  //   };
  // }, []);

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}


export default App;
