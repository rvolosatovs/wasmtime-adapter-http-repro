wit_bindgen::generate!({
    exports: {
        world: Component,
        "wasi:http/incoming-handler": Component,
    },
});

use wasi::http::types;

struct Component;

impl exports::wasi::http::incoming_handler::Guest for Component {
    fn handle(request: types::IncomingRequest, response_out: types::ResponseOutparam) {}
}
