import React from 'react';
import ReactDOM from 'react-dom';
import ReactWasmBridge from 'react-wasm-bridge';

let domModule;

class App extends React.PureComponent {
  state = {
    number: 3,
  }
    
  _updateNumberRef = node => this.numberNode = node;
  _updateMessageRef = node => this.messageNode = node;

  update = () => {
    this.setState({
      number: parseInt(this.numberNode.value),
    });
  }

  render() {
    const { number } = this.state;

    return (
      <div>
        <p>Number: <input type="number" defaultValue={ number } ref={ this._updateNumberRef } onChange={ this.update } /></p>
        <p>Output:</p>
        <ReactWasmBridge module={ domModule } slices={ number } method="dom" />
      </div>
    );
  }
}

window.addEventListener('load', () => {
  const dom = import('./dom');
  dom.then(m => {
    window.domModule = domModule = m;
    const content = document.getElementById('content');
    ReactDOM.render(<App />, content);
  }).catch(err => console.error(err));
});
