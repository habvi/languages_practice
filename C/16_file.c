#include <fcntl.h> // open
#include <unistd.h> // close, read, write
#include <errno.h> // error
#include <stdio.h> // perror

// mode : access level(chmod..)
// -> file descriptor
int open(const char *path, int flags);
int open(const char *path, int flags, int mode);

// -> 0:ok, -1:error
int close(int fd);

read();
// buf : head of buf pointer
// nbytes : unsigned int
// -> read bytes, -1:error
ssize_t read(int fd, void *buf, size_t nbytes);

// -> wrote bytes, -1:error
ssize_t write(int fd, const void *buf, size_t nbytes);

extern int errno;

// output stderr
void perror(const char *s);

// ------------------------------------------------
