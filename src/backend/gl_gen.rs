#[allow(clippy::all)]
#[allow(unsafe_op_in_unsafe_fn)]
pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
