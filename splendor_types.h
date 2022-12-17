/* Type defined */

#include <stdlib.h>
// 数字忘了多少张了
#define L1NUM    40
#define L2NUM	30
#define L3NUM 	20 
#define TOTNUM  (L1NUM+L2NUM+L3NUM)
// I后缀的都是代表索引值的
// _CI代表XX的Cost的Index
#define BLACK_CI	0
#define BLUE_CI		1
#define GREEN_CI	2
#define RED_CI		3
#define WHITE_CI	4 // card cost of the color index
#define GOLD_I		5
#define CARD_CLRI   5 // card color index
#define NUMCOLORS   6
#define SCORE_I 	6
// Card的信息字段不止有开销，还有颜色、等级和分数，所以各加一
// 于是fast_cards_pool[CardId][:]就为
// pool[cid][BLACK_CI] <=> [cid][0] 
// pool[cid][BLUE_CI]  <=> [cid][1]
// pool[cid][GREEN_CI] <=> [cid][2]
// pool[cid][RED_CI]   <=> [cid][3]
// pool[cid][WHITE_CI] <=> [cid][4]
// pool[cid][CARD_CLRI] <=> [cid][5]
// pool[cid][SCORE_I] <=>  [cid][6]
// 
#define NUMFILED (NUMCOLORS+1+1+1)

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



unsigned char fast_cards_pool[TOTNUM][NUMFILED];
unsigned char fast_noble_pool[NOBLESNUM][NUMCOLORS];

