/* Type defined */

#include <stdlib.h>
// 数字忘了多少张了
#define NUM3DECK    20
#define NUM2DECK	30
#define NUM1DECK 	40 
#define BLACK_I		0
#define WHITE_I		1
#define RED_I		2
#define GREEN_I		3
#define BLUE_I		4
#define GOLD_I		5
#define NUMCOLORS   6

// ensure the security
#define PICKED(A,CLR) 	(A[CLR]-=1) 
#define PICKED_DOUBLE(A,CLR)	(A[CLR]-=2)
#define GET(PL,CLR) 	(PL->amount+=1) 


typedef unsigned char uscore;
typedef unsigned char u8;
typedef unsigned char ucost;
typedef unsigned long ulong;
typedef enum action_enum {  RESERVE, PICK, BUY, } action;
typedef enum colors { BLACK, WHITE, RED, GREEN, BLUE, GOLD} color;
// struct cost <=> array ucost costs[5] 
struct cost {
	ucost black;
	ucost white;
	ucost red; 
	ucost green;
	ucost blue;
};


// details of a card 
typedef struct card {
	uscore scores;
	color clr;
	// struct ucost *cost;	
	ucost costs[5];
} card;

// info of a noble
struct noble {
	ucost prefer[5];
	uint id;
};

struct player {
	uscore current_score;
	u8 amount[NUMCOLORS];
	card built_cards_list[];
}