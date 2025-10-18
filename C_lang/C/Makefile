CC = gcc
CFLAGS = -Wall -O2
SRC = $(wildcard src/*.c)
OBJ = $(SRC:.c=.o)
TARGET = hello.exe

all: $(TARGET)

$(TARGET): $(OBJ)
	$(CC) $(OBJ) -o $(TARGET)

clean:
	rm -f $(OBJ) $(TARGET)


