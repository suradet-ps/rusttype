impl<W> Future for WriteAll<'_, W>
where
    W: AsyncWrite + Unpin + ?Sized,
{
    type Output = io::Result<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let me = self.project();
        while !me.buf.is_empty() {
            let n = loop {
                match ready!(Pin::new(&mut *me.writer).poll_write(cx, me.buf)) {
                    Ok(n) => break n,
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    Err(e) => return Poll::Ready(Err(e)),
                }
            };
            {
                let (_, rest) = mem::take(&mut *me.buf).split_at(n);
                *me.buf = rest;
            }
            if n == 0 {
                return Poll::Ready(Err(io::ErrorKind::WriteZero.into()));
            }
        }

        Poll::Ready(Ok(()))
    }
}
