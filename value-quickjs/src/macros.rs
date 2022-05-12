#[macro_export]
macro_rules! js_service {
    ($service: ident) => {
        impl rquickjs::ClassDef for $service {
            const CLASS_NAME: &'static str = stringify!($service);

            unsafe fn class_id() -> &'static mut rquickjs::ClassId {
                static mut CLASS_ID: rquickjs::ClassId = rquickjs::ClassId::new();
                &mut CLASS_ID
            }

            const HAS_PROTO: bool = true;

            // fn init_static<'js>(_ctx: Ctx<'js>, _static: &Object<'js>) -> Result<()> {
            //     println!("init static");
            // }

            fn init_proto<'js>(
                _ctx: rquickjs::Ctx<'js>,
                proto: &rquickjs::Object<'js>,
            ) -> rquickjs::Result<()> {
                use $crate::invoke::Service;
                proto.set(
                    "description",
                    rquickjs::Func::from(rquickjs::Method(
                        |this: &$service, ctx: rquickjs::Ctx<'js>| {
                            //
                            // let o = JsArray::new(ctx)?;
                            let mut out = Vec::default();
                            for i in this.interface() {
                                let mut ret = $crate::value::Map::default();

                                ret.insert("name", i.name.clone());
                                let mut params = Vec::default();
                                for p in i.parameters.params.iter() {
                                    // params.push(p.t)
                                }

                                ret.insert("params", $crate::value::Value::List(params));

                                out.push($crate::value::Value::Map(ret));
                            }

                            $crate::convert::into_js(ctx, $crate::value::Value::List(out))
                        },
                    )),
                )?;
                proto.set(
                    "invoke",
                    rquickjs::Func::from(rquickjs::Async(rquickjs::Method(
                        |this: &$service, name: String, value: rquickjs::Array<'js>| {
                            //
                            let args = $crate::convert::into_args(value);
                            let this = this.clone();

                            async move {
                                let args = args?;
                                let ret = this.call(&name, args).await.unwrap();
                                rquickjs::Result::<_>::Ok($crate::Response::new(ret))
                            }
                        },
                    ))),
                )?;

                Ok(())
            }

            fn into_js_obj<'js>(
                mut self,
                ctx: rquickjs::Ctx<'js>,
            ) -> rquickjs::Result<rquickjs::Value<'js>>
            where
                Self: Sized,
            {
                // self = self.init(ctx);
                rquickjs::Class::<$service>::instance(ctx, self).map(|val| val.into_value())
            }

            const HAS_STATIC: bool = false;

            const HAS_REFS: bool = false;
        }

        impl<'js> rquickjs::IntoJs<'js> for $service {
            fn into_js(self, ctx: rquickjs::Ctx<'js>) -> rquickjs::Result<rquickjs::Value<'js>> {
                use rquickjs::ClassDef;
                self.into_js_obj(ctx)
            }
        }

        impl<'js> rquickjs::FromJs<'js> for &'js $service {
            fn from_js(
                ctx: rquickjs::Ctx<'js>,
                value: rquickjs::Value<'js>,
            ) -> rquickjs::Result<Self> {
                use rquickjs::ClassDef;
                $service::from_js_ref(ctx, value)
            }
        }

        impl<'js> rquickjs::FromJs<'js> for &'js mut $service {
            fn from_js(
                ctx: rquickjs::Ctx<'js>,
                value: rquickjs::Value<'js>,
            ) -> rquickjs::Result<Self> {
                use rquickjs::ClassDef;
                $service::from_js_mut(ctx, value)
            }
        }
    };
}
