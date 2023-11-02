#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

#define MAX_WIDTH 30
#define MAX_HEIGHT 30
#define BUF_SIZE 900

typedef struct {
    int x;
    int y;
} Point;

typedef struct {
    int head;
    int tail;
    int size;
    Point buf[BUF_SIZE];
} Queue;

Queue* QUEUE;

int WIDTH;
int HEIGHT;
int CAN_GO_X[MAX_WIDTH + 2][MAX_HEIGHT + 2];
int CAN_GO_Y[MAX_WIDTH + 2][MAX_HEIGHT + 2];
Point DIRECTIONS[4] = {
    {1, 0},
    {-1, 0},
    {0, 1},
    {0, -1}
};

void move_point(Point* from, Point* direction, Point* result) {
    result->x = from->x + direction->x;
    result->y = from->y + direction->y;
}

void reset_queue() {
    QUEUE->head = 0;
    QUEUE->size = 0;
    QUEUE->tail = 0;
    for (int i = 0; i < BUF_SIZE; i++) {
        QUEUE->buf[i].x = 0;
        QUEUE->buf[i].y = 0;
    }
}

void enqueue(Point* data) {
    if (QUEUE->size < BUF_SIZE) {
        int tail = QUEUE->tail;
        QUEUE->buf[tail].x = data->x;
        QUEUE->buf[tail].y = data->y;
        QUEUE->tail = (tail + 1) % BUF_SIZE;
        QUEUE->size++;
    } else {
        printf("Buffer is full, cannot save data.");
    }
}

Point* dequeue() {
    if (QUEUE->size > 0) {
        int head = QUEUE->head;
        Point* result = &QUEUE->buf[head];
        QUEUE->head = (head + 1) % BUF_SIZE;
        QUEUE->size--;
        return result;
    } else {
        printf("Buffer is null, cannot get data");
        return NULL;
    }
}

bool can_go(Point* from, Point* direction) {
    bool result = false;
    if (direction->x == 1) {
        if (CAN_GO_X[1 + from->y][1 + from->x] == 1) {
            CAN_GO_X[1 + from->y][1 + from->x] = 0;
            result = true;
        }
    } else if (direction->x == -1) {
        if (CAN_GO_X[1 + from->y][from->x] == 1) {
            CAN_GO_X[1 + from->y][from->x] = 0;
            result = true;
        }
    } else if (direction->y == 1) {
        if (CAN_GO_Y[1 + from->y][1 + from->x] == 1) {
            CAN_GO_Y[1 + from->y][1 + from->x] = 0;
            result = true;
        }
    } else {
        if (CAN_GO_Y[from->y][1 + from->x] == 1) {
            CAN_GO_Y[from->y][1 + from->x] = 0;
            result = true;
        }
    }
    return result;
}

void setup_board(FILE* file, unsigned long* bytes_reader) {
    fseek(file, *bytes_reader, SEEK_SET);

    char* line = NULL;
    size_t len = 0;
    ssize_t read;
    
    if ((read = getline(&line, &len, file)) != -1) {
        int w, h;
        sscanf(line, "%d %d", &w, &h);
        WIDTH = w;
        HEIGHT = h;
    }
    free(line);

    for (int i = 0; i <= HEIGHT + 1; i++) {
        for (int j = 0; j <= WIDTH + 1; j++) {
            CAN_GO_X[i][j] = 0;
            CAN_GO_Y[i][j] = 0;
        }
    }

    for (int i = 1; i <= HEIGHT; i++) {
        line = NULL;
        len = 0;
        if ((read = getline(&line, &len, file)) != -1) {
            int j = 1;
            char* token = strtok(line, " ");
            while (token != NULL) {
                CAN_GO_X[i][j] = 1 - atoi(token);
                token = strtok(NULL, " ");
                j++;
            }
        } else {
            fprintf(stderr, "Error reading file");
            exit(1);
        }
        free(line);
        line = NULL;

        if (i==HEIGHT) break;

        len = 0;
        if ((read = getline(&line, &len, file)) != -1) {
            int j = 1;
            char* token = strtok(line, " ");
            while (token != NULL) {
                CAN_GO_Y[i][j] = 1 - atoi(token);
                token = strtok(NULL, " ");
                j++;
            }
        } else {
            fprintf(stderr, "Error reading file");
            exit(1);
        }
        free(line);
        line = NULL;
    }

    *bytes_reader = ftell(file);
}

void print_board() {
    int height = HEIGHT;
    int width = WIDTH;

    printf("Printing CAN_GO_X:\n");
    for (int i = 1; i <= height; i++) {
        for (int j = 0; j <= width; j++) {
            printf("%d ", CAN_GO_X[i][j]);
        }
        printf("\n");
    }
    printf("-----------\n");
    printf("Printing CAN_GO_Y:\n");
    for (int i = 0; i <= height; i++) {
        for (int j = 1; j <= width; j++) {
            printf("%d ", CAN_GO_Y[i][j]);
        }
        printf("\n");
    }
}

int solve() {
    int shortest_path_length = 0;
    reset_queue();
    enqueue(&(Point){0, 0});

    while (1) {
        shortest_path_length += 1;
        int current_locations = QUEUE->size;

        if (current_locations == 0) {
            shortest_path_length = 0;
            break;
        }

        for (int i = 0; i < current_locations; i++) {
            Point* here = dequeue();
            if (here->x == WIDTH - 1 && here->y == HEIGHT - 1) {
                return shortest_path_length;
            }

            for (int j = 0; j < 4; j++) {
                if (can_go(here, &(Point){DIRECTIONS[j].x, DIRECTIONS[j].y})) {
                    Point next;
                    move_point(here, &(Point){DIRECTIONS[j].x, DIRECTIONS[j].y}, &next);
                    enqueue(&next);
                }
            }
        }
    }
    return shortest_path_length;
}

int main(int argc, char *argv[]) {
    if (argc < 3) {
        printf("Usage: program_name input_file_path answer_file_path");
        return 1;
    }

    FILE* input_file = fopen(argv[1], "r");
    FILE* answer_file = fopen(argv[2], "r");
    
    QUEUE = malloc(sizeof(Queue));

    // メモリが確保されていることを確認
    if (QUEUE == NULL) {
        printf("Memory allocation failed.");
        return 1;
    }

    // 初期化
    reset_queue();

    char* buf = NULL;
    size_t len = 0;
    ssize_t read;
    unsigned long bite_reader = 0;

    while ((read = getline(&buf, &len, answer_file)) != -1) {
        char* token = strtok(buf, " ");
        int ans = atoi(token);
        setup_board(input_file, &bite_reader);
        // print_board();

        int res = solve();
        if (res != ans) {
            fprintf(stderr, "このアルゴリズムは不完全です\n");
            return 1;
        }

        free(buf);
        buf = NULL;
    }

    free(QUEUE);
    fclose(input_file);
    fclose(answer_file);
    return 0;
}
