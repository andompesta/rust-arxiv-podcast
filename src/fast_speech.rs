use tch;
use std::path::Path;

pub fn load_model() -> Result<tch::CModule, tch::TchError> {
    let path = Path::new("./resources/traced.pt");
    return tch::CModule::load(path);
}