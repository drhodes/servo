/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::HTMLQuoteElementBinding;
use dom::bindings::utils::{DOMString, ErrorResult};
use dom::document::AbstractDocument;
use dom::element::HTMLQuoteElementTypeId;
use dom::htmlelement::HTMLElement;
use dom::node::{AbstractNode, Node, ScriptView};

pub struct HTMLQuoteElement {
    htmlelement: HTMLElement,
}

impl HTMLQuoteElement {
    pub fn new_inherited(localName: ~str, document: AbstractDocument) -> HTMLQuoteElement {
        HTMLQuoteElement {
            htmlelement: HTMLElement::new(HTMLQuoteElementTypeId, localName, document)
        }
    }

    pub fn new(localName: ~str, document: AbstractDocument) -> AbstractNode<ScriptView> {
        let element = HTMLQuoteElement::new_inherited(localName, document);
        Node::reflect_node(@mut element, document, HTMLQuoteElementBinding::Wrap)
    }
}

impl HTMLQuoteElement {
    pub fn Cite(&self) -> DOMString {
        None
    }

    pub fn SetCite(&self, _cite: &DOMString) -> ErrorResult {
        Ok(())
    }
}
