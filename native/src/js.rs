//! Traits for converting between local values and neon values/handles
use neon::context::Context;
use neon::handle::{Handle};
use neon::types::JsValue;
use neon::result::{JsResult, NeonResult};

pub trait ToJs {
    fn to_js<'a, CX: Context<'a>>(self, cx: &mut CX) -> JsResult<'a, JsValue>;
}

pub trait FromJs: Sized {
    fn from_js<'a, CX: Context<'a>>(handle: Handle<'a, JsValue>, cx: &mut CX) -> NeonResult<Self>;
}

pub trait ToJsMulti {
    fn to_js_multi<'a, CX: Context<'a>>(self, cx: &mut CX) -> Vec<Handle<'a, JsValue>>;
}