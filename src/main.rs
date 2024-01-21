#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  use actix_files::Files;
  use actix_web::web::Data;
  use leptos_actix::LeptosRoutes;
  use leptos_hydrate_test_1::app::App;

  let conf = leptos::get_configuration(None).await.unwrap();
  let addr = conf.leptos_options.site_addr;
  let routes = leptos_actix::generate_route_list(App);
  actix_web::HttpServer::new(move || {
    let leptos_options = &conf.leptos_options;
    let site_root = &leptos_options.site_root;
    actix_web::App::new()
      .service(favicon)
      // .service(Files::new("/",site_root)  )
      .service(Files::new("/pkg", format!("{site_root}/pkg")))
      .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
      .app_data(Data::new(leptos_options.to_owned()))
  })
  .bind(&addr)?
  .run()
  .await
}
#[cfg(feature = "ssr")]
#[actix_web::get("/favicon.ico")]
async fn favicon(leptos_options: actix_web::web::Data<leptos::LeptosOptions>) -> impl actix_web::Responder {
  let leptos_options = leptos_options.into_inner();
  let site_root = &leptos_options.site_root;
  actix_files::NamedFile::open_async(format!("{site_root}/favicon.ico")).await
}

#[cfg(not(feature = "ssr"))]
fn main() {}

