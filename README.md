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
* **Rust Version**(方便未来的联机对战而采用Rust编写)
* -[1] 数据结构设计 (考虑不用C了)
* -[1] 交互设计
* -[1] 基本的CLI界面
* -[1] 完成“无黄金&无贵族”的Demo
* -[1] 完成“有黄金&无贵族”的Demo
* -[0] 完成“有黄金&有贵族”的Demo
* **Julia Version**(快速版本)
* -[1] 完成“无黄金和预约”的Demo
* -[1] 完成命令解析（马上会逸出命令长度异常时手动引入的assert）
* -[0] 基本指令推荐（相似已写，还未套上去，希望有AI，引入AI操作提示后一起整合进去）

#### 牌的数据结构

思路：
1. 首先定义`struct Card`, 其内部包含了价格、颜色、等级、分数的信息,总牌池结构是固定的: `card_pool: Vec<Card>`, 我们会序列化存储到数据文件中。
1. 或者直接定义`Vec<u8>`来存放牌信息，`Vec<Vec<u8>>`存放牌堆信息，`Matrix<Vec<u8>>`存放牌池，

洗牌算法使用了语言标准库提供的`shuffle`


### Part2 CLI游戏和ML模型的API设计与实现
基于DRL，我们将规则简化为:无限宝石供应（其实影响不大，就是少了几句逻辑）+无预约和金币。
金币影响也不大，预约影响相对更大。




### Part2.5 训练

### Part3 训练报告制作&写文档

### Part4 实现CLI-Splendor中, AI接管对手位

### (Option) 将游戏绘制到Terminal UI


## Installation

<!-- TODO -->



## More...


