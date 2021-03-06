
#[macro_export]
macro_rules! router {
    ($($method:ident $path:expr => |$req:ident, $res:ident| $b:block)+) => (
        {
            use nickel::{HttpRouter, MiddlewareResult};
            use nickel::ResponseFinalizer;
            let mut router = nickel::Router::new();

            #[inline(always)]
            fn restrict<R: ResponseFinalizer>(r: R, res: &mut nickel::Response) -> MiddlewareResult {
                r.respond(res)
            }

            $(
                {
                    #[allow(unused_variables)]
                    fn f($req: &mut nickel::Request, $res: &mut nickel::Response) -> MiddlewareResult {
                        restrict($b, $res)
                    }

                    // issue #20178
                    let fhandler: fn(&mut nickel::Request, &mut nickel::Response) -> MiddlewareResult = f;

                    router.$method($path, fhandler);
                }
            )+

            router
        }
    )
}
