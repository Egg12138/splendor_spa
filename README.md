# README

## About this project
## What is Splendor?

Splednor is a classical boardgame designed by Marc and Pascal Qyudault. 

## What do we want to do?

* (EZ)Build an pretty printed CLI-Splendor for two player.
* (Mid)Train a Reinforcement-learning-based AI for two-player splendor
* (Future & Hard) Enable remote battle.

## 施工进度

### Part1 游戏CLI本体的实现
* -[] 数据结构设计 (考虑不用C了)
* -[] 交互设计
* -[] 基本的CLI界面
* -[] 完成“无黄金&无贵族”的Demo
* -[] 完成“有黄金&无贵族”的Demo
* -[] 完成“有黄金&有贵族”的Demo

#### 牌的数据结构

思路：
1. 首先定义`struct Card`, 其内部包含了价格、颜色、等级、分数的信息
接下来会分化两个思路:
总牌池结构是固定的: `card_pool: Vec<Card>`, 我们会序列化存储到数据文件中。
1. 每一张 `struct Card`都添加一个索引字段`cid`表示其独一无二的标志(类似HashMap)。我们将所有的卡按等级分三叠存在牌池向量里: cid就是索引值: 0-L1, L1-L2, L2-L3分别代表lvl1, lvl2, lvl3的卡牌, 使用的使用，
#### 洗牌算法
每一张牌我们都有一个card_uid



### Part2 CLI游戏和模型的API设计与实现

### Part2.5 训练

### Part3 训练报告制作&写文档

### Part4 实现CLI-Splendor中, AI接管对手位

### (Option) 将游戏绘制到Terminal UI


## Installation

<!-- TODO -->



## More...

The latest version of the board game is available now, called `Splendor: Dual`!

