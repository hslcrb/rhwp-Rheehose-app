use crate::DocumentCore;
use crate::error::HwpError;
use std::sync::Mutex;

#[derive(uniffi::Object)]
pub struct HwpDocumentNative {
    core: Mutex<DocumentCore>,
}

#[uniffi::export]
impl HwpDocumentNative {
    #[uniffi::constructor]
    pub fn new(data: Vec<u8>) -> Result<Self, HwpError> {
        DocumentCore::from_bytes(&data).map(|core| HwpDocumentNative { core: Mutex::new(core) })
    }

    pub fn page_count(&self) -> u32 {
        self.core.lock().unwrap().page_count()
    }

    pub fn render_page_svg(&self, page_num: u32) -> Result<String, HwpError> {
        self.core.lock().unwrap().render_page_svg_native(page_num)
    }

    pub fn render_page_html(&self, page_num: u32) -> Result<String, HwpError> {
        self.core.lock().unwrap().render_page_html_native(page_num)
    }
}
