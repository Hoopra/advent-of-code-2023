#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <string.h>
#include <stdbool.h>

char findFirstDigit(char buffer[])
{
    for (size_t i = 0; i < strlen(buffer); i++)
    {
        char next = buffer[i];
        if (isdigit(next))
        {
            return next;
        }
    }

    return '\0';
}

char findLastDigit(char buffer[])
{
    for (size_t i = strlen(buffer) - 1; i >= 0; i--)
    {
        char next = buffer[i];
        if (isdigit(next))
        {
            return next;
        }
    }

    return '\0';
}

int digitsToNumber(char first, char last)
{
    char number[3];
    sprintf(number, "%c%c", first, last);

    int parsed = atoi(number);

    return parsed;
}

int main()
{
    FILE *filePointer;
    int bufferLength = 255;
    char buffer[bufferLength];

    filePointer = fopen("../input.txt", "r");

    int sum = 0;
    int lines = 0;

    while (fgets(buffer, bufferLength, filePointer))
    {
        char firstDigit = findFirstDigit(buffer);
        char lastDigit = findLastDigit(buffer);

        int parsed = digitsToNumber(firstDigit, lastDigit);

        sum += parsed;
        lines++;
    }

    printf("\ntotal lines: %d", lines);
    printf("\ngrand total: %d\n", sum); // 54390

    fclose(filePointer);
}
