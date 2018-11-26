import React, { Component } from 'react';
import './App.css';

class App extends Component {
  constructor(props) {
    super(props);

    this.state = {
      message: ""
    };
  }

  async componentWillMount() {
    const message = await fetch("/api").then(x => x.json());
    this.setState({ message });
  }

  render() {
    const { message } = this.state;
    return (
      <div className="App">
        <header className="App-header">
          <h1 className="App-title">Welcome to George's website</h1>
        </header>
        <p className="App-intro">
          {message ? `A message from the API: ${message}` : "No message from the API yet."}
        </p>
      <footer>
        <p className="App-info">The source for this site is located <a href="https://github.com/g-s-k/gsk-space">here</a>.</p>
      </footer>
      </div>
    );
  }
}

export default App;
