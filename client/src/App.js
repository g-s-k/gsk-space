import React, { Component } from "react";
import "./App.css";

class App extends Component {
  constructor(props) {
    super(props);

    this.state = {
      message: "",
      random: ""
    };
  }

  async componentWillMount() {
    const message = await fetch("/api").then(x => x.json());
    const random = await fetch("/api/sha256").then(x => x.text());
    this.setState({ message, random });
  }

  render() {
    const { message, random } = this.state;
    return (
      <div className="App">
        <header className="App-header">
          <h1 className="App-title">Welcome to George's website</h1>
        </header>
      <div className="App-body">
        <div>
          <div>
            {message
              ? `A message from the API: ${message}`
              : "No message from the API yet."}
          </div>
          <div>
            {random ? (
              <span>
                A timestamped hash of an API request: <code>{random}</code>
              </span>
            ) : (
              "No random response from the API yet."
            )}
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
