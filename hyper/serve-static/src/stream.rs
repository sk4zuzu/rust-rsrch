use futures_util::stream::Stream;
use futures_util::task::{Context, Poll};
use hyper::body::Bytes;
use std::mem::MaybeUninit;
use std::pin::Pin;
use tokio::fs::File;
use tokio::io::{AsyncRead, ReadBuf};

const BUF_SIZE: usize = 8*1024;

#[derive(Debug)]
pub struct FileStream {
    file: File,
    buf: Box<[MaybeUninit<u8>; BUF_SIZE]>,
}

impl FileStream {
    pub fn new(file: File) -> Self {
        Self { file, buf: Box::new([MaybeUninit::uninit(); BUF_SIZE]) }
    }
}

impl Stream for FileStream {
    type Item = Result<Bytes, std::io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let me = &mut *self;
        let mut read_buf = ReadBuf::uninit(&mut me.buf[..]);
        match Pin::new(&mut me.file).poll_read(cx, &mut read_buf) {
            Poll::Ready(Ok(())) => {
                let filled = read_buf.filled();
                if filled.is_empty() {
                    Poll::Ready(None)
                } else {
                    Poll::Ready(Some(Ok(Bytes::copy_from_slice(filled))))
                }
            },
            Poll::Ready(Err(e)) => Poll::Ready(Some(Err(e))),
            Poll::Pending => Poll::Pending,
        }
    }
}
