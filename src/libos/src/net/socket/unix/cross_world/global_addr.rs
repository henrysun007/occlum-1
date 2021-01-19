use super::*;

pub enum GlobalAddr {
    Host(SockAddr),
    Libos(Addr),
}

impl GlobalAddr {
    pub unsafe fn try_from_raw(
        sockaddr: *const libc::sockaddr,
        addr_len: libc::socklen_t,
    ) -> Result<Self> {
        let unix_addr = Addr::try_from_raw(sockaddr, addr_len)?;
        if unix_addr.is_from_host() {
            Ok(Self::Host(unix_addr.to_sockaddr()))
        } else {
            Ok(Self::Libos(unix_addr))
        }
    }
}
