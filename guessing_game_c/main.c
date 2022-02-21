#include <stdio.h>
#include <stdlib.h>
#include <time.h>

int main() {
    int guess;
    int secret_number;

    printf("Welcome to guessing game!\n");

    // Generate a random number
    srand(time(0));
    secret_number = rand() % 100;

    while(1) {
        printf("Enter your guess below\n");
        scanf("%d", &guess);  // Get number from user

        printf("Your guess was %d\n", guess);

        // Check if the guess was correct
        if(guess < secret_number) {
            printf("Higher\n");
        } else if(guess > secret_number) {
            printf("Lower\n");
        } else {
            printf("You win!\n");
            break;
        }
    }

    return 0;
}