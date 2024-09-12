/// 读取至少长度为size的大小的字节数, 如果足够则返回Ok(())
pub async fn read_len<T>(stream: &mut T, buffer: &mut BinaryMut, size: usize) -> ProxyResult<()>
where
    T: AsyncRead + Unpin {
        
    }