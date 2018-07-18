import React from 'react';
import { DOMContext, ReactContext } from './context';


export class Builder {
    constructor(name, contextClass = ReactContext) {
        this.contextClass = contextClass;
        this.context = new this.contextClass(name);
        this.stack = [];
    }

    newContext (name) {
        this.stack.push(this.context);
        this.context = new this.contextClass(name);
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

    setNS(v) {
        this.context.setNS(v);
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
        if (this.props.method === 'dom') {
          this.domRef = React.createRef();
        }
    }

    _prepareModuleState() {
        this.moduleState.props_clear();
        for (let k in this.props) {
            const v = this.props[k];
            if (typeof v == 'string') {
                this.moduleState.props_set_string(k, v);
            } else if (typeof v == 'number') {
                this.moduleState.props_set_number(k, v);
            }
        }

    }

    _getReactTree = () => {
        const { module } = this.props;

        this._prepareModuleState();

        const builder = new Builder("dummy", ReactContext);
        return module.render(this.moduleState, builder);
    }

    _getDOMTree = () => {
        const { module } = this.props;

        this._prepareModuleState();

        const builder = new Builder("dummy", DOMContext);
        return module.render(this.moduleState, builder);
    }

    _checkDomRender = () => {
        if (this.props.method === 'dom') {
            const ref = this.domRef.current;
            if (ref) {
                while (ref.firstChild) {
                    ref.removeChild(ref.firstChild);
                }
                ref.appendChild(this._getDOMTree());
            }
        }
    }

    componentDidMount() {
        this._checkDomRender();
    }

    componentDidUpdate() {
        this._checkDomRender();
    }

    render() {
        if (this.props.method === 'dom') {
            return <div ref={ this.domRef } />;
        } else {
            return this._getReactTree();
        }
    }
}
