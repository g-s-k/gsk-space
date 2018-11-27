import React, { Component, Fragment } from "react";
import "./App.css";

class App extends Component {
  constructor(props) {
    super(props);

    this.state = {
      message: "",
      stats: [],
      random: ""
    };
  }

  componentWillMount() {
    this.getMessage();
    this.getRequestHash();
    this.connectWebsocket();
  }

  connectWebsocket = () => {
    this.socket = new WebSocket("ws://localhost/api/ws/");
    this.socket.onmessage = this.receiveStats;
    //setInterval(() => {
      //this.socket.send(Math.random());
    //}, 2000)
  };

  receiveStats = ({ data }) => {
    const { stats } = this.state;
    stats.push(JSON.parse(data));
    this.setState({ stats });
  };

  getRequestHash = async () => {
    const random = await fetch("/api/sha256").then(x => x.text());
    this.setState({ random });
  };

  getMessage = async () => {
    const message = await fetch("/api").then(x => x.json());
    this.setState({ message });
  };

  render() {
    const { message, random, stats } = this.state;
    return (
      <div className="App">
        <header className="App-header">
          <h1 className="App-title">Welcome to George's website</h1>
        </header>
        <div className="App-body">
          <div>
            <div className="App-info">
              {message
                ? `A message from the API: ${message}`
                : "No message from the API yet."}
            </div>
            <div className="App-info Hash">
              <Fragment>
                {random ? (
                  <span>
                    A timestamped hash of an API request: <code>{random}</code>
                  </span>
                ) : (
                  "No random response from the API yet."
                )}
                <button onClick={this.getRequestHash}>
                  {random ? "Get new hash" : "Try again"}
                </button>
              </Fragment>
            </div>
          <div className="App-info Stats">
            {stats.map((v, i) => (
              <div key={i}>
                {v.one}
              </div>
            ))}
          </div>
          </div>
          <footer>
            <p className="App-info">
              The source for this site is located{" "}
              <a href="https://github.com/g-s-k/gsk-space">here</a>.
            </p>
          </footer>
        </div>
      </div>
    );
  }
}

export default App;
