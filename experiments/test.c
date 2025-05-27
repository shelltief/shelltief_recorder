#include <sys/statvfs.h>
#include <string.h>
#include <stdio.h>

int	main(int argc, const char **argv){
	char *path = "/Volumes/T7";
	struct statvfs buf;
	memset(&buf, 0, sizeof buf);
	int ret = statvfs(path, &buf);
	if (ret < 0) {
		return (1);
	}
	printf("f_bsize: %lu\n", buf.f_bsize);
	printf("f_frsize: %lu\n", buf.f_frsize);
	printf("available blocks: %u\n", buf.f_bavail);
}
