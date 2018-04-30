import React from 'react';

class Builder {
    constructor(name) {
	this.name  = name;
	this.attrs = {};
	this.children = [];
    }

    factory (name) {
	return new Builder(name);
    }

    addChild ( child ) {
	this.children.push(child);
    }

    addText ( text ) {
	this.children.push(text);
    }

    setAttr( k, v ) {
	this.attrs[k] = v;
    }

    finish ( ) {
	return React.createElement(this.name, this.attrs, this.children);
    }
}

export default class ReactWasmBridge extends React.PureComponent {
  componentWillMount() {
    const { module } = this.props;

    this.moduleState = module.State.new();
  }

  _transformTreeToReact(fragment) {
      console.log(fragment);
      return fragment.hello();

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

      let builder = new Builder("dummy");
      return module.render(this.moduleState, builder);
      /*
    const elem = module.render(this.moduleState);

    return this._transformTreeToReact(elem);
    */
  }

  render() {
    const content = this._getRenderTree();
    return content;
  }
}
