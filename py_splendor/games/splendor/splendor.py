# splendor games

import numpy as np
import itertools as it
import copy 
from utils import actcode_to_action
from config import *

num_clr_map = {
	0: 'green',
	1: 'white',
	2: 'blue',
	3: 'black',
	4: 'red'
}

def num_to_color(num):
	assert num in [0,1,2,3,4]
	num_clr_map[num]	

	# TODO: NOTICE: 宝石追踪！完全棋盘表示！
class Game:
	# TODO: 下一次commit完成CNN的处理

	def __init__(self):
		"""
		ActionSpace: 对于玩家，它一次有最大105个可选行动：
						[0, 90): 买卡(1,2,3级分别40,30,20张)	
						[90, 95): 拿同色2宝石，5种选择
						[95, 105): C53拿3个不同色宝石，10种选择。

		# NOTICE ？这是真实的ActionState吗？
		# NOTICE ？状态空间多大？不同玩家可做的行动是不对称的
		"""
		self.currentPlayer = 1 # 等价于playerTurn
		#self.gameState = GameState(np.zeros(90, dtype=np.int8), 1, Player(1), Player(-1))
		self.gameState = GameState(cards_pool.copy(), 1, Player(1), Player(-1))
		self.actionSpace = np.zeros(ACTIONS_NUM, dtype=np.int8)
		#NOTICE: remove the deprecated field!
		self.pieces = {'1': 'X', '0': '-', '-1':'O'}
		self.name = 'Splendor'
		self.grid_shape = (1, ACTIONS_NUM) 
		self.input_shape = (2, 1, ACTIONS_NUM) # two players. two channels
		self.state_size = len(self.gameState.binary)
		self.action_size = ACTIONS_NUM

	def reset(self):
		self.gameState = GameState(cards_pool.copy(), 1, Player(1), Player(-1))
		self.currentPlayer = 1
		return self.gameState

	def step(self, actcode):
		next_state, value, done = self.gameState.takeAction(actcode)
		self.gameState = next_state
		self.currentPlayer = -self.currentPlayer
		return ((next_state, value, done, None))

	# 总是作为一个方法指针被调用
	def identities(self, state, actionValues):
		identities = [(state, actionValues)]
		return identities

class Player:
	"""并不是一个智能体，只是一个存储玩家数据的结构"""
	def __init__(self, id):
		self.id = id
		self.gems = np.zeros(5, dtype=np.int8)
		self.score = 0
		self.bought = np.zeros(5, dtype=np.int8)
		self.taken_actions = np.zeros(ACTIONS_NUM, dtype=np.int8)
		self.recent_actcode = None # 最近一次的行动代号 
		# NOTICE taken_actions跟binary相对接
		# 预设为一个ACTIONS_NUM位的行动记录槽，
		# 需要注意的是，拿取宝石操作会重复，所以目前用binary不太妥，

	def affortable(self, card_code):
		delta = self.gems + self.bought - cards_pool[card_code][:5] 
		if all(delta) > 0:
			return True
		else:
			return False

	def will_not_overhold(self, num):
		return False if self.gems.sum()+num > 10 else True
	def show(self):
		print(self.id, self.gems, self.score, sep='~\n')

	def reach_target(self):
		return True if self.score >= TARGET else False

	def step_incrmnt(self,actcode):
		self.taken_actions[actcode] += 1
		self.recent_actcode = actcode

	def buy(self, card_code):
		# 已经检查过了
		rest =  cards_pool[card_code][:5] - self.bought
		rest[rest<0] = 0
		self.gems -= rest
		assert all(self.gems) > 0

class GameState:
	# TODO 将 board 进行重构，使其更接近二值化棋盘——算的应该会快一点
	def __init__(self, 
		board, 
		playerTurn,
		p0: Player,
		p1: Player,
		# nobles: init = nobles_pool.copy()
		):
		# board就是cards_pool
		self.board = board
		self.gems = np.ones(COLORS_NUM, dtype=np.int8) * GEMS_EACH_MAX
		# self.nobles = nobles   
		self.playerTurn = playerTurn
		self.playerlist = [playerTurn, p0, p1]
		self.binary = self._binary()			# 兼容API的命名
		self.id = self._convertStateToId()
		self.allowedActions = self._allowedActions()
		self.isEndGame = self._checkForEndGame()
		self.value = self._getValue()
		self.score = self._getScore()


	def _binary(self):
		"""
		兼容API命名。总之,它是描述状态空间的：
		board的牌空间: (1, 90)二值化数组, 1为可获取, 0为已被买走
		公共区域gems空间: (1, 5)数组
		player1的可选牌: (1, 90)二值化数组，1为买得起，0为已被买走或买不起
		player1的宝石存量
		player2的可选牌: (1, 90)二值化数组，1为买得起，0为已被买走或买不起
		"""
		action_states_record = np.append(
								self.playerlist[1].taken_actions, 
								self.playerlist[-1].taken_actions
		)	
		return action_states_record

	def _allowedActions(self):
		res = [ actcode for actcode in range(90, ACTIONS_NUM) if self._allowed(actcode)]
		print(res)
		return res

	def TODO_not_been_bought(self, card_code):
		return False if self.board[card_code] == 0 else True

	def _not_been_bought(self, card_code):
		return False if self.board[card_code][LV_I] == 0 else True

	def _allowed(self, actcode):
		assert actcode in range(ACTIONS_NUM)
		if actcode < CARDS_NUM:
			# card
			if self._not_been_bought(actcode) and self.playerlist[self.playerTurn].affortable(actcode):
				return True
			else:
				return False
		elif actcode < (CARDS_NUM+COLORS_NUM):
			# pick gems with a single color
			color_num = actcode - CARDS_NUM
			return True if (self.gems[color_num] >= 4 and self.playerlist[self.playerTurn].will_not_overhold(2)) else False
		else:
			# pick three gems
			color_indices = list(it.combinations([0,1,2,3,4], 3))[actcode-(CARDS_NUM+COLORS_NUM)]
			"""
			[(0, 1, 2), (0, 1, 3), (0, 1, 4), (0, 2, 3), (0, 2, 4), (0, 3, 4), (1, 2, 3), (1, 2, 4), (1, 3, 4), (2, 3, 4)]
			"""

			if any(self.gems[list(color_indices)]) < 1 or not self.playerlist[self.playerTurn].will_not_overhold(3) :
				return False
			else:
				return True	

	def _convertStateToId(self):
		# TODO: id简短特质化
		p0_log = np.append(self.playerlist[1].gems, self.playerlist[1].bought)
		# +10 bytes
		p0_log = np.append(p0_log, self.playerlist[1].score)
		# +1 bytes
		p1_log = np.append(self.playerlist[-1].gems, self.playerlist[1].bought)
		# +10 bytes
		p1_log = np.append(p1_log, self.playerlist[-1].score)
		# +1 bytes	
		log = np.append(self._allowedActions, p0_log)
		id = ''.join(map(str, np.append(log, p1_log)))
		return id

	def TODO_checkForEndGame(self):
		# TODO: 搞完竞速版本后，再使用标准的行动轮判定
		if (self.playerlist[1].reach_target() or self.playerlist[-1].reach_target()) and self.playerTurn == -1:
			return 1
		else:
			return 0

	def _checkForEndGame(self):
		if (self.playerlist[1].reach_target() or self.playerlist[-1].reach_target()):
			return 1
		else:
			return 0

	def TODO_getValue(self):
		pass

	# 15分竞速版
	def _getValue(self):
		# 只考虑上家的现状是不是达到15了，如果到达了就直接gg
		if self.isEndGame:
			if self.playerlist[-self.playerTurn].reach_target():	return (-1, -1, 1)
		else:
			return (0, 0, 0)

	def _getScore(self):
		tmp = self.value		
		return (tmp[1], tmp[2])


	def getWinner(self):
		return self.value[0]

	def takeAction(self, actcode):

		# 从allowed_action选出,action已经合法
		if actcode in range(90):
			assert self.board[actcode][LV_I] != 0
			self.playerlist[self.playerTurn].buy(actcode)
			self.board[actcode][LV_I] = 0 # 标记为已被购买
		elif actcode < 95:
			# pick two
			color = actcode - 90	
			self.gems[color] -= 2
			self.playerlist[self.playerTurn].gems[color] += 2
		else:
			color_indices = list(it.combinations([0,1,2,3,4], 3))[actcode-95]
			self.gems[list(color_indices)] -= 1
			self.playerlist[self.playerTurn].gems[list(color_indices)] += 1

		self.playerlist[self.playerTurn].step_incrmnt(actcode)
		newState = copy.deepcopy(self)	
		newState.playerTurn = -self.playerTurn
		# 没有结束和没有胜利都是0的value
		value = 0
		done = 0
		if newState.isEndGame:
			value = newState.value[0]
			done = 1

		return (newState, value, done)

	def render(self, logger):
		action = self.playerlist[-self.playerTurn].recent_actcode
		if action == None:
			action = self.playerTurn
		else:
			action = actcode_to_action(action)
		logger.info(action)
		logger.info('-----------------')


if __name__ == '__main__':
	g = Game()
	print(g.gameState.board)
	print(g.gameState.allowedActions)

