pub async fn process(mut inbound: TcpStream) -> ProxyResult<()> {
    let request = webparse::Request::new();
    // 通过该方法解析标头是否合法, 若是partial(部分)则继续读数据
    // 若解析失败, 则表示非http协议能处理, 则抛出错误
    match request.parse_buffer(&mut buffer.clone()) {
    }
}