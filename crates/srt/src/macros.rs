macro_rules! auto_try_from {
    (#[repr($vtype:ident)] $(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        #[repr($vtype)]
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<$vtype> for $name {
            type Error = anyhow::Error;

            fn try_from(v: $vtype) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as $vtype => Ok($name::$vname),)*
                    _ => Err(anyhow::anyhow!("Unknown value: 0x{v:x}")),
                }
            }
        }
    }
}
pub(crate) use auto_try_from;

/// Add `to_raw` implementation
///
/// Converts all fields sequentially to `Vec<u8>` (Big-Endian)
macro_rules! simple_raw {
    (
        $(#[$meta:meta])* $vis:vis struct $name:ident {
            $($(#[$vmeta:meta])* $vvis:vis $vname:ident: $vtype:ident,)*
        }
    ) => {
        $(#[$meta])*
        $vis struct $name {
            $($(#[$vmeta])* $vvis $vname: $vtype,)*
        }

        impl $name {
            pub fn to_raw(&self) -> Vec<u8> {
                let mut res = Vec::new();
                $(res.extend(self.$vname.to_be_bytes());)*
                res
            }
        }
    }
}
pub(crate) use simple_raw;
