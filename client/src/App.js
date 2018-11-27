import React, { Component, Fragment } from "react";
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
    this.setState({ message });
    this.getRequestHash();
  }

  getRequestHash = async () => {
    const random = await fetch("/api/sha256").then(x => x.text());
    this.setState({ random });
  };

  render() {
    const { message, random } = this.state;
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
                <button role="button" onClick={this.getRequestHash}>
                  {random ? "Get new hash" : "Try again"}
                </button>
              </Fragment>
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
