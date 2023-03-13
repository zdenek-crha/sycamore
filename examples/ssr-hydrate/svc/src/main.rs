use std::path::PathBuf;
use std::sync::Arc;

use tower_http::services::ServeDir;

use axum::extract::Extension;
use axum::routing::get;
use axum::response::Html;
use axum::response::IntoResponse;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let appdir   = PathBuf::from("../app/dist");
    let template = Template::from_path(&appdir)?;
    let socket   = "127.0.0.1:3000".parse::<std::net::SocketAddr>()?;

    let app = axum::Router::new();
    let app = app.route("/", get(index));

    // Endpoint service will serve all files from app/dist directory to the browser. A care must be
    // taken to ensure prefix used by endpoint is the same as prefix set in Trunk.toml.
    // Otherwise the generated index.html we use as template will point browser to paths our server
    // can't provide
    let app = app.nest_service("/app", ServeDir::new(appdir));

    // extension providing HTML template to endpoints to use for wrapping SSR rendering of Sycamore
    // components
    let app = app.layer(Extension(Arc::new(template)));

    axum::Server::bind(&socket)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

/// Index endpoint handler that renders components via SSR and wraps them in index.html template
pub async fn index(Extension(template): Extension<Arc<Template>>) -> impl IntoResponse {
    let component = sycamore::render_to_string(app::App);

    Html(template.render(component))
}

/// Template of an index HTML response
///
/// The backend service need to generate and return valid HTML from endpoints that serve SSR
/// rendered Sycamore components. The HTML must contain links to app JavaScript, WASM package,
/// stylesheets... And some of those may be parametrized by build hash to force browser to
/// re-request when changed.
///
/// That is a lot of things and making custom HTML would be non-trivial, especially working with
/// build hashes in asset filenames. It would also duplicate work that Trunk has done building WASM
/// module.
///
/// We won't be using Trunk generated index.html directly, but we can re-use it as template. We
/// read it from WASM dist directory and split it on `<body>` string. Then we can 'sandwich' any
/// components rendered by SSR into this 'template'. 
///
/// To pass the [`Template`] into relevant endpoint handlers, we will use
/// [`axum::extract::Extension`]
///
/// sfor that would be duplicating work
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Template {
    prev: String,
    post: String,
}

impl Template {

    pub fn from_path(path: &PathBuf) -> anyhow::Result<Self> {
        let mut path = path.to_owned();
        path.push("index.html");

        let text = std::fs::read_to_string(&path)?;
        let (prev, post) = text
            .split_once("<body>")
            .ok_or_else(|| anyhow::Error::msg("<body> tag not found in html"))?;

        Ok(Self {
            prev: String::from(prev),
            post: String::from(post),
        })
    }

    pub fn render(&self, body: String)-> String {
        format!("{}<body>{}</body>{}", self.prev, body, self.post)
    }
}
