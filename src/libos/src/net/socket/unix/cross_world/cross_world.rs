use super::*;

pub struct CrossWorld {
    inner: SgxMutex<Status>,
}

#[derive(Clone, Debug)]
enum Status {
    Undetermined(UndeterminedSocket),
    Libos(Stream),
    Host(HostSocket),
}

#[derive(Clone, Debug)]
struct UndeterminedSocket {
    libos: Arc<Stream>,
    host: Arc<HostSocket>,
    libos_err: RwLock<Option<Errno>>,
    host_err: RwLock<Option<Errno>>,
    socket_type: SocketType,
}

impl CrossWorld {
    


}
