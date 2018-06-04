import React from 'react';
import ReactDOM from 'react-dom';
import ReactWasmBridge from 'react-wasm-bridge';

let helloModule;

class App extends React.PureComponent {
  state = {
    message: 'Hello',
  }
    
  _updateMessageRef = node => this.messageNode = node;

  update = () => {
    this.setState({
      message: this.messageNode.value,
    });
  }

  render() {
    const { message } = this.state;

    return (
      <div>
        <p>Message: <textarea defaultValue={ message } ref={ this._updateMessageRef } onChange={ this.update } /></p>
        <div>Output: <ReactWasmBridge module={ helloModule } message={ message } /></div>
      </div>
    );
  }
}

window.addEventListener('load', () => {
  const hello = import('./md');
  hello.then(m => {
    window.helloModule = helloModule = m;
    const content = document.getElementById('content');
    ReactDOM.render(<App />, content);
  }).catch(err => console.error(err));
});
