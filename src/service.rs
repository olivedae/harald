#[derive(Clone)]
pub struct Service;

impl PartialEq for Service {
    fn eq(&self, _other: &Service) -> bool {
        true
    }
}
