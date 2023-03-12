// We use a macro to create the app.
// This is because the type of the App instance is too complex to return.
// This allows us to share the app between the main function and the tests.
#[macro_export]
macro_rules! app (
    ($pool: expr) => ({
      App::new()
      .app_data(web::Data::new($pool.clone()))
      .wrap(middleware::NormalizePath::new(TrailingSlash::Always))
      .wrap_api()
      .wrap(Logger::default())
      .configure(entry::service::configure)
      .with_json_spec_at("/openapi.json/")
      .build()
    });
);
