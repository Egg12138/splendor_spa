# config
# 配置命名复制自DeepReinforcementLearning
import numpy as np
#### SELF PLAY
EPISODES = 5
MCTS_SIMS = 5
MEMORY_SIZE = 30000
TURNS_UNTIL_TAU0 = 10 # turn on which it starts playing deterministically
CPUCT = 1
# LEARN 
EPSILON = 0.2
ALPHA = 0.8
INITIAL_REGISTER_VERSION = None

#### RETRAINING
BATCH_SIZE = 256
EPOCHS = 1
REG_CONST = 0.0001
LEARNING_RATE = 0.1
MOMENTUM = 0.9
TRAINING_LOOPS = 10
HIDDEN_CNN_LAYERS = [
	{'filters':75, 'kernel_size': (4,4)}
	 , {'filters':75, 'kernel_size': (4,4)}
	 , {'filters':75, 'kernel_size': (4,4)}
	 , {'filters':75, 'kernel_size': (4,4)}
	 , {'filters':75, 'kernel_size': (4,4)}
	 , {'filters':75, 'kernel_size': (4,4)}
	]

#### EVALUATION
EVAL_EPISODES = 20
SCORING_THRESHOLD = 1.3
TARGET: int = 2

GEMS_INIT: int = 0
GEMS_HOLD_MAX: int = 10
GEMS_EACH_MAX: int = 7 
NOBLES: int = 3
NOBLE_BONUS: int = 3
DECK1: int = 40
DECK2: int = 30
DECK3: int = 20
CARDS_NUM: int = 10
COLORS_NUM: int = 5
ACTIONS_NUM: int = CARDS_NUM + COLORS_NUM + 10  #标准5色就是+10, 就省的算一次C(n,3)组合数了
BINARY_SPACE: int = CARDS_NUM + COLORS_NUM * GEMS_EACH_MAX

# 一张卡的0-7各索引位
GREEN_I:int = 0
WHITE_I:int = 1
BLUE_I:int = 2
BLACK_I:int = 3 
RED_I:int = 4
LV_I:int = 5
SCORE_I:int = 6
COLOR_I:int = 7

# buy_regex::Regex =  r"[Bb]\w*\s+([1-5],[1-4])" 
# pick_regex::Regex = r"[Pp]\w*\s+(([1-5],?){1,3})"

BLACK_G:str = '🟤'
BLUE_G:str = '💎'
WHITE_G:str = '⚪'
RED_G:str = '🔴'
GREEN_G: str = '🟢'
GOLD_G: str = '🌕'
BLACK_C:str = '🟫'#U+1F7EB	
BLUE_C:str = '🟦'#U+1F7E6
WHITE_C:str = '🔲'#U+1F532
RED_C:str = '🟥'#U+1F7E5
GREEN_C:str = '🟩'#U+1F7E9
GOLD_C:str = '🟨'#事实上用不到U+1F7E8

CARD_COLORS = [GREEN_C, WHITE_C, BLUE_C, BLACK_C, RED_C]
GEMS_ICONS = [GREEN_G, WHITE_G, BLUE_G, BLACK_G, RED_G]
GREEN_BIN = 0b1 << 4
WHITE_BIN = 0b1 << 3
BLUE_BIN = 0b1 << 2
BLACK_BIN = 0b1 << 1
RED_BIN = 0b1 << 0

diff_color_map = {
	0: GREEN_G + WHITE_G + BLUE_G,
	1: GREEN_G + WHITE_G + BLACK_G,
	2: GREEN_G + WHITE_G + RED_G,
	3: GREEN_G + BLUE_G +BLACK_G,
	4: GREEN_G + BLUE_G +RED_G,
	5: GREEN_G + BLACK_G + RED_G,
	6: WHITE_G + BLUE_G +BLACK_G,
	7: WHITE_G + BLUE_G +RED_G,
	8: WHITE_G + BLACK_G + RED_G,
	9: BLUE_G+ BLACK_G +RED_G, 

}
# action code为90+的时候映射到颜色操作
diff_color_map_bit = {
	0: GREEN_BIN | WHITE_BIN | BLUE_BIN,
	1: GREEN_BIN | WHITE_BIN | BLACK_BIN,
	2: GREEN_BIN | WHITE_BIN | RED_BIN,
	3: GREEN_BIN | BLUE_BIN | BLACK_BIN,
	4: GREEN_BIN | BLUE_BIN | RED_BIN,
	5: GREEN_BIN | BLACK_BIN | RED_BIN,
	6: WHITE_BIN | BLUE_BIN | BLACK_BIN,
	7: WHITE_BIN | BLUE_BIN | RED_BIN,
	8: WHITE_BIN | BLACK_BIN | RED_BIN,
	9: BLUE_BIN | BLACK_BIN | RED_BIN, 
}


REG_SIZE = 100000
nobles_pool = [
[0,4,4,0,0],
[0,0,4,4,0],
[4,0,0,4,0],
[4,0,0,0,4],
[0,4,0,0,4],
[0,3,3,0,3],
[0,3,3,3,0],
[3,0,3,3,0],
[3,0,0,3,3],
[3,3,0,0,3],
]

cards_pool = np.array(
[
[ 0, 2, 1, 0, 0, 1, 0, 1],
[ 0, 0, 0, 1, 2, 1, 0, 2],
[ 0, 1, 0, 2, 0, 1, 0, 3],
[ 2, 0, 0, 0, 1, 1, 0, 4],
[ 1, 0, 2, 0, 0, 1, 0, 5],
[ 0, 0, 0, 0, 3, 1, 0, 1],
[ 0, 0, 3, 0, 0, 1, 0, 2],
[ 0, 0, 0, 3, 0, 1, 0, 3],
[ 3, 0, 0, 0, 0, 1, 0, 4],
[ 0, 3, 0, 0, 0, 1, 0, 5],
[ 0, 0, 0, 4, 0, 1, 1, 1],
[ 4, 0, 0, 0, 0, 1, 1, 2],
[ 0, 0, 0, 0, 4, 1, 1, 3],
[ 0, 0, 4, 0, 0, 1, 1, 4],
[ 0, 4, 0, 0, 0, 1, 1, 5],
[ 0, 1, 1, 1, 1, 1, 0, 1],
[ 1, 0, 1, 1, 1, 1, 0, 2],
[ 1, 1, 0, 1, 1, 1, 0, 3],
[ 1, 1, 1, 0, 1, 1, 0, 4],
[ 1, 1, 1, 1, 0, 1, 0, 5],
[ 0, 0, 2, 0, 2, 1, 0, 1],
[ 0, 0, 2, 2, 0, 1, 0, 2],
[ 2, 0, 0, 2, 0, 1, 0, 3],
[ 2, 2, 0, 0, 0, 1, 0, 4],
[ 0, 2, 0, 0, 2, 1, 0, 5],
[ 1, 1, 3, 0, 0, 1, 0, 1],
[ 0, 3, 1, 1, 0, 1, 0, 2],
[ 3, 0, 1, 0, 1, 1, 0, 3],
[ 1, 0, 0, 1, 3, 1, 0, 4],
[ 0, 1, 0, 3, 1, 1, 0, 5],
[ 0, 0, 1, 2, 2, 1, 0, 1],
[ 2, 0, 2, 1, 0, 1, 0, 2],
[ 2, 1, 0, 0, 2, 1, 0, 3],
[ 0, 2, 2, 0, 1, 1, 0, 4],
[ 1, 2, 0, 2, 0, 1, 0, 5],
[ 0, 1, 1, 2, 1, 1, 0, 1],
[ 2, 0, 1, 1, 1, 1, 0, 2],
[ 1, 1, 0, 1, 2, 1, 0, 3],
[ 1, 1, 2, 0, 1, 1, 0, 4],
[ 1, 2, 1, 1, 0, 1, 0, 5],
[ 0, 2, 3, 2, 0, 2, 1, 1],
[ 3, 0, 0, 2, 2, 2, 1, 2],
[ 2, 0, 2, 0, 3, 2, 1, 3],
[ 2, 3, 2, 0, 0, 2, 1, 4],
[ 0, 2, 0, 3, 2, 2, 1, 5],
[ 2, 3, 0, 0, 3, 2, 1, 1],
[ 0, 2, 3, 0, 3, 2, 1, 2],
[ 3, 0, 2, 3, 0, 2, 1, 3],
[ 3, 3, 0, 2, 0, 2, 1, 4],
[ 0, 0, 3, 3, 2, 2, 1, 5],
[ 5, 0, 0, 0, 0, 2, 2, 1],
[ 0, 0, 0, 0, 5, 2, 2, 2],
[ 0, 0, 5, 0, 0, 2, 2, 3],
[ 0, 5, 0, 0, 0, 2, 2, 4],
[ 0, 0, 0, 5, 0, 2, 2, 5],
[ 0, 4, 2, 1, 0, 2, 2, 1],
[ 0, 0, 0, 2, 4, 2, 2, 2],
[ 1, 2, 0, 4, 1, 2, 2, 3],
[ 4, 0, 1, 0, 2, 2, 2, 4],
[ 2, 1, 4, 0, 0, 2, 2, 5],
[ 3, 0, 5, 0, 0, 2, 2, 1],
[ 0, 0, 0, 3, 5, 2, 2, 2],
[ 0, 5, 3, 0, 0, 2, 2, 3],
[ 5, 0, 0, 0, 3, 2, 2, 4],
[ 0, 3, 0, 5, 0, 2, 2, 5],
[ 6, 0, 0, 0, 0, 2, 3, 1],
[ 0, 6, 0, 0, 0, 2, 3, 2],
[ 0, 0, 6, 0, 0, 2, 3, 3],
[ 0, 0, 0, 6, 0, 2, 3, 4],
[ 0, 0, 0, 0, 6, 2, 3, 5],
[ 0, 5, 3, 3, 3, 3, 3, 1],
[ 3, 0, 3, 3, 5, 3, 3, 2],
[ 3, 3, 0, 5, 3, 3, 3, 3],
[ 5, 3, 3, 0, 3, 3, 3, 4],
[ 3, 3, 5, 3, 0, 3, 3, 5],
[ 0, 0, 7, 0, 0, 3, 4, 1],
[ 0, 0, 0, 7, 0, 3, 4, 2],
[ 0, 7, 0, 0, 0, 3, 4, 3],
[ 0, 0, 0, 0, 7, 3, 4, 4],
[ 7, 0, 0, 0, 0, 3, 4, 5],
[ 3, 3, 6, 0, 0, 3, 4, 1],
[ 0, 3, 0, 6, 3, 3, 4, 2],
[ 0, 6, 3, 3, 0, 3, 4, 3],
[ 3, 0, 0, 3, 6, 3, 4, 4],
[ 6, 0, 3, 0, 3, 3, 4, 5],
[ 3, 0, 7, 0, 0, 3, 5, 1],
[ 0, 3, 0, 7, 0, 3, 5, 2],
[ 0, 7, 3, 0, 0, 3, 5, 3],
[ 0, 0, 0, 3, 7, 3, 5, 4],
[ 7, 0, 0, 0, 3, 3, 5, 5],
], dtype=np.int8)



"""
行动有27种
[0,11]: (1,1) (1,2) (1,3) (1,4) (2,1).. (3,4)
[12,21]: C_5^3
[22:26]: 00 11 22 33 44 
"""

