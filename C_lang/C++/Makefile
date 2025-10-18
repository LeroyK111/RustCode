CXX = g++
CXXFLAGS = -Wall -O2

SRC = src/main.cpp
OBJ = $(SRC:.cpp=.o)
TARGET = hello_cpp.exe

all: $(TARGET)

$(TARGET): $(OBJ)
	$(CXX) $(OBJ) -o $(TARGET)

%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

clean:
	del /f $(OBJ) $(TARGET)
