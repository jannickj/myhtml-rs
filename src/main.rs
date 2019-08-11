mod bindings;
use bindings as mh;
use std::ffi;

fn main() {
    unsafe {
        let html = "<div><span>HTML</span></div>";
        let myhtml = mh::myhtml_create();
        let default_opt = mh::myhtml_options_MyHTML_OPTIONS_DEFAULT;
        mh::myhtml_init(myhtml, default_opt, 1, 0);
        let tree = mh::myhtml_tree_create();
        mh::myhtml_tree_init(tree, myhtml);

        let utf8env = mh::myencoding_list_MyENCODING_UTF_8;
        let html_cstr = ffi::CString::new(html).expect("can convert");
        mh::myhtml_parse(tree, utf8env, html_cstr.as_ptr(), html.len());

        let mut mhstring = std::mem::zeroed::<mh::mycore_string_raw_t>();
        mh::myhtml_serialization_tree_buffer(mh::myhtml_tree_get_document(tree), &mut mhstring);
        println!("{:#?}", mhstring);
        println!("{:#?}", ffi::CStr::from_ptr(mhstring.data).to_str());
    }
}
