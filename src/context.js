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

    setNS(v) {
        this.namespace = v;
    }

    finish() {
        console.error('Cannot use base Context');
    }
}

export class ReactContext extends Context {
    finish() {
        if ( this.children.length > 0 )
            return React.createElement(this.name, this.attrs, this.children);
        else
            return React.createElement(this.name, this.attrs);
    }
}

export class DOMContext extends Context {
    addText ( text ) {
        this.children.push(document.createTextNode(text));
    }

    finish() {
        const elem = this.namespace ? 
            document.createElementNS(this.namespace, this.name) :
            document.createElement(this.name);
        for (let a in this.attrs) {
            elem.setAttribute(a, this.attrs[a]);
        }
        for (let c in this.children) {
            elem.appendChild(this.children[c]);
        }

        return elem;
    }
}
