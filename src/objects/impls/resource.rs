use crate::{constants::ResourceType, macros::*, objects::Resource};

impl Resource {
    pub fn resource_type(&self) -> ResourceType {
        js_unwrap!(__resource_type_str_to_num(@{self.as_ref()}.resourceType))
    }
}

simple_accessors! {
    Resource;
    (amount -> amount -> u32),
}
