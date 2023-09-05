#ifndef _SYS_SELECT_H
#define _SYS_SELECT_H

#include "../bits/sys/select.h"
#include "time.h"

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

int select(int nfds, fd_set *readfds, fd_set *writefds, fd_set *exceptfds, struct timeval *timeout);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* _SYS_SELECT_H */
