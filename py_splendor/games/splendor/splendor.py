# splendor games

import numpy as np
import itertools as it
import copy 
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
		self.gameState = GameState(cards_pool.copy(), 1, Player(1), Player(-1))
		self.actionSpace = np.zeros(ACTIONS_NUM, dtype=np.int8)
		#NOTICE: remove the deprecated field!
		self.pieces = {'1': 'X', '0': '-', '-1':'O'}
		self.name = 'Splendor'
		# 一维的序列化的棋盘，这一点暗示我们似乎可以用一维卷积操作？
		self.grid_shape = (1, ACTIONS_NUM) 
		self.input_shape = (2, 1, ACTIONS_NUM) # two players. two channels
		self.state_size =  len(self.gameState.binary)
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
		# NOTICE taken_actions跟binary相对接
		# 预设为一个ACTIONS_NUM位的行动记录槽，
		# 需要注意的是，拿取宝石操作会重复，所以目前用binary不太妥，

	def affortable(self, card_code):
		delta = self.gems + self.bought - cards_pool[card_code][:5] 
		return False if delta.any() < 0 else True

	def show(self):
		print(self.id, self.gems, self.score, sep='~\n')

	def reach_target(self):
		return True if self.score >= TARGET else False

	def step_incrmnt(self,actcode):
		self.taken_actions[actcode] += 1

	def buy(self, card_code):
		# 已经检查过了
		rest =  cards_pool[card_code][:5] - self.bought
		rest[rest<0] = 0
		self.gems -= rest
		assert all(self.gems) > 0

class GameState:

	def __init__(self, 
		board, 
		playerTurn,
		p0: Player,
		p1: Player,
		# nobles: init = nobles_pool.copy()
		):
		# board就是cards_pool
		self.board = board
		self.gems = np.ones(5) * GEMS_EACH_MAX
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
		[ actcode for actcode in range(ACTIONS_NUM) if self._allowed(actcode)]

	def _not_bought(self, card_code):
		return False if self.board[card_code][LV_I] == 1 else True

	def _allowed(self, actcode):
		assert actcode in range(ACTIONS_NUM)
		if actcode < 90:
			# card
			if self._not_bought(actcode) and self.playerlist[self.playerTurn].affortable(actcode):
				return True
			else:
				return False
		elif actcode < 95:
			# pick gems with a single color
			color_num = actcode - 90
			return True if self.gems[color_num] >= 4 else False
		else:
			# pick three gems
			color_indices = list(it.combinations([0,1,2,3,4], 3))[actcode-95]
			"""
			[(0, 1, 2), (0, 1, 3), (0, 1, 4), (0, 2, 3), (0, 2, 4), (0, 3, 4), (1, 2, 3), (1, 2, 4), (1, 3, 4), (2, 3, 4)]
			"""

			if any(self.gems[list(color_indices)]) < 1:
				return False
			else:
				return True	

	def _convertStateToId(self):
		p0_log = np.append(self.playerlist[1].gems, self.playerlist[1].bought)
		p0_log = np.append(p0_log, self.playerlist[1].score)
		p1_log = np.append(self.playerlist[-1].gems, self.playerlist[1].bought)
		p1_log = np.append(p1_log, self.playerlist[-1].score)
		# log = np.append(self.board, p0_log)
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
			self.playerlist[self.playerTurn].buy(self, cards_pool[actcode])
			self.board[actcode][LV_I] = 0
		elif actcode < 95:
			# pick two
			color = actcode - 90	
			self.gems[color] -= 2
			self.playerlist[self.playerTurn].gems[color] += 2	
		else:
			color_indices = list(it.combinations([0,1,2,3,4], 3))
			self.gems[color_indices] -= 1
			self.playerlist[self.playerTurn].gems[color_indices] += 1
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
		logger.info('-----------------')


if __name__ == '__main__':
	g = Game()
	print(g.gameState.board)

