use std::future::Future;

pub trait Service<Request> {
    type Response;
    type Error;

    /// Process the request and return the response.
    ///
    /// The returned future must be `Send` so the service can be used with a
    /// multi-threaded Tokio runtime.
    fn call(
        &mut self,
        req: Request,
    ) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send;
}
