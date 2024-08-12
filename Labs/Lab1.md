# LAB-1 TOCTOU resilient program and XSS

## [Press here to return to HOME](../index.md)

## TOCTOU resilient program

The following program shows the resilient version of the TOCTOU. Please use the same procedure as in lab sheet to check the output.
``` c
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/stat.h>
#include <fcntl.h>

void write_to_file(int fd, const char *message) {
    FILE *file = fdopen(fd, "w");
    if (file == NULL) {
        perror("Error converting file descriptor to FILE pointer");
        close(fd);  
        return;
    }

    fprintf(file, "%s\n", message);
    fclose(file);
}

int main(int argc, char *argv[]) {
    if (argc != 3) {
        fprintf(stderr, "Usage: %s <filename> <message>\n", argv[0]);
        return 1;
    }

    const char *filename = argv[1];
    const char *message = argv[2];

    int fd = open(filename, O_WRONLY);
    if (fd == -1) {
        perror("Error opening file");
        return 1;
    }

    struct stat file_stat;
    if (fstat(fd, &file_stat) == -1) {
        perror("Error in fstat");
        close(fd);
        return 1;
    }

    if (file_stat.st_uid == getuid() && (file_stat.st_mode & S_IWUSR)) {
        printf("User has permission to write to the file %s.\n", filename);
        sleep(40); 

        write_to_file(fd, message);
        printf("Written given message to the file %s\n", filename);
    } else {
        printf("The user does not have permission to write to the file %s\n", filename);
        close(fd);
    }
    return 0;
}
```
