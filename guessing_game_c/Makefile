
CFLAGS = -Werror -Wextra -Wall -g
# TESTFLAGS = -Wno-unused-but-set-variable -Wno-unused-variable -Wno-unused-parameter


compile: main.o
	gcc $(CFLAGS) -o app main.o

main.o: main.c
	gcc $(CFLAGS) -c -o main.o main.c


.PHONY: clean
clean:
	rm -f *.o app