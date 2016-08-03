#include <stdio.h>

void print_stuff(const unsigned char* const* array_of_strings)
{
    int i = 0;
    const char* next = array_of_strings[0];
    while (next != NULL)
    {
        printf("string: '%s'\n", next);
        next = array_of_strings[++i];
    }
    printf("done! %i strings.\n", i);
}