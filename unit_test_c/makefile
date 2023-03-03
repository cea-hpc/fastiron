CC = gcc
CFLAGS = -lstdc++ -lm

SRC = main.cc MC_RNG_State.cc DirectionCosine.cc UpdateTrajectory.cc Computation.cc
OUT = main.o

.PHONY: clean

$(OUT): $(SRC)
	$(CC) -o $@ $(SRC) $(CFLAGS)

clean:
	rm *.o