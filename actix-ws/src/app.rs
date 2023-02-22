use actix_web::{
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    error, middleware,
    web::{self, Data},
    App, Error, HttpResponse,
};
use domains::DataSource;

use crate::cat;

#[derive(Debug, Clone)]
pub struct WebServiceApp {
    data_source: Data<DataSource>,
}

impl WebServiceApp {
    /// Constructor
    pub fn new(data_source: DataSource) -> Self {
        Self {
            // web::Data will wrap our data into an Arc
            data_source: web::Data::new(data_source),
        }
    }

    /// App factory
    pub fn build(
        &self,
    ) -> App<
        impl ServiceFactory<
            ServiceRequest,
            Config = (),
            Response = ServiceResponse,
            Error = Error,
            InitError = (),
        >,
    > {
        App::new()
            .app_data(self.data_source.clone())
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Always,
            ))
            // .app_data(web::JsonConfig::default().error_handler(|err, _req| {
            //     error::InternalError::from_response(err, HttpResponse::Conflict().into()).into()
            // }))
            .service(web::scope("/api").configure(cat::routes::routes))
    }
}
