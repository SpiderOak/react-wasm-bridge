import React from 'react';

class Context {
    constructor(name) {
	this.name  = name;
	this.attrs = {};
	this.children = [];
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

export class Builder {
    constructor(name) {
	this.context = new Context(name);
	this.stack = [];
    }

    newContext (name) {
	this.stack.push(this.context);
	this.context = new Context(name);
    }

    addChild ( child ) {
	this.context.addChild(child);
    }

    addText ( text ) {
	this.context.addText(text);
    }

    setAttr( k, v ) {
	this.context.setAttr(k,v);
    }

    finishContext ( ) {
	if (this.context === undefined ) {
	    console.error("attempt to finish undefined context!");
	    return undefined;
	}
	
	let ret = this.context.finish();

	if (this.stack.length === 0) {
	    //this.context = undefined;
	}

	else {
	    this.context = this.stack.pop();
	    this.context.addChild(ret);
	}

	return ret;
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
