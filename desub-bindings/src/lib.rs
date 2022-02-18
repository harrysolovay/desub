// use desub_current::Metadata;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub fn seed() -> String {
	// let x = Metadata::from_bytes(bytes).expect("");
	// println!("{:?}", x);
	"HELLO".to_string()
}
