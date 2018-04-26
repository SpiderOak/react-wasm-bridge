import React from 'react';

export default class ReactWasmBridge extends React.PureComponent {
  componentWillMount() {
    const { module } = this.props;

    this.moduleState = module.State.new();
  }

  _transformTreeToReact(node) {
    switch (node.type) {
      case 'element':
        const reactChildren = node.children ?
          // Ugh
          (node.children.length > 1 ?
            node.children.map(n => this._transformTreeToReact(n)) :
            this._transformTreeToReact(node.children[0])
          ) : null;
        return React.createElement(node.name, node.attributes, reactChildren);
      case 'text':
        return node.text;
    }
  }

  _getRenderTree() {
    const { module } = this.props;

    this.moduleState.props_clear();
    for (let k in this.props) {
      const v = this.props[k];
      if (typeof v == 'string') {
        this.moduleState.props_set_string(k, v);
      } else if (typeof v == 'number') {
        this.moduleState.props_set_number(k, v);
      }
    }

    const treeJSON = module.render(this.moduleState);
    const tree = JSON.parse(treeJSON);

    return this._transformTreeToReact(tree);
  }

  render() {
    const content = this._getRenderTree();
    return content;
  }
}
