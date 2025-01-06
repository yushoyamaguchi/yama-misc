#include <stdio.h>
#include <stdint.h>
#include <sys/ioctl.h>
#include <linux/vhost.h>

int main(void) {
    printf("VHOST_GET_BACKEND_FEATURES = 0x%lx\n", VHOST_GET_BACKEND_FEATURES);
    return 0;
}