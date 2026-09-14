fn eof() -> io::Error {
    io::Error::new(io::ErrorKind::UnexpectedEof, "early eof")
}

impl<A> Future for ReadExact<'_, A>
where
    A: AsyncRead + Unpin + ?Sized,
{
    type Output = io::Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<usize>> {
        let me = self.project();

        loop {
            // if our buffer is empty, then we need to read some data to continue.
            let rem = me.buf.remaining();
            if rem != 0 {
                match ready!(Pin::new(&mut *me.reader).poll_read(cx, me.buf)) {
                    Ok(()) => {}
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    Err(e) => return Err(e).into(),
                }
                if me.buf.remaining() == rem {
                    return Err(eof()).into();
                }
            } else {
                return Poll::Ready(Ok(me.buf.capacity()));
            }
        }
    }
}
