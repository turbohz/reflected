use crate::{FieldRef, Reflected};

pub trait HasVariantsField: Reflected {
    fn variants_for(field: FieldRef<Self>) -> Option<Vec<String>>;
}

impl<T: Reflected> HasVariantsField for T {
    default fn variants_for(_: FieldRef<Self>) -> Option<Vec<String>> {
        None
    }
}
