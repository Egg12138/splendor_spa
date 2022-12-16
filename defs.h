// defs.h
// declare all functions

#include <stdbool.h>
typedef unsigned char uid;
struct card;
struct noble;
struct cost;
struct player;

// splendor_set.c
void	shuffle_deck(struct card *[]);
void	uncover_a_new_card(uint); // level  &  including handle????
bool	is_win();
bool 	is_valid(); //check if the operation is vlaid(包括拿取之后宝石数量的合法性)
bool 	has_got_noble(uid); 
bool 	pick(uid);
