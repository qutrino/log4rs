use file::raw::Encoder;

pub struct ConsoleAppenderConfig {
    pub encoder: Option<Encoder>,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_ConsoleAppenderConfig: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for ConsoleAppenderConfig {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<ConsoleAppenderConfig, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        _ => {
                                            Err(_serde::de::Error::invalid_value("expected a field"))
                                        }
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "encoder" => { Ok(__Field::__field0) }
                                        _ =>
                                        Err(_serde::de::Error::unknown_field(value)),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"encoder" => {
                                            Ok(__Field::__field0)
                                        }
                                        _ => {
                                            let value =
                                                ::std::string::String::from_utf8_lossy(value);
                                            Err(_serde::de::Error::unknown_field(&value))
                                        }
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: 
                          _serde::de::Deserializer> _serde::de::Visitor for
                     __Visitor<__D> {
                        type
                        Value
                        =
                        ConsoleAppenderConfig;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<ConsoleAppenderConfig,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: <
                                               Option<Encoder> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            return Err(_serde::de::Error::end_of_stream());
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(ConsoleAppenderConfig{encoder: __field0,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<ConsoleAppenderConfig,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<Option<Encoder>> =
                                    None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("encoder"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<Encoder> > (
                                                          )));
                                        }
                                    }
                                }
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "encoder" )),
                                    };
                                try!(visitor . end (  ));
                                Ok(ConsoleAppenderConfig{encoder: __field0,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["encoder"];
                    deserializer.deserialize_struct("ConsoleAppenderConfig",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
