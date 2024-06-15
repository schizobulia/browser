pub fn get_init_js_code() -> &'static str {return "class Element {
    constructor(tag, innerText, _id) {
        this.tag = tag;
        this._id = _id
        this._text = innerText
        const descriptor = Object.getOwnPropertyDescriptor(Element.prototype, 'innerText');
        Object.defineProperty(this, 'innerText', {
            get: () => { return this._text },
            set: (value) => {
                Deno.core.ops.change_element_text(_id.toString(), value);
                this._text = value;
            }
        });
    }
}
globalThis.Element = Element
globalThis.document = {
    getElementById: (id) => {
        let data = Deno.core.ops.get_element_by_id(id);
        if (data) {
            data = JSON.parse(data)
            const res = new globalThis.Element(data.tag_name, '', data.id)
            if (data.text) {
                res._text = data.text.text
            }
            return res
        }
        return undefined
    }
}"; }