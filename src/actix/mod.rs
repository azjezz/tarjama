use crate::{locale::Locale, Translator};
use actix_utils::future::{ready, Ready};
use actix_web::{
    dev::{
        forward_ready, Payload, Service, ServiceRequest, ServiceResponse,
        Transform,
    },
    Error, FromRequest, HttpMessage, HttpRequest,
};
use std::{future::Future, pin::Pin};

/// Middleware to inject the translator and locale into the request extensions.
///
/// The middleware will look for the `Accept-Language` header in the request and
/// set the locale to the first language that is supported by the translator.
///
/// If no supported language is found, the default locale will be used.
///
/// The translator and locale will be available in the request extensions as
/// `Translator` and `Locale` respectively.
///
/// Note: This middleware requires the `actix-web` feature to be enabled.
///
/// # Example
///
/// ```rust
/// # async fn example() -> std::io::Result<()> {
/// use actix_web::{error, middleware, web};
/// use actix_web::{App, HttpResponse, HttpServer, Result};
/// use tarjama::loader::toml::load;
/// use tarjama::locale::Locale;
/// use tarjama::locale::EnglishVariant;
/// use tarjama::Translator;
/// use tarjama::context;
/// use tarjama::actix::TranslatorMiddleware;
///
/// let translator = Translator::with_catalogue_bag(
///   load("/path/to/translations/").await.expect("couldn't load translations"),
/// );
///
/// async fn example(translator: Translator, locale: Locale) -> Result<HttpResponse> {
///     let content = translator.trans(locale, "messages", "greeting", context!(name = "World"))
///         .map_err(error::ErrorInternalServerError)?;
///
///     Ok(HttpResponse::Ok().content_type("text/plain").body(content))
/// }
///
/// HttpServer::new(move || {
///   App::new()
///     .wrap(TranslatorMiddleware::new(translator.clone(), Locale::English(EnglishVariant::Default)))
///     .route("/", web::get().to(example))
/// })
/// .bind(("127.0.0.1", 8080))?
/// .run()
/// .await
/// # }
/// ```
#[derive(Clone)]
pub struct TranslatorMiddleware {
    translator: Translator,
    default_locale: Locale,
}

impl TranslatorMiddleware {
    pub fn new(translator: Translator, default_locale: Locale) -> Self {
        TranslatorMiddleware { translator, default_locale }
    }
}

impl<S, B> Transform<S, ServiceRequest> for TranslatorMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>
        + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = TranslatorMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let mut translator = self.translator.clone();
        let default_locale = self.default_locale.clone();
        translator.set_fallback_locale(default_locale);

        ready(Ok(TranslatorMiddlewareService {
            service,
            translator,
            default_locale,
        }))
    }
}

#[derive(Clone)]
pub struct TranslatorMiddlewareService<S> {
    service: S,
    translator: Translator,
    default_locale: Locale,
}

impl<S, B> Service<ServiceRequest> for TranslatorMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>
        + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        req.extensions_mut().insert(self.translator.clone());

        let mut locale = self.default_locale;
        if let Some(header_value) = req.headers().get("Accept-Language") {
            if let Ok(header) = header_value.to_str() {
                let langauges = accept_language::parse(header);
                for language in langauges {
                    if let Ok(accept_locale) = Locale::try_from(language) {
                        locale = accept_locale;
                        break;
                    }
                }
            }
        }

        req.extensions_mut().insert(locale);

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

impl FromRequest for Translator {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &HttpRequest,
        _payload: &mut Payload,
    ) -> Self::Future {
        if let Some(translator) = req.extensions().get::<Translator>() {
            ready(Ok(translator.clone()))
        } else {
            ready(Err(actix_web::error::ErrorInternalServerError(
                "Translator not found",
            )))
        }
    }
}

impl FromRequest for Locale {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &HttpRequest,
        _payload: &mut Payload,
    ) -> Self::Future {
        if let Some(locale) = req.extensions().get::<Locale>() {
            ready(Ok(locale.clone()))
        } else {
            ready(Err(actix_web::error::ErrorInternalServerError(
                "Locale not found",
            )))
        }
    }
}
