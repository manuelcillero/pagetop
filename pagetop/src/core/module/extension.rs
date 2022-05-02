use crate::util;

pub trait BaseExtension {
    fn type_name(&self) -> &'static str;

    fn single_name(&self) -> &'static str;

    fn qualified_name(&self, last: u8) -> &'static str;
}

/// Las extensiones deben extender este *trait*.
pub trait ExtensionTrait: BaseExtension + Send + Sync {
    fn weight(&self) -> i8 {
        0
    }
}

impl<E: ?Sized + ExtensionTrait> BaseExtension for E {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn single_name(&self) -> &'static str {
        util::partial_type_name(std::any::type_name::<Self>(), 1)
    }

    fn qualified_name(&self, last: u8) -> &'static str {
        util::partial_type_name(std::any::type_name::<Self>(), last)
    }
}
