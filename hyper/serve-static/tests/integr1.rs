mod util;

#[tokio::test]
async fn test_serve_static() -> serve_static::Result<()> {
    pretty_env_logger::init();

    let (mut file, _) = util::file::random(10*1024)?;

    let port = 1234;

    let server = serve_static::ServeStatic::new_with_tmp(format!("http://127.0.0.1:{}", port));

    let (_, r2) = tokio::join!(
        tokio::time::timeout(
            std::time::Duration::from_secs(4),
            server.serve(),
        ),
        util::http::get(format!("http://127.0.0.1:{}/{}", port, file.path().file_name().unwrap().to_str().unwrap())),
    );

    let resp = r2?;
    assert_eq!(resp.status(), hyper::StatusCode::OK);

    let bytes = hyper::body::to_bytes(resp.into_body()).await?;

    let hash = util::hash::from_bytes(&bytes[..])?;
    assert_eq!(hash, util::hash::from_file(file.as_file_mut())?);

    Ok(())
}
