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
