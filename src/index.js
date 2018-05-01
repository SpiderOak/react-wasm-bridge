import React from 'react';
import ReactDOM from 'react-dom';
import ReactWasmBridge from './ReactWasmBridge';

let helloModule;

class App extends React.PureComponent {
  state = {
    number: 3,
    message: 'Hello',
  }
    
  _updateNumberRef = node => this.numberNode = node;
  _updateMessageRef = node => this.messageNode = node;

  update = () => {
    this.setState({
      number: parseInt(this.numberNode.value),
      message: this.messageNode.value,
    });
  }

  render() {
    const { number, message } = this.state;

    return (
      <div>
        <p>Number: <input type="number" defaultValue={ number } ref={ this._updateNumberRef } onChange={ this.update } /></p>
        <p>Message: <textarea defaultValue={ message } ref={ this._updateMessageRef } onChange={ this.update } /></p>
        <div>Output: <ReactWasmBridge module={ helloModule } x={ number } message={ message } /></div>
      </div>
    );
  }
}

window.addEventListener('load', () => {
  const hello = import('./hello/hello');
  hello.then(m => {
    window.helloModule = helloModule = m;
    const content = document.getElementById('content');
    ReactDOM.render(<App />, content);
  }).catch(err => console.error(err));
});
