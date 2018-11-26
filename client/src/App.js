import React, { Component } from 'react';
import './App.css';

class App extends Component {
  constructor(props) {
    super(props);

    this.state = {
      message: "Placeholder Text"
    };
  }

  async componentWillMount() {
    const message = await fetch("/api").then(x => x.json());
    console.log(message);
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
          {`A message from the API: ${message}`}
        </p>
      </div>
    );
  }
}

export default App;
