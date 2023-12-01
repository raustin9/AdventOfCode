CXX := clang++
CFLAGS := -std=c++17 -O2 -Wall -Wextra
BIN := bin
OBJ := obj
INCLUDES := -Iinclude/ -Isrc/include

CCFILES := $(shell find src/ -type f -name "*.cc")

all: $(BIN)/solution

clean:
	rm -f obj/* bin/*

run: $(BIN)/solution
	cat input.txt | ./$(BIN)/solution

$(OBJ)/%.o: $(CCFILES)
	$(CXX) $(INCLUDES) -c $< -o $@ $(CFLAGS)

$(BIN)/solution: obj/solution.o
	$(CXX) $(INCLUDES) $< -o $@ $(CFLAGS)
