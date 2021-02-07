use crate::Result;
use crate::FileStream;

use hyper::{Body, Request, Response, Server, Uri};
use hyper::service::{make_service_fn, service_fn};
use std::net::IpAddr;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{AsyncSeekExt, SeekFrom};

#[derive(Debug)]
pub struct ServeStatic {
    scheme: String,
    tls: bool,
    addr: IpAddr,
    port: u16,
    dir: PathBuf,
}

impl ServeStatic {
    pub fn new_with_dir(bind: String, dir: PathBuf) -> Self {
        let bind: Uri = bind.parse().unwrap();
        let scheme = bind.scheme_str().unwrap_or("http");
        let tls = scheme == "https";
        assert!(!tls);  // not implemented (yet)
        assert!(dir.as_path().is_dir());
        Self {
            scheme: scheme.into(),
            tls,
            addr: bind.host().unwrap().parse().unwrap(),
            port: bind.port_u16().unwrap_or_else(|| if tls {443} else {80}),
            dir,
        }
    }

    pub fn new_with_tmp(bind: String) -> Self {
        Self::new_with_dir(bind, std::env::temp_dir())
    }

    pub fn new(bind: String) -> Self {
        Self::new_with_dir(bind, std::env::current_dir().unwrap())
    }

    async fn serve_static_file(req: Request<Body>, dir: PathBuf) -> Result<Response<Body>> {
        let path = dir.join(req.uri().path().strip_prefix('/').unwrap());
        log::debug!("path = {:?}", path);

        let mut file = File::open(path).await?;
        file.seek(SeekFrom::Start(0)).await?;

        let stream = FileStream::new(file);
        Ok(Response::new(Body::wrap_stream(stream)))
    }

    pub async fn serve(self: &Self) -> Result<()> {
        let addr = (self.addr, self.port).into();

        let make_svc = make_service_fn(move |_| {
            let dir = self.dir.clone();
            async move {
                Ok::<_, hyper::Error>(service_fn(move |req| {
                    Self::serve_static_file(req, dir.clone())
                }))
            }
        });

        let server = Server::bind(&addr).serve(make_svc);

        log::info!("Listening on {}://{}:{}", self.scheme, self.addr, self.port);
        log::info!("Serving from {}/", self.dir.to_str().unwrap());

        Ok(server.await?)
    }
}
