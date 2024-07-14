use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

use actix_service::{Service, Transform};
use actix_web::dev::{forward_ready, ServiceRequest, ServiceResponse};
use actix_web::Error;
use futures_util::future::{ok, Ready};
use log::info;

pub struct Logging;

impl<S, B> Transform<S, ServiceRequest> for Logging
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LoggingMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggingMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct LoggingMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // リクエストのログを記録
        let method = req.method().clone();
        let path = req.path().to_string();
        let headers = req.headers().clone();
        info!("Incoming request: {} {}", method, path);
        for (key, value) in headers.iter() {
            info!("Header: {}: {:?}", key, value);
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            // レスポンスのログを記録
            let status = res.status();
            let headers = res.headers().clone();
            info!("Response status: {}", status);
            for (key, value) in headers.iter() {
                info!("Header: {}: {:?}", key, value);
            }

            Ok(res)
        })
    }
}
