#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

#define ALLOC_SIZE 256

struct Map {
    int rows;
    int cols;
    char **tiles;
};

struct Map parse_input(char *filename) {
    struct Map map = {};
    map.tiles = (char**) calloc(ALLOC_SIZE, sizeof(char*));

    FILE *fptr = fopen(filename, "r");
    char ch;
    int rows = 0;
    int cols = 0;
    map.tiles[0] = (char*) calloc(ALLOC_SIZE, sizeof(char));
    while ((ch = fgetc(fptr)) != EOF) {
        if (ch == '\n') {
            rows++;
            if (map.cols == 0) {
                map.cols = cols;
            }
            map.tiles[rows] = (char*) calloc(ALLOC_SIZE, sizeof(char));
            cols = 0;
        } else {
            map.tiles[rows][cols] = ch;
            cols++;
        }
        map.rows = rows;
    }
    return map;
}

int part1(struct Map map) {
    enum Dir {
        UP, RIGHT, DOWN, LEFT
    };
    struct Guard {
        int row;
        int col;
        enum Dir dir;
    } guard;

    // Find carret
    for (int row = 0; row < map.rows; row++) {
        for (int col = 0; col < map.cols; col++) {
            if (map.tiles[row][col] == '^') {
                guard = (struct Guard) {row, col, UP};
            }
        }
    }

    int count = 0;
    // Do walking algo
    while(1) {
        struct Guard next_guard = guard;
        switch (guard.dir) {
            case UP: next_guard.row--; break;
            case RIGHT: next_guard.col++; break;
            case DOWN: next_guard.row++; break;
            case LEFT: next_guard.col--; break;
        }
        if (next_guard.col < 0 || next_guard.col >= map.cols || next_guard.row < 0 || next_guard.row >= map.rows) {
            map.tiles[guard.row][guard.col] = 'X';
            count++;
            break;
        }
        char next_tile = map.tiles[next_guard.row][next_guard.col];
        if (next_tile == '#') {
            guard.dir = (guard.dir + 1) % 4;
        } else {
            if (map.tiles[guard.row][guard.col] != 'X') {
                map.tiles[guard.row][guard.col] = 'X';
                count++;
            }
            guard = next_guard;
        }
    }

    return count;
}

int main() {
    struct Map map = parse_input("input.txt");
    printf("rows %d, cols %d\n", map.rows, map.cols);
    printf("Part 1: %d\n", part1(map));
}
